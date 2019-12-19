use gopher;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text = '0' as isize,       // 0 | 96 | cyan
    Menu,                      // 1 | 94 | blue
    CSOEntity,                 // 2
    Error,                     // 3 | 91 | red
    Binhex,                    // 4 |  4 | white underline
    DOSFile,                   // 5 |  4 | white underline
    UUEncoded,                 // 6 |  4 | white underline
    Search,                    // 7 |  0 | white
    Telnet,                    // 8
    Binary,                    // 9 |  4 | white underline
    Mirror = '+' as isize,     // +
    GIF = 'g' as isize,        // g |  4 | white underline
    Telnet3270 = 'T' as isize, // T
    HTML = 'h' as isize,       // h | 92 | green
    Info = 'i' as isize,       // i | 93 | yellow
    Sound = 's' as isize,      // s |  4 | white underline
    Document = 'd' as isize,   // d |  4 | white underline
}

impl Type {
    pub fn is_download(self) -> bool {
        match self {
            Type::Binhex
            | Type::DOSFile
            | Type::UUEncoded
            | Type::Binary
            | Type::GIF
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
        'i' => Some(Type::Info),
        's' => Some(Type::Sound),
        'd' => Some(Type::Document),
        _ => None,
    }
}

// Fetches a URL and returns a raw Gopher response.
pub fn fetch_url(url: &str) -> io::Result<String> {
    let (_, host, port, sel) = parse_url(url);
    fetch(host, port, sel)
}

// Fetches a URL by its component parts and returns a raw Gopher response.
pub fn fetch(host: &str, port: &str, selector: &str) -> io::Result<String> {
    let mut body = String::new();
    let selector = selector.replace('?', "\t"); // search queries
    let stream = TcpStream::connect(format!("{}:{}", host, port))
        .and_then(|mut stream| {
            stream.write(format!("{}\r\n", selector).as_ref());
            Ok(stream)
        })
        .and_then(|mut stream| {
            stream.read_to_string(&mut body);
            Ok(())
        });

    match stream {
        Ok(_) => Ok(body),
        Err(e) => Err(e),
    }
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
                match c {
                    ':' => state = Parsing::Port,
                    '/' => state = Parsing::Selector,
                    _ => continue,
                }
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
