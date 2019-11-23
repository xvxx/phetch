#![allow(unused_must_use)]

extern crate termion;

use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let response = phetch("phkt.io", 70, "/links");
    render(&response);
    user_input();
}

fn user_input() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut y = 1;
    let mut input = String::new();
    if let Ok((_col, row)) = termion::terminal_size() {
        y = row + 1;
    }

    print!("{}\x1B[92;1m>> \x1B[0m", termion::cursor::Goto(1, y));
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, y),
            termion::clear::CurrentLine
        )
        .unwrap();
        print!("\x1B[92;1m>> \x1B[0m");

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                input.clear();
            }
            Key::Char(c) => input.push(c),
            Key::Alt(c) => print!("Alt-{}", c),
            Key::Ctrl(c) => print!("Ctrl-{}", c),
            Key::Left => print!("<left>"),
            Key::Right => print!("<right>"),
            Key::Up => print!("<up>"),
            Key::Down => print!("<down>"),
            Key::Backspace | Key::Delete => {
                input.pop();
            }
            _ => print!("Other"),
        }

        print!("{}", input);
        stdout.flush().unwrap();
    }
}

fn phetch(host: &str, port: i8, selector: &str) -> String {
    let mut out = String::new();
    TcpStream::connect(format!("{}:{}", host, port))
        .and_then(|mut stream| {
            stream.write(format!("{}\r\n", selector).as_ref());
            Ok(stream)
        })
        .and_then(|mut stream| {
            stream.read_to_string(&mut out);
            Ok(())
        })
        .map_err(|err| {
            eprintln!("err: {}", err);
        });
    out
}

fn render(buf: &str) {
    print!("\x1B[2J\x1B[H{}", draw(buf));
}

fn draw(buf: &str) -> String {
    let mut start = true;
    let mut skip_to_end = false;
    let mut links = 0;
    let mut out = String::with_capacity(buf.len() * 2);
    let mut prefix = "";
    for (i, c) in buf.chars().enumerate() {
        let mut is_link = false;
        if start {
            match c {
                'i' => {
                    prefix = "\x1B[93m";
                    is_link = false;
                }
                'h' => {
                    prefix = "\x1B[96m";
                    links += 1;
                    is_link = true;
                }
                '0' => {
                    prefix = "\x1B[94m";
                    links += 1;
                    is_link = true;
                }
                '1' => {
                    prefix = "\x1B[94m";
                    links += 1;
                    is_link = true;
                }
                '.' => {
                    if buf.len() > i + 2
                        && buf[i..].chars().next().unwrap() == '\r'
                        && buf[i + 1..].chars().next().unwrap() == '\n'
                    {
                        continue;
                    }
                }
                '\r' => continue,
                '\n' => continue,
                _ => prefix = "",
            }
            out.push_str("  ");
            if is_link {
                out.push_str("\x1B[95m");
                if links < 10 {
                    out.push(' ');
                }
                out.push_str(&links.to_string());
                out.push_str(". \x1B[0m");
            } else {
                out.push(' ');
                out.push_str("\x1B[0m");
                out.push_str("   ");
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
