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
