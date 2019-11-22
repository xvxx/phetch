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
                println!("\r");
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
}
