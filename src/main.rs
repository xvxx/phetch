#![allow(unused_must_use)]

extern crate termion;

use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct App {
    pages: HashMap<String, Page>,
    history: Vec<String>,
    pos: usize,
}

#[derive(Debug)]
struct Page {
    body: String,
    url: String,
    link: usize,
    links: Vec<Link>,
}

#[derive(Debug)]
struct Link {
    name: String,
    host: String,
    port: String,
    selector: String,
}

#[derive(Debug)]
enum Action {
    None,
    Up,
    Down,
    Back,
    Forward,
    Open,
    Link(usize),
    Select(usize),
    Quit,
}

fn main() {
    let mut app = App::new();
    app.load("phkt.io", "70", "/");
    loop {
        app.render();
        app.respond();
    }
}

impl App {
    fn new() -> App {
        App {
            pages: HashMap::new(),
            pos: 0,
            history: Vec::new(),
        }
    }

    fn back(&mut self) {
        if self.history.len() > 1 && self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn forward(&mut self) {
        if self.pos < self.history.len() - 1 {
            self.pos += 1;
        }
    }

    fn load(&mut self, host: &str, port: &str, selector: &str) {
        let mut page = self.fetch(host, port, selector);
        page.parse_links();
        if self.history.len() > 0 {
            self.pos += 1;
            self.history.insert(self.pos, page.url.to_string());
        } else {
            self.history.push(page.url.to_string());
            self.pos = 0;
        }
        self.pages.insert(page.url.to_string(), page);
    }

    fn render(&self) {
        if let Some(url) = self.history.get(self.pos) {
            if let Some(page) = self.pages.get(url) {
                // print!("\x1B[2J\x1B[H{}", page.draw()); // clear
                print!("{}", page.draw()); // clear
            }
        }
    }

    fn respond(&mut self) {
        let mut addr = (String::new(), String::new(), String::new());
        let url = self.history.get(self.pos).unwrap();
        let page = self.pages.get_mut(url);
        match page {
            None => return,
            Some(page) => match read_input(&page.links) {
                Action::Up => page.cursor_up(),
                Action::Down => page.cursor_down(),
                Action::Back => self.back(),
                Action::Forward => self.forward(),
                Action::Select(n) => page.link = n,
                Action::Link(n) => {
                    if n < page.links.len() {
                        let link = &page.links[n];
                        addr.0 = link.host.to_string();
                        addr.1 = link.port.to_string();
                        addr.2 = link.selector.to_string();
                    }
                }
                Action::Open => {
                    if page.link > 0 && page.link - 1 < page.links.len() {
                        let link = &page.links[page.link - 1];
                        addr.0 = link.host.to_string();
                        addr.1 = link.port.to_string();
                        addr.2 = link.selector.to_string();
                    }
                }
                Action::Quit => std::process::exit(0),
                _ => {}
            },
        }
        if !addr.0.is_empty() {
            self.load(&addr.0, &addr.1, &addr.2);
        }
    }

    fn fetch(&self, host: &str, port: &str, selector: &str) -> Page {
        let mut body = String::new();
        TcpStream::connect(format!("{}:{}", host, port))
            .and_then(|mut stream| {
                stream.write(format!("{}\r\n", selector).as_ref());
                Ok(stream)
            })
            .and_then(|mut stream| {
                stream.read_to_string(&mut body);
                Ok(())
            })
            .map_err(|err| {
                eprintln!("err: {}", err);
            });
        Page {
            body: body,
            link: 0,
            url: format!("{}:{}{}", host, port, selector),
            links: Vec::new(),
        }
    }
}

impl Page {
    fn cursor_up(&mut self) {
        if self.link > 0 {
            self.link -= 1;
        }
    }
    fn cursor_down(&mut self) {
        if self.link < self.links.len() {
            self.link += 1;
        }
    }

    fn parse_links(&mut self) {
        if self.links.len() > 0 {
            self.links.clear();
        }
        let mut start = true;
        let mut is_link = false;
        let mut link = (0, 0);
        for (i, c) in self.body.chars().enumerate() {
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
                    for s in self.body[link.0..link.1].split('\t') {
                        line.push(s);
                    }
                    self.links.push(Link {
                        name: line[0].to_string(),
                        selector: line[1].to_string(),
                        host: line[2].to_string(),
                        port: line[3].trim_end_matches('\r').to_string(),
                    });
                    is_link = false;
                }
            }
        }
    }

    fn draw(&self) -> String {
        let mut start = true;
        let mut skip_to_end = false;
        let mut links = 0;
        let mut out = String::with_capacity(self.body.len() * 2);
        let mut prefix = "";
        for (i, c) in self.body.chars().enumerate() {
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
                        if self.body.len() > i + 2
                            && self.body[i..].chars().next().unwrap() == '\r'
                            && self.body[i + 1..].chars().next().unwrap() == '\n'
                        {
                            continue;
                        }
                    }
                    '\r' => continue,
                    '\n' => continue,
                    _ => prefix = "",
                }
                if is_link && self.link > 0 && self.link == links {
                    out.push_str("\x1b[92;1m*\x1b[0m");
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
                    out.push_str("\x1B[0m");
                    start = true;
                }
            }
        }
        out
    }
}

fn read_input(links: &Vec<Link>) -> Action {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut y = 1;
    let mut input = String::new();
    if let Ok((_col, row)) = termion::terminal_size() {
        y = row + 1;
    } else {
        panic!("can't determine terminal size.");
    }

    print!("{}", termion::cursor::Hide);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, y),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('c') | Key::Ctrl('q') => return Action::Quit,
            Key::Char('\n') => return Action::Open,
            Key::Up | Key::Ctrl('p') => return Action::Up,
            Key::Down | Key::Ctrl('n') => return Action::Down,
            Key::Left => return Action::Back,
            Key::Right => return Action::Forward,
            Key::Char(c) => {
                input.push(c);
                for (i, link) in links.iter().enumerate() {
                    if input == (i + 1).to_string() {
                        return Action::Link(i);
                    } 
                }
            }
            Key::Backspace => {
                if input.is_empty() {
                    return Action::Back;
                } else {
                    input.pop();
                }
            }
            Key::Delete => {
                input.pop();
            }
            _ => print!("Other"),
        }
        stdout.flush().unwrap();
    }
    Action::None
}
