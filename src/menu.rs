use gopher::Type;
use std::io;
use ui::{Action, Key, View};

#[derive(Debug)]
pub struct MenuView {
    pub input: String, // user's inputted value
    pub menu: Menu,    // data
    pub line: usize,   // selected line
    pub scroll: usize, // scrolling offset
}

#[derive(Debug)]
pub struct Menu {
    url: String,      // gopher url
    lines: Vec<Line>, // lines
    raw: String,      // raw gopher response
}

#[derive(Debug)]
pub struct Line {
    name: String,
    url: String,
    typ: Type,
    link: usize, // link #, if any
}

impl View for MenuView {
    fn render(&self) -> String {
        self.render_lines()
    }

    fn process_input(&mut self, key: Key) -> Action {
        match self.process_key(key) {
            a @ Action::Unknown => return a,
            a => a,
        }
    }
}

impl MenuView {
    pub fn from(url: String, response: String) -> MenuView {
        MenuView {
            menu: Menu::from(url, response),
            input: String::new(),
            line: 0,
            scroll: 0,
        }
    }

    fn lines(&self) -> &Vec<Line> {
        &self.menu.lines
    }

    fn links(&self) -> impl Iterator<Item = &Line> {
        self.menu.links()
    }

    fn render_lines(&self) -> String {
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

        let mut links = 0;
        for line in self.lines() {
            if line.typ == Type::Info {
                out.push_str("     ");
            } else {
                links += 1;
                out.push(' ');
                out.push_str("\x1b[95m");
                if links < 10 {
                    out.push(' ');
                }
                out.push_str(&links.to_string());
                out.push_str(".\x1b[0m ");
            }
            match line.typ {
                Type::Text => push!("96", &line.name),
                Type::Menu => push!("94", &line.name),
                Type::Info => push!("93", &line.name),
                Type::HTML => push!("92", &line.name),
                _ => {}
            }
            out.push('\n');
        }
        out
    }

    fn action_page_down(&self) {}
    fn action_page_up(&self) {}
    fn action_up(&self) {}
    fn action_down(&self) {}

    fn action_follow_link(&self, line: usize) -> Action {
        if let Some(line) = self.links().nth(line) {
            Action::Open(line.url.to_string())
        } else {
            Action::None
        }
    }

    fn process_key(&mut self, key: Key) -> Action {
        match key {
            Key::Char('\n') => {
                if let Some(line) = self.lines().get(self.line) {
                    Action::Open(line.url.to_string())
                } else {
                    Action::None
                }
            }
            Key::Up | Key::Ctrl('p') => {
                self.action_up();
                Action::None
            }
            Key::Down | Key::Ctrl('n') => {
                self.action_down();
                Action::None
            }
            Key::Backspace => {
                if self.input.is_empty() {
                    Action::Back
                } else {
                    self.input.pop();
                    Action::Input
                }
            }
            Key::Delete => {
                self.input.pop();
                Action::Input
            }
            Key::Ctrl('c') => {
                if self.input.len() > 0 {
                    self.input.clear();
                    Action::Input
                } else {
                    Action::Quit
                }
            }
            Key::Char('-') => {
                if self.input.is_empty() {
                    self.action_page_up();
                    Action::None
                } else {
                    self.input.push('-');
                    Action::Input
                }
            }
            Key::Char(' ') => {
                if self.input.is_empty() {
                    self.action_page_down();
                    Action::None
                } else {
                    self.input.push(' ');
                    Action::Input
                }
            }
            Key::Char(c) => {
                self.input.push(c);
                let count = self.links().count();
                for (i, link) in self.links().enumerate() {
                    // jump to number
                    if count < 10 && c == '1' && i == 0 {
                        return self.action_follow_link(i);
                    } else if count < 20 && c == '2' && i == 1 {
                        return self.action_follow_link(i);
                    } else if count < 30 && c == '3' && i == 2 {
                        return self.action_follow_link(i);
                    } else if count < 40 && c == '4' && i == 3 {
                        return self.action_follow_link(i);
                    } else if count < 50 && c == '5' && i == 4 {
                        return self.action_follow_link(i);
                    } else if count < 60 && c == '6' && i == 5 {
                        return self.action_follow_link(i);
                    } else if count < 70 && c == '7' && i == 6 {
                        return self.action_follow_link(i);
                    } else if count < 80 && c == '8' && i == 7 {
                        return self.action_follow_link(i);
                    } else if count < 90 && c == '9' && i == 8 {
                        return self.action_follow_link(i);
                    } else if self.input.len() > 1 && self.input == (i + 1).to_string() {
                        return self.action_follow_link(i);
                    } else if self.input.len() == 1 && self.input == (i + 1).to_string() {
                        return self.action_follow_link(i);
                    } else {
                        if link
                            .name
                            .to_ascii_lowercase()
                            .contains(&self.input.to_ascii_lowercase())
                        {
                            return self.action_follow_link(i);
                        }
                    }
                }
                Action::Input
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

                let parts: Vec<&str> = line.split_terminator("\t").collect();
                let mut url = String::from("gopher://");
                if parts.len() > 2 {
                    url.push_str(parts[2]); // host
                }
                if parts.len() > 3 {
                    url.push(':');
                    url.push_str(parts[3].trim_end_matches('\r')); // port
                }
                if parts.len() > 1 {
                    url.push_str(parts[1]); // selector
                }
                let name = parts[0][1..].to_string();
                link += 1;
                let link = if typ == Type::Info { 0 } else { link };

                lines.push(Line {
                    name,
                    url,
                    typ,
                    link,
                });
            }
        }

        Menu { raw, url, lines }
    }
}
