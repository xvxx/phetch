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
    pages: HashMap<String, Page>, // url -> Page
    history: Vec<String>,         // ordered history of urls
    pos: usize,                   // position in history vec
}

#[derive(Debug)]
struct Page {
    body: String,     // response body
    url: String,      // url of this page
    link: usize,      // selected link
    links: Vec<Link>, // links on page
    input: String,    // what the user has typed
    ptype: PageType,  // type of page
    offset: u16,      // scrolling offset
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PageType {
    Dir,
    Text,
    HTML,
}

#[derive(Debug)]
struct Link {
    name: String,
    host: String,
    port: String,
    selector: String,
    ptype: PageType,
}

#[derive(Debug)]
enum Action {
    None,
    Up,
    Down,
    PageUp,
    PageDown,
    Back,
    Forward,
    Open,
    Link(usize),
    Select(usize),
    Fetch(String, String, String, PageType),
    Quit,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", args[0]);
    if args.len() < 2 {
        usage();
        return;
    }
    let host = args.get(1).unwrap();
    let port = String::from("70");
    let selector = String::from("/");
    let port = args.get(2).unwrap_or(&port);
    let selector = args.get(3).unwrap_or(&selector);
    if host == "--help" || host == "-h" || host == "-help" {
        usage();
        return;
    }

    let mut app = App::new();
    app.load(host, port, selector, PageType::Dir);
    loop {
        app.render();
        app.respond();
    }
}

fn usage() {
    println!("\x1B[93;1musage:\x1B[0m ");
    println!("\t$ phetch host [port [selector]]");
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

    fn load(&mut self, host: &str, port: &str, selector: &str, ptype: PageType) {
        let mut page = self.fetch(host, port, selector);
        page.ptype = ptype;
        if page.ptype == PageType::Dir {
            page.parse_links();
        }
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
        let url = self.history.get(self.pos).expect("bad self.pos");
        let page = self.pages.get(url).expect("bad url");
        // clear
        print!("\x1B[2J\x1B[H{}", page.render());
        // print!("{}", page.draw());
        print!("{}", termion::cursor::Hide);
        println!(" \x1B[0;37m{}\x1B[0m", page.input);
    }

    fn respond(&mut self) {
        let mut addr = (String::new(), String::new(), String::new(), PageType::Dir);
        let url = self.history.get(self.pos).expect("bad self.pos");
        let page = self.pages.get_mut(url);
        match page {
            None => return,
            Some(page) => match page.respond() {
                Action::Back => self.back(),
                Action::Forward => self.forward(),
                Action::Fetch(host, port, sel, ptype) => {
                    page.input.clear();
                    addr.0 = host;
                    addr.1 = port;
                    addr.2 = sel;
                    addr.3 = ptype;
                }
                Action::Quit => {
                    println!("{}", termion::cursor::Show);
                    std::process::exit(0);
                }
                _ => {}
            },
        }
        if !addr.0.is_empty() {
            self.load(&addr.0, &addr.1, &addr.2, addr.3);
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
            input: String::new(),
            ptype: PageType::Dir,
            offset: 0,
        }
    }
}

impl Page {
    fn cursor_up(&mut self) {
        if self.links.len() == 0 && self.offset > 0 {
            self.offset -= 1;
        } else if self.link > 1 {
            self.link -= 1;
        }
    }

    fn cursor_down(&mut self) {
        if self.links.len() == 0 {
            self.offset += 1;
        } else if self.link < self.links.len() {
            self.link += 1;
        }
    }

    fn respond(&mut self) -> Action {
        match self.read_input() {
            Action::Up => self.cursor_up(),
            Action::Down => self.cursor_down(),
            Action::PageUp => {
                for _ in 0..=30 {
                    self.cursor_up()
                }
            }
            Action::PageDown => {
                for _ in 0..=30 {
                    self.cursor_down()
                }
            }
            Action::Select(n) => self.link = n + 1,
            Action::Link(n) => {
                if n < self.links.len() {
                    let link = &self.links[n];
                    return Action::Fetch(
                        link.host.to_string(),
                        link.port.to_string(),
                        link.selector.to_string(),
                        link.ptype,
                    );
                }
            }
            Action::Open => {
                if self.link > 0 && self.link - 1 < self.links.len() {
                    let link = &self.links[self.link - 1];
                    return Action::Fetch(
                        link.host.to_string(),
                        link.port.to_string(),
                        link.selector.to_string(),
                        link.ptype,
                    );
                }
            }
            other => return other,
        }
        Action::None
    }

    fn read_input(&mut self) -> Action {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => return Action::Quit,
                Key::Ctrl('c') => {
                    if self.input.len() > 0 {
                        self.input.clear();
                        return Action::None;
                    } else {
                        return Action::Quit;
                    }
                }
                Key::Char('\n') => return Action::Open,
                Key::Up | Key::Ctrl('p') => return Action::Up,
                Key::Down | Key::Ctrl('n') => return Action::Down,
                Key::Left => return Action::Back,
                Key::Right => return Action::Forward,
                Key::Char('-') => {
                    if self.input.is_empty() {
                        return Action::PageUp;
                    }
                }
                Key::Char(' ') => {
                    if self.input.is_empty() {
                        return Action::PageDown;
                    }
                }
                Key::Char(c) => {
                    self.input.push(c);
                    for (i, link) in self.links.iter().enumerate() {
                        // jump to number
                        let count = self.links.len();
                        if count < 10 && c == '1' && i == 0 {
                            return Action::Link(i);
                        } else if count < 20 && c == '2' && i == 1 {
                            return Action::Link(i);
                        } else if count < 30 && c == '3' && i == 2 {
                            return Action::Link(i);
                        } else if count < 40 && c == '4' && i == 3 {
                            return Action::Link(i);
                        } else if count < 50 && c == '5' && i == 4 {
                            return Action::Link(i);
                        } else if count < 60 && c == '6' && i == 5 {
                            return Action::Link(i);
                        } else if count < 70 && c == '7' && i == 6 {
                            return Action::Link(i);
                        } else if count < 80 && c == '8' && i == 7 {
                            return Action::Link(i);
                        } else if count < 90 && c == '9' && i == 8 {
                            return Action::Link(i);
                        } else if self.input.len() > 1 && self.input == (i + 1).to_string() {
                            return Action::Link(i);
                        } else if self.input.len() == 1 && self.input == (i + 1).to_string() {
                            return Action::Select(i);
                        } else {
                            if link
                                .name
                                .to_ascii_lowercase()
                                .contains(&self.input.to_ascii_lowercase())
                            {
                                return Action::Select(i);
                            }
                        }
                    }
                    return Action::None;
                }
                Key::Backspace => {
                    if self.input.is_empty() {
                        return Action::Back;
                    } else {
                        self.input.pop();
                    }
                    return Action::None;
                }
                Key::Delete => {
                    self.input.pop();
                    return Action::None;
                }
                _ => {}
            }
        }
        Action::None
    }

    fn parse_links(&mut self) {
        self.links.clear();
        let mut start = true;
        let mut is_link = false;
        let mut link = (0, 0, PageType::Dir);
        for (i, c) in self.body.chars().enumerate() {
            if start {
                match c {
                    '0' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = PageType::Text;
                    }
                    '1' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = PageType::Dir;
                    }
                    'h' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = PageType::HTML;
                    }
                    '\n' => continue,
                    _ => is_link = false,
                }
                start = false;
            } else if c == '\n' {
                start = true;
                if is_link && i > link.0 {
                    link.1 = i;
                    let mut line = [""; 4];
                    for (j, s) in self.body[link.0..link.1].split('\t').enumerate() {
                        line[j] = s;
                    }
                    self.links.push(Link {
                        name: line[0].to_string(),
                        selector: line[1].to_string(),
                        host: line[2].to_string(),
                        port: line[3].trim_end_matches('\r').to_string(),
                        ptype: link.2,
                    });
                    is_link = false;
                }
            }
        }
        self.link = 1;
    }

    fn render(&self) -> String {
        let (cols, rows) = termion::terminal_size().expect("can't get terminal size");
        self.draw(cols, rows, self.ptype != PageType::Dir)
    }

    fn draw(&self, _cols: u16, rows: u16, text_mode: bool) -> String {
        let mut line = 0;
        let mut start = true;
        let mut skip_to_end = false;
        let mut links = 0;
        let mut out = String::with_capacity(self.body.len() * 2);
        let mut prefix = "";
        for (i, c) in self.body.chars().enumerate() {
            let mut is_link = false;
            if line < self.offset {
                if c == '\n' {
                    line += 1;
                }
                continue;
            }
            if line >= (rows + self.offset - 2) {
                return out;
            }
            if text_mode {
                match c {
                    '\n' => {
                        out.push(c);
                        line += 1;
                    }
                    _ => out.push(c),
                }
                continue;
            }
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
                        prefix = "\x1B[92m";
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
                    '\n' => {
                        line += 1;
                        continue;
                    }
                    _ => {
                        skip_to_end = true;
                        start = false;
                        continue;
                    }
                }
                if is_link && self.link > 0 && self.link == links {
                    out.push_str("\x1b[90;1m*\x1b[1m");
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
                    out.push_str(". ");
                } else {
                    out.push(' ');
                    out.push_str("\x1B[0m");
                    out.push_str("   ");
                }
                out.push_str(&prefix);
                start = false;
            } else if skip_to_end {
                if c == '\n' {
                    out.push_str("\r\n\x1B[0m");
                    start = true;
                    line += 1;
                    skip_to_end = false;
                }
            } else if c == '\t' {
                skip_to_end = true;
            } else {
                if c == '\n' {
                    out.push_str("\x1B[0m\r\n");
                    line += 1;
                    start = true;
                } else {
                    out.push(c);
                }
            }
        }
        out
    }
}
