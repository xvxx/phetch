use gopher::Type;
use std::io::stdout;
use std::io::Write;
use ui::{Action, Key, View};

pub struct MenuView {
    pub input: String, // user's inputted value
    pub menu: Menu,    // data
    pub link: usize,   // selected link
    pub scroll: i16,   // scrolling offset
}

pub struct Menu {
    url: String,      // gopher url
    lines: Vec<Line>, // lines
    longest: usize,   // size of the longest line
}

#[derive(Debug)]
pub struct Line {
    name: String,
    url: String,
    typ: Type,
    link: usize, // link #, if any
}

impl View for MenuView {
    fn render(&self, cols: u16, rows: u16) -> String {
        self.render_lines(cols, rows)
    }

    fn process_input(&mut self, key: Key) -> Action {
        self.process_key(key)
    }

    fn url(&self) -> String {
        self.menu.url.to_string()
    }
}

impl MenuView {
    pub fn from(url: String, response: String) -> MenuView {
        MenuView {
            menu: Menu::from(url, response),
            input: String::new(),
            link: 0,
            scroll: 0,
        }
    }

    fn lines(&self) -> &Vec<Line> {
        &self.menu.lines
    }

    fn links(&self) -> impl Iterator<Item = &Line> {
        self.menu.links()
    }

    fn render_lines(&self, cols: u16, rows: u16) -> String {
        let mut out = String::new();

        macro_rules! push {
            ($c:expr, $e:expr) => {{
                out.push_str("\x1b[");
                out.push_str($c);
                out.push_str("m");
                out.push_str($e);
                out.push_str("\x1b[0m");
            }};
        }

        let iter = self
            .lines()
            .iter()
            .skip(self.scroll as usize)
            .take(rows as usize - 1);

        let indent = if self.menu.longest > cols as usize {
            String::from("")
        } else {
            " ".repeat(((cols as usize - self.menu.longest) / 2) - 6)
        };

        for line in iter {
            out.push_str(&indent);
            if line.typ == Type::Info {
                out.push_str("      ");
            } else {
                if line.link - 1 == self.link {
                    out.push_str("\x1b[97;1m*\x1b[0m")
                } else {
                    out.push(' ');
                }
                out.push(' ');
                out.push_str("\x1b[95m");
                if line.link < 10 {
                    out.push(' ');
                }
                out.push_str(&line.link.to_string());
                out.push_str(".\x1b[0m ");
            }
            match line.typ {
                Type::Text => push!("96", &line.name),
                Type::Menu => push!("94", &line.name),
                Type::Info => push!("93", &line.name),
                Type::HTML => push!("92", &line.name),
                Type::Error => push!("91", &line.name),
                typ if typ.is_download() => push!("4;97", &line.name),
                _ => push!("0", &line.name),
            }
            out.push('\n');
        }
        out.push_str(&self.input);
        out
    }

    fn redraw_input(&self) -> Action {
        print!("\r\x1b[K{}", self.input);
        stdout().flush();
        Action::None
    }

    fn action_page_down(&mut self) -> Action {
        if (self.scroll as usize) < self.lines().len() - 15 {
            self.scroll += 15;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_page_up(&mut self) -> Action {
        if self.scroll > 0 {
            self.scroll -= 15;
            if self.scroll < 0 {
                self.scroll = 0;
            }
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_up(&mut self) -> Action {
        if self.link > 0 {
            self.link -= 1;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_down(&mut self) -> Action {
        if self.link < self.links().count() - 1 {
            self.link += 1;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_select_link(&mut self, line: usize) -> Action {
        if line < self.links().count() {
            self.link = line;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_follow_link(&mut self, link: usize) -> Action {
        self.input.clear();
        if let Some(line) = self.links().nth(link) {
            let url = line.url.to_string();
            Action::Open(url)
        } else {
            Action::None
        }
    }

    fn process_key(&mut self, key: Key) -> Action {
        match key {
            Key::Char('\n') => {
                self.input.clear();
                if let Some(line) = self.links().nth(self.link) {
                    let url = line.url.to_string();
                    Action::Open(url)
                } else {
                    Action::None
                }
            }
            Key::Up | Key::Ctrl('p') | Key::Ctrl('k') => self.action_up(),
            Key::Down | Key::Ctrl('n') | Key::Ctrl('j') => self.action_down(),
            Key::Backspace | Key::Delete => {
                if self.input.is_empty() {
                    Action::Back
                } else {
                    self.input.pop();
                    self.redraw_input()
                }
            }
            Key::Ctrl('c') => {
                if self.input.len() > 0 {
                    self.input.clear();
                    self.redraw_input()
                } else {
                    Action::Quit
                }
            }
            Key::PageUp | Key::Char('-') => {
                if self.input.is_empty() {
                    self.action_page_up()
                } else {
                    self.input.push('-');
                    self.redraw_input()
                }
            }
            Key::PageDown | Key::Char(' ') => {
                if self.input.is_empty() {
                    self.action_page_down()
                } else {
                    self.input.push(' ');
                    self.redraw_input()
                }
            }
            Key::Char(c) => {
                self.input.push(c);
                let count = self.links().count();
                let input = &self.input;
                for i in 0..count {
                    // jump to number
                    for z in 1..=9 {
                        if count < (z * 10) && c == to_char(z as u32) && i == z - 1 {
                            return self.action_follow_link(i);
                        }
                    }
                    if input.len() > 1 && input == &(i + 1).to_string() {
                        return self.action_select_link(i);
                    } else {
                        let name = if let Some(link) = self.links().nth(i) {
                            link.name.to_ascii_lowercase()
                        } else {
                            "".to_string()
                        };

                        if name.contains(&self.input.to_ascii_lowercase()) {
                            return self.action_select_link(i);
                        }
                    }
                }
                self.action_select_link(0)
            }
            _ => Action::Unknown,
        }
    }
}

impl Menu {
    pub fn from(url: String, gopher_response: String) -> Menu {
        Self::parse(url, gopher_response)
    }

    pub fn links(&self) -> impl Iterator<Item = &Line> {
        self.lines.iter().filter(|&line| line.link > 0)
    }

    fn parse(url: String, raw: String) -> Menu {
        let mut lines = vec![];
        let mut link = 0;
        let mut longest = 0;
        for line in raw.split_terminator("\n") {
            if let Some(c) = line.chars().nth(0) {
                let typ = match c {
                    '0' => Type::Text,
                    '1' => Type::Menu,
                    '2' => Type::CSOEntity,
                    '3' => Type::Error,
                    '4' => Type::Binhex,
                    '5' => Type::DOSFile,
                    '6' => Type::UUEncoded,
                    '7' => Type::Search,
                    '8' => Type::Telnet,
                    '9' => Type::Binary,
                    '+' => Type::Mirror,
                    'g' => Type::GIF,
                    'T' => Type::Telnet3270,
                    'h' => Type::HTML,
                    'i' => Type::Info,
                    's' => Type::Sound,
                    'd' => Type::Document,
                    '.' => continue,
                    '\n' => continue,
                    _ => continue,
                };

                // build string URL
                let parts: Vec<&str> = line.split_terminator("\t").collect();
                let mut url = String::from("gopher://");
                if parts.len() > 2 {
                    url.push_str(parts[2]); // host
                }
                if parts.len() > 3 {
                    // port
                    let port = parts[3].trim_end_matches('\r');
                    if port != "70" {
                        url.push(':');
                        url.push_str(parts[3].trim_end_matches('\r'));
                    }
                }

                // auto-prepend gopher type to selector
                if let Some(first_char) = parts[0].chars().nth(0) {
                    if first_char == '0' || first_char == '1' || first_char == 'h' {
                        url.push_str("/");
                        url.push(first_char);
                    }
                }

                if parts.len() > 1 {
                    url.push_str(parts[1]); // selector
                }
                let name = parts[0][1..].to_string();
                if typ != Type::Info {
                    link += 1;
                }
                let link = if typ == Type::Info { 0 } else { link };

                if name.len() > longest {
                    longest = name.len();
                }

                lines.push(Line {
                    name,
                    url,
                    typ,
                    link,
                });
            }
        }

        Menu {
            url,
            lines,
            longest,
        }
    }
}

// number -> char of that number
fn to_char(c: u32) -> char {
    if let Some(ch) = std::char::from_digit(c, 10) {
        ch
    } else {
        '0'
    }
}
