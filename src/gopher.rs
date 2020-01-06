use native_tls::TlsConnector;
use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    net::ToSocketAddrs,
    os::unix::fs::OpenOptionsExt,
    time::Duration,
};
use termion::input::TermRead;

mod r#type;
pub use self::r#type::Type;

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

/// Wrapper for TLS and regular TCP streams.
pub struct Stream {
    io: Box<dyn ReadWrite>,
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.io.read(buf)
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }
}

/// Some Gopher servers can be kind of slow, we may want to up this or
/// make it configurable eventually.
pub const TCP_TIMEOUT_IN_SECS: u64 = 8;
pub const TCP_TIMEOUT_DURATION: Duration = Duration::from_secs(TCP_TIMEOUT_IN_SECS);

/// Fetches a gopher URL and returns a raw Gopher response.
pub fn fetch_url(url: &str) -> Result<String> {
    let (_, host, port, sel) = parse_url(url);
    fetch(host, port, sel)
}

/// Fetches a gopher URL by its component parts and returns a raw
/// Gopher response.
pub fn fetch(host: &str, port: &str, selector: &str) -> Result<String> {
    let mut stream = request(host, port, selector)?;
    let mut body = String::new();
    stream.read_to_string(&mut body)?;
    Ok(body)
}

/// Downloads a binary to disk. Allows canceling with Ctrl-c.
/// Returns the path it was saved to and the size in bytes.
pub fn download_url(url: &str) -> Result<(String, usize)> {
    let (_, host, port, sel) = parse_url(url);
    let filename = sel
        .split_terminator('/')
        .rev()
        .nth(0)
        .ok_or_else(|| error!("Bad download filename: {}", sel))?;
    let mut path = std::path::PathBuf::from(".");
    path.push(filename);
    let stdin = termion::async_stdin();
    let mut keys = stdin.keys();

    let mut stream = request(host, port, sel)?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o770)
        .open(path)?;

    let mut buf = [0; 1024];
    let mut bytes = 0;
    while let Ok(count) = stream.read(&mut buf) {
        if count == 0 {
            break;
        }
        bytes += count;
        file.write(&buf[..count]);
        if let Some(Ok(termion::event::Key::Ctrl('c'))) = keys.next() {
            return Err(error!("Download canceled"));
        }
    }
    Ok((filename.to_string(), bytes))
}

/// Make a Gopher request and return a TcpStream ready to be read()'d.
/// Will attempt a TLS connection first, then retry a regular
/// connection if it fails.
pub fn request(host: &str, port: &str, selector: &str) -> Result<Stream> {
    let selector = selector.replace('?', "\t"); // search queries
    let sock = format!("{}:{}", host, port)
        .to_socket_addrs()
        .and_then(|mut socks| socks.next().ok_or_else(|| error!("Can't create socket")))?;

    // attempt tls connection
    if let Ok(connector) = TlsConnector::new() {
        let stream = TcpStream::connect_timeout(&sock, TCP_TIMEOUT_DURATION)?;
        stream.set_read_timeout(Some(TCP_TIMEOUT_DURATION))?;
        if let Ok(mut stream) = connector.connect(host, stream) {
            stream.write(format!("{}\r\n", selector).as_ref())?;
            return Ok(Stream {
                io: Box::new(stream),
            });
        }
    }

    let mut stream = TcpStream::connect_timeout(&sock, TCP_TIMEOUT_DURATION)?;
    stream.write(format!("{}\r\n", selector).as_ref())?;
    Ok(Stream {
        io: Box::new(stream),
    })
}

/// Parses gopher URL into parts.
/// Returns (Type, host, port, sel)
pub fn parse_url(url: &str) -> (Type, &str, &str, &str) {
    let url = url.trim_start_matches("gopher://");

    // simple URLs, ex: "dog.com"
    if !url.contains(':') && !url.contains('/') {
        return (Type::Menu, url, "70", "");
    }

    // non-gopher URLs, stick everything in selector
    if url.contains("://") {
        return (Type::HTML, "", "", url);
    }

    let mut typ = Type::Menu;
    let mut host;
    let mut port = "70";
    let mut sel = "";

    // check selector first
    if let Some(idx) = url.find('/') {
        host = &url[..idx];
        sel = &url[idx..];
    } else {
        host = &url;
    }

    // ipv6
    if let Some(idx) = host.find('[') {
        if let Some(end) = host[idx + 1..].find(']') {
            host = &host[idx + 1..=end];
            if host.len() > end {
                if let Some(idx) = host[end..].find(':') {
                    port = &host[idx + 1..];
                }
            }
        } else {
            return (Type::Error, "Unclosed ipv6 bracket", "", url);
        }
    } else if let Some(idx) = host.find(':') {
        // two :'s == probably ipv6
        if host.len() > idx + 1 && !host[idx + 1..].contains(':') {
            // regular hostname w/ port -- grab port
            port = &host[idx + 1..];
            host = &host[..idx];
        }
    }

    // ignore type prefix on selector
    let mut chars = sel.chars();
    if let (Some('/'), Some(c), Some('/')) = (chars.nth(0), chars.nth(0), chars.nth(0)) {
        if let Some(t) = Type::from(c) {
            typ = t;
            sel = &sel[2..];
        }
    }

    (typ, host, port, sel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let urls = vec![
            "gopher://gopher.club/1/phlogs/",
            "gopher://sdf.org:7777/1/maps",
            "gopher.floodgap.org",
            "gopher.floodgap.com/0/gopher/relevance.txt",
            "gopher://gopherpedia.com/7/lookup?Gopher",
            "gopher://dead:beef:1234:5678:9012:3456:feed:deed",
            "gopher://[1234:2345:dead:4567:7890:1234:beef:1111]:7443/1/files",
            "gopher://2001:cdba:0000:0000:0000:0000:3257:9121",
            "[2001:cdba::3257:9652]",
            "gopher://9999:aaaa::abab:baba:aaaa:9999",
            "[2001:2099:dead:beef:0000",
            "::1",
            "ssh://kiosk@bitreich.org",
            "https://github.com/dvkt/phetch",
        ];

        let (typ, host, port, sel) = parse_url(urls[0]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "gopher.club");
        assert_eq!(port, "70");
        assert_eq!(sel, "/phlogs/");

        let (typ, host, port, sel) = parse_url(urls[1]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "sdf.org");
        assert_eq!(port, "7777");
        assert_eq!(sel, "/maps");

        let (typ, host, port, sel) = parse_url(urls[2]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "gopher.floodgap.org");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[3]);
        assert_eq!(typ, Type::Text);
        assert_eq!(host, "gopher.floodgap.com");
        assert_eq!(port, "70");
        assert_eq!(sel, "/gopher/relevance.txt");

        let (typ, host, port, sel) = parse_url(urls[4]);
        assert_eq!(typ, Type::Search);
        assert_eq!(host, "gopherpedia.com");
        assert_eq!(port, "70");
        assert_eq!(sel, "/lookup?Gopher");

        let (typ, host, port, sel) = parse_url(urls[5]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "dead:beef:1234:5678:9012:3456:feed:deed");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[6]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "1234:2345:dead:4567:7890:1234:beef:1111");
        assert_eq!(port, "70");
        assert_eq!(sel, "/files");

        let (typ, host, port, sel) = parse_url(urls[7]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "2001:cdba:0000:0000:0000:0000:3257:9121");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[8]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "2001:cdba::3257:9652");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[9]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "9999:aaaa::abab:baba:aaaa:9999");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[10]);
        assert_eq!(typ, Type::Error);
        assert_eq!(host, "Unclosed ipv6 bracket");
        assert_eq!(port, "");
        assert_eq!(sel, "[2001:2099:dead:beef:0000");

        let (typ, host, port, sel) = parse_url(urls[11]);
        assert_eq!(typ, Type::Menu);
        assert_eq!(host, "::1");
        assert_eq!(port, "70");
        assert_eq!(sel, "");

        let (typ, host, port, sel) = parse_url(urls[12]);
        assert_eq!(typ, Type::HTML);
        assert_eq!(host, "");
        assert_eq!(port, "");
        assert_eq!(sel, "ssh://kiosk@bitreich.org");

        let (typ, host, port, sel) = parse_url(urls[13]);
        assert_eq!(typ, Type::HTML);
        assert_eq!(host, "");
        assert_eq!(port, "");
        assert_eq!(sel, "https://github.com/dvkt/phetch");
    }
}
