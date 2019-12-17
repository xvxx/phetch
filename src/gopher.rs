#![allow(dead_code)]

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text = '0' as isize,       // 0
    Menu,                      // 1
    CSOEntity,                 // 2
    Error,                     // 3
    Binhex,                    // 4
    DOSFile,                   // 5
    UUEncoded,                 // 6
    Search,                    // 7
    Telnet,                    // 8
    Binary,                    // 9
    Mirror = '+' as isize,     // +
    GIF = 'g' as isize,        // g
    Telnet3270 = 'T' as isize, // T
    HTML = 'h' as isize,       // h
    Info = 'i' as isize,       // i
    Sound = 's' as isize,      // s
    Document = 'd' as isize,   // d
}

// Fetches a URL and returns a raw Gopher response.
pub fn fetch(host: &str, port: &str, selector: &str) -> io::Result<String> {
    let mut body = String::new();
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

enum Parsing {
    Host,
    Port,
    Selector,
}

// Parses gopher URL into parts.
pub fn parse_url<'a>(url: &'a str) -> (Type, &'a str, &'a str, &'a str) {
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
                start = i + 1;
            }
            Parsing::Port => {
                if c == '/' {
                    state = Parsing::Selector;
                    port = &url[start..i];
                    start = i + 1;
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
    if let (Some(fst), Some('/')) = (chars.nth(0), chars.nth(1)) {
        match fst {
            '0' => typ = Type::Text,
            '1' => typ = Type::Menu,
            'h' => typ = Type::HTML,
            _ => {}
        }
        sel = &sel[2..];
    }

    (typ, host, port, sel)
}
