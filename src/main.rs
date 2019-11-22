#![allow(unused_must_use)]

extern crate termion;

use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    TcpStream::connect("phkt.io:70")
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
                        println!("");
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

            for c in stdin.keys() {
                // Clear the current line.
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::CurrentLine
                )
                .unwrap();

                // Print the key we type...
                match c.unwrap() {
                    // Exit.
                    Key::Char('q') => break,
                    Key::Char(c) => println!("{}", c),
                    Key::Alt(c) => println!("Alt-{}", c),
                    Key::Ctrl('c') => {
                        return Ok(());
                    }
                    Key::Ctrl(c) => println!("Ctrl-{}", c),
                    Key::Left => println!("<left>"),
                    Key::Right => println!("<right>"),
                    Key::Up => println!("<up>"),
                    Key::Down => println!("<down>"),
                    _ => println!("Other"),
                }

                // Flush again.
                stdout.flush().unwrap();
            }

            Ok(())
        })
        .map_err(|err| {
            eprintln!("err: {}", err);
        });
}
