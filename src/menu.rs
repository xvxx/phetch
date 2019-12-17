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
}

impl View for MenuView {
    fn render(&self) -> String {
        let mut out = self.menu.raw.to_string();
        out.push('\n');
        out.push_str(&format!("{:#?}", self));
        out
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

    fn action_page_down(&self) {}
    fn action_page_up(&self) {}
    fn action_up(&self) {}
    fn action_down(&self) {}

    fn process_key(&mut self, key: Key) -> Action {
        match key {
            Key::Char('\n') => {
                if let Some(line) = self.menu.lines.get(self.line) {
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
                for (i, link) in self.menu.lines.iter().enumerate() {
                    // jump to number
                    let count = self.menu.lines.len();
                    if count < 10 && c == '1' && i == 0 {
                        return Action::FollowLink(i);
                    } else if count < 20 && c == '2' && i == 1 {
                        return Action::FollowLink(i);
                    } else if count < 30 && c == '3' && i == 2 {
                        return Action::FollowLink(i);
                    } else if count < 40 && c == '4' && i == 3 {
                        return Action::FollowLink(i);
                    } else if count < 50 && c == '5' && i == 4 {
                        return Action::FollowLink(i);
                    } else if count < 60 && c == '6' && i == 5 {
                        return Action::FollowLink(i);
                    } else if count < 70 && c == '7' && i == 6 {
                        return Action::FollowLink(i);
                    } else if count < 80 && c == '8' && i == 7 {
                        return Action::FollowLink(i);
                    } else if count < 90 && c == '9' && i == 8 {
                        return Action::FollowLink(i);
                    } else if self.input.len() > 1 && self.input == (i + 1).to_string() {
                        return Action::FollowLink(i);
                    } else if self.input.len() == 1 && self.input == (i + 1).to_string() {
                        return Action::FollowLink(i);
                    } else {
                        if link
                            .name
                            .to_ascii_lowercase()
                            .contains(&self.input.to_ascii_lowercase())
                        {
                            return Action::FollowLink(i);
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

    fn parse(url: String, raw: String) -> Menu {
        let mut lines = vec![];
        for line in raw.split_terminator("\n") {
            if let Some(c) = line.chars().nth(0) {
                let typ = match c {
                    '0' => Type::Text,
                    '1' => Type::Menu,
                    '2' => panic!("CSOEntity not supported"), // TODO
                    '3' => Type::Error,
                    '4' => panic!("Binhex not supported"), // TODO
                    '5' => panic!("DOSFile not supported"), // TODO
                    '6' => panic!("UUEncoded not supported"), // TODO
                    '7' => panic!("Search not supported"), // TODO
                    '8' => panic!("Telnet not supported"), // TODO
                    '9' => panic!("Binary not supported"), // TODO
                    '+' => panic!("Mirrors not supported"), // TODO
                    'g' => panic!("GIF not supported"),    // TODO
                    'T' => panic!("Telnet3270 not supported"), // TODO
                    'h' => Type::HTML,
                    'i' => Type::Info,
                    's' => panic!("Sound not supported"), // TODO
                    'd' => panic!("Document not supported"), // TODO
                    '\n' => continue,
                    _ => {
                        eprintln!("unknown line type: {}", c);
                        continue;
                    }
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

                lines.push(Line { name, url, typ });
            }
        }

        Menu { raw, url, lines }
    }
}
