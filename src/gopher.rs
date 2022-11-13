//! phetch's Gopher library contains a few phetch-specific features:
//! the ability to make requests or downloads over TLS or Tor,
//! cleaning Unicode control characters from Gopher responses, and
//! URL parsing that recognizes different protocols like telnet and
//! IPv6 addresses.

use crate::ui::{self, Key};
use std::{
    fs,
    io::{Read, Result, Write},
    net::TcpStream,
    net::ToSocketAddrs,
    os::unix::fs::OpenOptionsExt,
    time::Duration,
};

#[cfg(feature = "tor")]
use tor_stream::TorStream;

#[cfg(feature = "tls")]
use native_tls::TlsConnector;

mod r#type;
pub use self::r#type::Type;

/// Some Gopher servers can be kind of slow, we may want to up this or
/// make it configurable eventually.
pub const TCP_TIMEOUT_IN_SECS: u64 = 8;
/// Based on `TCP_TIMEOUT_IN_SECS` but a `Duration` type.
pub const TCP_TIMEOUT_DURATION: Duration = Duration::from_secs(TCP_TIMEOUT_IN_SECS);

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

/// Wrapper for TLS and regular TCP streams.
pub struct Stream {
    io: Box<dyn ReadWrite>,
    tls: bool,
}

impl Stream {
    fn is_tls(&self) -> bool {
        self.tls
    }
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

/// Gopher URL. Returned by `parse_url()`.
pub struct Url<'a> {
    /// Gopher Type
    pub typ: Type,
    /// Hostname
    pub host: &'a str,
    /// Port. Defaults to 70
    pub port: &'a str,
    /// Selector
    pub sel: &'a str,
}

/// Fetches a gopher URL and returns a tuple of:
///   (did tls work?, raw Gopher response)
pub fn fetch_url(url: &str, tls: bool, tor: bool) -> Result<(bool, Vec<u8>)> {
    let u = parse_url(url);
    fetch(u.host, u.port, u.sel, tls, tor)
}

/// Fetches a gopher URL by its component parts and returns a tuple of:
///   (did tls work?, raw Gopher response)
pub fn fetch(
    host: &str,
    port: &str,
    selector: &str,
    tls: bool,
    tor: bool,
) -> Result<(bool, Vec<u8>)> {
    let mut stream = request(host, port, selector, tls, tor)?;
    let mut body = Vec::new();
    stream.read_to_end(&mut body)?;
    Ok((stream.is_tls(), body))
}

/// Turn a Gopher response from `fetch` into a UTF8 String, cleaning
/// up unprintable characters along the way.
pub fn response_to_string(res: &[u8]) -> String {
    let mut s = String::from_utf8_lossy(res).to_string();
    clean_response(&mut s);
    s
}

/// Removes unprintable characters from Gopher response.
/// https://en.wikipedia.org/wiki/Control_character#In_Unicode
fn clean_response(res: &mut String) {
    res.retain(|c| match c {
        '\u{007F}' => false,
        _ if ('\u{0080}'..='\u{009F}').contains(&c) => false,
        _ => true,
    })
}

/// Downloads a binary to disk. Allows canceling with Ctrl-c, but it's
/// kind of hacky - needs the UI receiver passed in.
/// Returns a tuple of:
///   (path it was saved to, the size in bytes)
pub fn download_url(
    url: &str,
    tls: bool,
    tor: bool,
    chan: ui::KeyReceiver,
) -> Result<(String, usize)> {
    let u = parse_url(url);
    let filename = u
        .sel
        .split_terminator('/')
        .rev()
        .next()
        .ok_or_else(|| error!("Bad download filename: {}", u.sel))?;
    let mut path = std::path::PathBuf::from(".");
    path.push(filename);

    let mut stream = request(u.host, u.port, u.sel, tls, tor)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o770)
        .open(&path)?;

    let mut buf = [0; 1024];
    let mut bytes = 0;
    while let Ok(count) = stream.read(&mut buf) {
        if count == 0 {
            break;
        }
        bytes += count;
        file.write_all(&buf[..count])?;
        if let Ok(chan) = chan.lock() {
            if let Ok(Key::Ctrl('c')) = chan.try_recv() {
                if path.exists() {
                    fs::remove_file(path)?;
                }
                return Err(error!("Download cancelled"));
            }
        }
    }

    Ok((filename.to_string(), bytes))
}

/// Make a Gopher request and return a TcpStream ready to be read()'d.
/// Will attempt a TLS connection first, then retry a regular
/// connection if it fails.
pub fn request(host: &str, port: &str, selector: &str, tls: bool, tor: bool) -> Result<Stream> {
    let selector = selector.replace('?', "\t"); // search queries
    let addr = format!("{}:{}", host, port);

    // attempt tls connection
    if tls {
        #[cfg(feature = "tls")]
        {
            {
                if let Ok(connector) = TlsConnector::new() {
                    let stream = addr
                        .to_socket_addrs()?
                        .find_map(|s| TcpStream::connect_timeout(&s, TCP_TIMEOUT_DURATION).ok())
                        .ok_or_else(|| error!("Can't create socket"))?;
                    stream.set_read_timeout(Some(TCP_TIMEOUT_DURATION))?;
                    if let Ok(mut stream) = connector.connect(host, stream) {
                        stream.write_all(selector.as_ref())?;
                        stream.write_all("\r\n".as_ref())?;
                        return Ok(Stream {
                            io: Box::new(stream),
                            tls: true,
                        });
                    }
                }
            }
        }
    }

    // tls didn't work or wasn't selected, try Tor or default
    if tor {
        #[cfg(feature = "tor")]
        {
            let mut stream = std::env::var("TOR_PROXY")
                .unwrap_or_else(|_| "127.0.0.1:9050".into())
                .to_socket_addrs()?
                .find_map(|s| TorStream::connect_with_address(s, addr.as_ref()).ok())
                .ok_or_else(|| error!("Can't create socket"))?;
            stream.write_all(selector.as_ref())?;
            stream.write_all("\r\n".as_ref())?;
            return Ok(Stream {
                io: Box::new(stream),
                tls: false,
            });
        }
    }

    // no tls or tor, try regular connection
    let mut stream = addr
        .to_socket_addrs()?
        .find_map(|s| TcpStream::connect_timeout(&s, TCP_TIMEOUT_DURATION).ok())
        .ok_or_else(|| error!("Can't create socket"))?;
    stream.set_read_timeout(Some(TCP_TIMEOUT_DURATION))?;
    stream.write_all(selector.as_ref())?;
    stream.write_all("\r\n".as_ref())?;
    Ok(Stream {
        io: Box::new(stream),
        tls: false,
    })
}

impl<'a> Url<'a> {
    /// Creates a new Gopher Url quickly from a tuple of Url fields.
    pub fn new(typ: Type, host: &'a str, port: &'a str, sel: &'a str) -> Url<'a> {
        Url {
            typ,
            host,
            port,
            sel,
        }
    }
}

/// Given a Gopher URL, returns a gopher::Type.
pub fn type_for_url(url: &str) -> Type {
    if url.starts_with("telnet://") {
        return Type::Telnet;
    }

    if url.starts_with("URL:") || url.starts_with("/URL:") {
        return Type::HTML;
    }

    let url = url.trim_start_matches("gopher://");
    if let Some(idx) = url.find('/') {
        if let Some(t) = url.chars().nth(idx + 1) {
            return Type::from(t).unwrap_or(Type::Menu);
        }
    }

    Type::Menu
}

/// Parses gopher URL into parts.
pub fn parse_url(url: &str) -> Url {
    let mut url = url.trim_start_matches("gopher://");
    let mut typ = Type::Menu;
    let mut host;
    let mut port = "70";
    let mut sel = "";

    // simple URLs, ex: "dog.com"
    if !url.contains(':') && !url.contains('/') {
        return Url::new(Type::Menu, url, "70", "");
    }

    // telnet urls
    if url.starts_with("telnet://") {
        typ = Type::Telnet;
        url = url.trim_start_matches("telnet://");
    } else if url.contains("://") {
        // non-gopher URLs, stick everything in selector
        return Url::new(Type::HTML, "", "", url);
    }

    // check selector first
    if let Some(idx) = url.find('/') {
        host = &url[..idx];
        sel = &url[idx..];
    } else {
        host = url;
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
            return Url::new(Type::Error, "Unclosed ipv6 bracket", "", url);
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
    if typ != Type::Telnet {
        let mut chars = sel.chars();
        if let (Some('/'), Some(c)) = (chars.next(), chars.next()) {
            if let Some(t) = Type::from(c) {
                typ = t;
                sel = &sel[2..];
            }
        }
    }

    Url::new(typ, host, port, sel)
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
            "https://github.com/xvxx/phetch",
            "telnet://bbs.impakt.net:6502/",
            "gopher://some.url/9/file.mp4",
            "gopher://some.url/;/file.mp4",
            "mtv.com/s/best-of-britney-spears.mp3",
            "gopher://microsoft.com:7070/x/developer/sitemap.xml",
            "gopher://mtv.com/c/kriss-kross/tour-dates.ical",
            "gopher://protonmail.com/M/mymail/inbox.eml",
        ];
        let mut urls = urls.iter();

        macro_rules! parse_next_url {
            () => {
                parse_url(urls.next().unwrap())
            };
        }

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "gopher.club");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/phlogs/");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "sdf.org");
        assert_eq!(url.port, "7777");
        assert_eq!(url.sel, "/maps");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "gopher.floodgap.org");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Text);
        assert_eq!(url.host, "gopher.floodgap.com");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/gopher/relevance.txt");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Search);
        assert_eq!(url.host, "gopherpedia.com");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/lookup?Gopher");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "dead:beef:1234:5678:9012:3456:feed:deed");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "1234:2345:dead:4567:7890:1234:beef:1111");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/files");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "2001:cdba:0000:0000:0000:0000:3257:9121");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "2001:cdba::3257:9652");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "9999:aaaa::abab:baba:aaaa:9999");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Error);
        assert_eq!(url.host, "Unclosed ipv6 bracket");
        assert_eq!(url.port, "");
        assert_eq!(url.sel, "[2001:2099:dead:beef:0000");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Menu);
        assert_eq!(url.host, "::1");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::HTML);
        assert_eq!(url.host, "");
        assert_eq!(url.port, "");
        assert_eq!(url.sel, "ssh://kiosk@bitreich.org");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::HTML);
        assert_eq!(url.host, "");
        assert_eq!(url.port, "");
        assert_eq!(url.sel, "https://github.com/xvxx/phetch");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Telnet);
        assert_eq!(url.host, "bbs.impakt.net");
        assert_eq!(url.port, "6502");
        assert_eq!(url.sel, "/");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Binary);
        assert_eq!(url.host, "some.url");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/file.mp4");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Video);
        assert_eq!(url.host, "some.url");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/file.mp4");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Sound);
        assert_eq!(url.host, "mtv.com");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/best-of-britney-spears.mp3");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Xml);
        assert_eq!(url.host, "microsoft.com");
        assert_eq!(url.port, "7070");
        assert_eq!(url.sel, "/developer/sitemap.xml");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Calendar);
        assert_eq!(url.host, "mtv.com");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/kriss-kross/tour-dates.ical");

        let url = parse_next_url!();
        assert_eq!(url.typ, Type::Mailbox);
        assert_eq!(url.host, "protonmail.com");
        assert_eq!(url.port, "70");
        assert_eq!(url.sel, "/mymail/inbox.eml");

        // make sure we got em all
        assert_eq!(urls.next(), None);
    }

    #[test]
    fn test_type_for_url() {
        assert_eq!(type_for_url("phkt.io"), Type::Menu);
        assert_eq!(type_for_url("phkt.io/1"), Type::Menu);
        assert_eq!(type_for_url("phkt.io/1/"), Type::Menu);
        assert_eq!(type_for_url("phkt.io/0/info.txt"), Type::Text);
        assert_eq!(
            type_for_url("gopher://vernunftzentrum.de/0/tfurrows/resources/tokipona.txt"),
            Type::Text
        );
        assert_eq!(type_for_url("URL:https://google.com"), Type::HTML);
        assert_eq!(
            type_for_url("telnet://bbs.inter.net:6502/connect"),
            Type::Telnet
        );
    }

    #[test]
    fn test_clean_response() {
        let mut test = "Hi".to_string();
        test.push('\u{007F}');
        test.push_str(" there!");
        test.push('\u{0082}');
        clean_response(&mut test);
        assert_eq!(test, "Hi there!".to_string());

        let mut test = "* \x1b[92mTitle\x1b[0m".to_string();
        clean_response(&mut test);
        assert_eq!(test, "* \x1b[92mTitle\x1b[0m".to_string());
    }
}
