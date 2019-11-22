#![allow(unused_must_use)]

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    TcpStream::connect("gopher.black:70")
        .and_then(|mut stream| {
            stream.write("\r\n".as_ref()).unwrap();
            let mut buf = String::new();
            stream.read_to_string(&mut buf);
            let mut start = true;
            let mut skip_to_end = false;
            for c in buf.chars() {
                if start {
                    match c {
                        'i' => print!("\x1B[93m"),
                        'h' => print!("\x1B[94m"),
                        '0' => print!("\x1B[95m"),
                        '1' => print!("\x1B[96m"),
                        _ => print!("\x1B[0m"),
                    }
                    start = false
                } else if skip_to_end {
                    if c == '\n' {
                        print!("{}", c);
                        start = true;
                        skip_to_end = false;
                    }
                } else if c == '\t' {
                    skip_to_end = true;
                } else {
                    print!("{}", c);
                    if c == '\n' {
                        start = true
                    }
                }
            }
            Ok(())
        })
        .map_err(|err| {
            eprintln!("err: {}", err);
        });
}
