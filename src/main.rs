#![allow(unused_must_use)]

extern crate termion;

use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct Link<'res> {
    name: &'res str,
    host: &'res str,
    port: &'res str,
    selector: &'res str,
}

struct Cursor {
    link: usize,
}

enum Action {
    None,
    Up,
    Down,
    Quit,
}

fn main() {
    let response = phetch("phkt.io", 70, "/links");
    let links = parse(&response);
    println!("{:?}", links);
    let mut cursor = Cursor { link: 0 };
    loop {
        render(&response, &cursor);
        match user_input() {
            Action::Up => {
                if cursor.link > 0 {
                    cursor.link -= 1;
                }
            }
            Action::Down => {
                if cursor.link < links.len() {
                    cursor.link += 1;
                }
            }
            Action::Quit => return,
            _ => {}
        }
    }
}

fn parse<'res>(response: &'res str) -> Vec<Link> {
    let mut links: Vec<Link> = Vec::new();
    let mut start = true;
    let mut is_link = false;
    let mut link = (0, 0);
    for (i, c) in response.chars().enumerate() {
        if start {
            match c {
                '0' | '1' => {
                    is_link = true;
                    link.0 = i + 1;
                }
                '\n' => continue,
                _ => is_link = false,
            }
            start = false;
        } else if c == '\n' {
            start = true;
            if is_link && i > link.0 {
                link.1 = i;
                let mut line = Vec::new();
                for s in response[link.0..link.1].split('\t') {
                    line.push(s);
                }
                links.push(Link {
                    name: line[0],
                    selector: line[1],
                    host: line[2],
                    port: line[3].trim_end_matches('\r'),
                });
                is_link = false;
            }
        }
    }
    links
}

fn user_input() -> Action {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut y = 1;
    let mut input = String::new();
    if let Ok((_col, row)) = termion::terminal_size() {
        y = row + 1;
    } else {
        panic!("can't determine terminal size.");
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
            Key::Ctrl('c') | Key::Char('q') => return Action::Quit,
            Key::Char('\n') => {
                input.clear();
            }
            Key::Char(c) => input.push(c),
            Key::Alt(c) => print!("Alt-{}", c),
            Key::Up | Key::Ctrl('p') => return Action::Up,
            Key::Down | Key::Ctrl('n') => return Action::Down,
            Key::Ctrl(c) => print!("Ctrl-{}", c),
            Key::Left => print!("<left>"),
            Key::Right => print!("<right>"),
            Key::Backspace | Key::Delete => {
                input.pop();
            }
            _ => print!("Other"),
        }

        print!("{}", input);
        stdout.flush().unwrap();
    }
    Action::None
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

fn render(buf: &str, cur: &Cursor) {
    // let clear = "";
    let clear = "\x1B[2J\x1B[H";
    print!("{}{}", clear, draw(buf, cur));
}

fn draw(buf: &str, cur: &Cursor) -> String {
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
            if is_link && cur.link > 0 && cur.link == links {
                out.push_str("\x1b[93;1m*\x1b[0m");
            } else {
                out.push(' ');
            }
            out.push_str(" ");
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
                out.push_str("\r\n\x1B[0m");
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
