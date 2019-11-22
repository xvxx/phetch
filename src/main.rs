#![allow(unused_must_use)]

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    TcpStream::connect("phkt.io:70")
        .and_then(|mut stream| {
            stream.write("\r\n".as_ref()).unwrap();
            let mut buf = String::new();
            stream.read_to_string(&mut buf);
            println!("{}", buf);
            Ok(())
        })
        .map_err(|err| {
            eprintln!("err: {}", err);
        });
}
