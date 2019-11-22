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
            stream.write("\r\n".as_ref());
            Ok(stream)
        })
        .and_then(|mut stream| {
            let mut buf = String::new();
            stream.read_to_string(&mut buf);
            render(&buf);

            let mut y = 1;
            if let Ok((_col, row)) = termion::terminal_size() {
                y = row + 1;
            }
            for c in stdin.keys() {
                // Clear the current line.
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::CurrentLine
                )
                .unwrap();

                // Print the key we type...
                match c.unwrap() {
                    // Exit.
                    Key::Char('q') => break,
                    Key::Char(c) => print!("{}", c),
                    Key::Alt(c) => print!("Alt-{}", c),
                    Key::Ctrl('c') => break,
                    Key::Ctrl(c) => print!("Ctrl-{}", c),
                    Key::Left => print!("<left>"),
                    Key::Right => print!("<right>"),
                    Key::Up => print!("<up>"),
                    Key::Down => print!("<down>"),
                    _ => print!("Other"),
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

fn render(buf: &str) {
    println!("{}", draw(buf));
}

fn draw(buf: &str) -> String {
    let mut start = true;
    let mut skip_to_end = false;
    let mut links = 0;
    let mut out = String::with_capacity(buf.len() * 2);
    let mut prefix: &str;
    let mut is_link = false;
    for c in buf.chars() {
        if start {
            match c {
                'i' => {
                    prefix = "\x1B[93m";
                    is_link = false;
                }
                'h' => {
                    prefix = "\x1B[94m";
                    links += 1;
                    is_link = true;
                }
                '0' => {
                    prefix = "\x1B[95m";
                    links += 1;
                    is_link = true;
                }
                '1' => {
                    prefix = "\x1B[96m";
                    links += 1;
                    is_link = true;
                }
                _ => prefix = "",
            }
            out.push_str("  ");
            if is_link {
                out.push_str(&links.to_string());
                out.push_str(". ");
            } else {
                out.push(' ');
                out.push_str("\x1B[0m");
                out.push_str("  ");
            }
            out.push_str(prefix);
            start = false
        } else if skip_to_end {
            if c == '\n' {
                out.push_str("\r\n");
                start = true;
                skip_to_end = false;
            }
        } else if c == '\t' {
            skip_to_end = true;
        } else {
            out.push(c);
            if c == '\n' {
                start = true
            }
        }
    }
    out
}
