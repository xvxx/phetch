use gopher;
use std::io::{BufWriter, Read, Result, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::os::unix::fs::OpenOptionsExt;
use std::time::Duration;

pub const TCP_TIMEOUT_IN_SECS: u64 = 8;
pub const TCP_TIMEOUT_DURATION: Duration = Duration::from_secs(TCP_TIMEOUT_IN_SECS);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text,       // 0 | 96 | cyan
    Menu,       // 1 | 94 | blue
    CSOEntity,  // 2
    Error,      // 3 | 91 | red
    Binhex,     // 4 |  4 | white underline
    DOSFile,    // 5 |  4 | white underline
    UUEncoded,  // 6 |  4 | white underline
    Search,     // 7 |  0 | white
    Telnet,     // 8
    Binary,     // 9 |  4 | white underline
    Mirror,     // +
    GIF,        // g |  4 | white underline
    Telnet3270, // T
    HTML,       // h | 92 | green
    Image,      // I |  4 | white underline
    PNG,        // p |  4 | white underline
    Info,       // i | 93 | yellow
    Sound,      // s |  4 | white underline
    Document,   // d |  4 | white underline
}

impl Type {
    pub fn is_download(self) -> bool {
        match self {
            Type::Binhex
            | Type::DOSFile
            | Type::UUEncoded
            | Type::Binary
            | Type::GIF
            | Type::Image
            | Type::PNG
            | Type::Sound
            | Type::Document => true,
            _ => false,
        }
    }
}

pub fn type_for_char(c: char) -> Option<Type> {
    match c {
        '0' => Some(Type::Text),
        '1' => Some(Type::Menu),
        '2' => Some(Type::CSOEntity),
        '3' => Some(Type::Error),
        '4' => Some(Type::Binhex),
        '5' => Some(Type::DOSFile),
        '6' => Some(Type::UUEncoded),
        '7' => Some(Type::Search),
        '8' => Some(Type::Telnet),
        '9' => Some(Type::Binary),
        '+' => Some(Type::Mirror),
        'g' => Some(Type::GIF),
        'T' => Some(Type::Telnet3270),
        'h' => Some(Type::HTML),
        'I' => Some(Type::Image),
        'p' => Some(Type::PNG),
        'i' => Some(Type::Info),
        's' => Some(Type::Sound),
        'd' => Some(Type::Document),
        _ => None,
    }
}

macro_rules! error {
    ($e:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $e)
    };
    ($e:expr, $($y:expr),*) => {
        error!(format!($e, $($y),*));
    };
}

// Fetches a gopher URL and returns a raw Gopher response.
pub fn fetch_url(url: &str) -> Result<String> {
    let (_, host, port, sel) = parse_url(url);
    fetch(host, port, sel)
}

// Fetches a gopher URL by its component parts and returns a raw Gopher response.
pub fn fetch(host: &str, port: &str, selector: &str) -> Result<String> {
    let mut body = String::new();
    let selector = selector.replace('?', "\t"); // search queries

    format!("{}:{}", host, port)
        .to_socket_addrs()
        .and_then(|mut socks| socks.next().ok_or_else(|| error!("Can't create socket")))
        .and_then(|sock| TcpStream::connect_timeout(&sock, TCP_TIMEOUT_DURATION))
        .and_then(|mut stream| {
            stream.write(format!("{}\r\n", selector).as_ref());
            Ok(stream)
        })
        .and_then(|mut stream| {
            stream.set_read_timeout(Some(TCP_TIMEOUT_DURATION));
            stream.read_to_string(&mut body)?;
            Ok(body)
        })
}

// Downloads a binary to disk.
// Returns the path it was saved to and the size in bytes.
pub fn download_url(url: &str) -> Result<(String, usize)> {
    let (_, host, port, sel) = parse_url(url);
    let sel = sel.replace('?', "\t"); // search queries
    let filename = sel
        .split_terminator('/')
        .rev()
        .nth(0)
        .ok_or_else(|| error!("Bad download filename: {}", sel))?;
    let mut path = std::path::PathBuf::from(".");
    path.push(filename);

    format!("{}:{}", host, port)
        .to_socket_addrs()
        .and_then(|mut socks| socks.next().ok_or_else(|| error!("Can't create socket")))
        .and_then(|sock| TcpStream::connect_timeout(&sock, TCP_TIMEOUT_DURATION))
        .and_then(|mut stream| {
            stream.write(format!("{}\r\n", sel).as_ref());
            Ok(stream)
        })
        .and_then(|mut stream| {
            stream.set_read_timeout(Some(TCP_TIMEOUT_DURATION))?;

            let file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o770)
                .open(path)?;

            let mut file_buffer = BufWriter::new(file);
            let mut buf = [0 as u8; 8]; // read 8 bytes at a time
            let mut bytes = 0;
            while let Ok(count) = stream.read(&mut buf) {
                if count == 0 {
                    break;
                }
                bytes += count;
                file_buffer.write_all(&buf);
            }
            Ok((filename.to_string(), bytes))
        })
}

// url parsing states
enum Parsing {
    Host,
    Port,
    Selector,
}

// Parses gopher URL into parts.
pub fn parse_url(url: &str) -> (Type, &str, &str, &str) {
    let url = url.trim_start_matches("gopher://");

    let mut host = "";
    let mut port = "70";
    let mut sel = "/";
    let mut typ = Type::Menu;
    let mut state = Parsing::Host;
    let mut start = 0;

    for (i, c) in url.char_indices() {
        match state {
            Parsing::Host => {
                state = match c {
                    ':' => Parsing::Port,
                    '/' => Parsing::Selector,
                    _ => continue,
                };
                host = &url[start..i];
                start = if c == '/' { i } else { i + 1 };
            }
            Parsing::Port => {
                if c == '/' {
                    state = Parsing::Selector;
                    port = &url[start..i];
                    start = i;
                }
            }
            Parsing::Selector => {}
        }
    }

    match state {
        Parsing::Selector => sel = &url[start..],
        Parsing::Port => port = &url[start..],
        Parsing::Host => host = &url[start..],
    };

    let mut chars = sel.chars();
    if let (Some('/'), Some(c), Some('/')) = (chars.nth(0), chars.nth(0), chars.nth(0)) {
        if let Some(t) = gopher::type_for_char(c) {
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
        assert_eq!(sel, "/");

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
    }
}
