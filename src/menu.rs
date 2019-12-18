use gopher;
use gopher::Type;
use std::io::stdout;
use std::io::Write;
use ui::{Action, Key, View, MAX_COLS, SCROLL_LINES};

pub struct MenuView {
    pub input: String, // user's inputted value
    pub menu: Menu,    // data
    pub link: usize,   // selected link
    pub scroll: usize, // scrolling offset
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
    fn render(&self, cols: usize, rows: usize) -> String {
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

    fn render_lines(&self, cols: usize, rows: usize) -> String {
        let mut out = String::new();

        macro_rules! push {
            ($c:expr, $e:expr) => {{
                out.push_str("\x1b[");
                out.push_str($c);
                out.push_str("m");
                out.push_str(&$e);
                out.push_str("\x1b[0m");
            }};
        }

        let iter = self.lines().iter().skip(self.scroll).take(rows - 1);
        let longest = if self.menu.longest > MAX_COLS {
            MAX_COLS
        } else {
            self.menu.longest
        };
        let indent = if longest > cols {
            String::from("")
        } else {
            let left = (cols - longest) / 2;
            if left > 6 {
                " ".repeat(left - 6)
            } else {
                String::from("")
            }
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
            // truncate long lines, instead of wrapping
            let name = if line.name.len() + 6 > cols {
                line.name.chars().take(cols - 6).collect::<String>()
            } else {
                line.name.to_string()
            };
            match line.typ {
                Type::Text => push!("96", name),
                Type::Menu => push!("94", name),
                Type::Info => push!("93", name),
                Type::HTML => push!("92", name),
                Type::Error => push!("91", name),
                typ if typ.is_download() => push!("4;97", name),
                _ => push!("0", name),
            }
            out.push('\n');
        }
        if self.lines().len() < rows {
            // fill in empty space
            out.push_str(&format!(
                "{}",
                " \r\n".repeat(rows - 1 - self.lines().len())
            ));
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
        let lines = self.lines().len();
        if lines > SCROLL_LINES && self.scroll < lines - SCROLL_LINES {
            self.scroll += SCROLL_LINES;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_page_up(&mut self) -> Action {
        if self.scroll > 0 {
            if self.scroll > SCROLL_LINES {
                self.scroll -= SCROLL_LINES;
            } else {
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
        let count = self.links().count();
        if count > 0 && self.link < count - 1 {
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
            Key::Esc => {
                if self.input.len() > 0 {
                    self.input.clear();
                    self.redraw_input()
                } else {
                    Action::None
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
            Key::Char('-') => {
                if self.input.is_empty() {
                    self.action_page_up()
                } else {
                    self.input.push('-');
                    self.redraw_input()
                }
            }
            Key::PageUp => self.action_page_up(),
            Key::PageDown => self.action_page_down(),
            Key::Char(' ') => {
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

                // jump to <10 number
                if input.len() == 1 {
                    if let Some(c) = input.chars().nth(0) {
                        if c.is_digit(10) {
                            let i = c.to_digit(10).unwrap() as usize;
                            if i < count {
                                if count < (i * 10) {
                                    return self.action_follow_link(i - 1);
                                } else {
                                    return self.action_select_link(i - 1);
                                }
                            }
                        }
                    }
                } else if input.len() == 2 {
                    // jump to >=10 number
                    let s = input.chars().take(2).collect::<String>();
                    if let Ok(num) = s.parse::<usize>() {
                        if num <= count {
                            if count < (num * 10) {
                                return self.action_follow_link(num - 1);
                            } else {
                                return self.action_select_link(num - 1);
                            }
                        }
                    }
                } else if input.len() == 3 {
                    // jump to >=100 number
                    let s = input.chars().take(3).collect::<String>();
                    if let Ok(num) = s.parse::<usize>() {
                        if num <= count {
                            if count < (num * 10) {
                                return self.action_follow_link(num - 1);
                            } else {
                                return self.action_select_link(num - 1);
                            }
                        }
                    }
                }

                for i in 0..count {
                    // check for name match
                    let name = if let Some(link) = self.links().nth(i) {
                        link.name.to_ascii_lowercase()
                    } else {
                        "".to_string()
                    };

                    if name.contains(&self.input.to_ascii_lowercase()) {
                        return self.action_select_link(i);
                    }
                }

                self.link = 0;
                Action::Redraw
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
                let typ = match gopher::type_for_char(c) {
                    Some(t) => t,
                    None => continue,
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
                    url.push_str("/");
                    url.push(first_char);
                }

                if parts.len() > 1 {
                    url.push_str(parts[1]); // selector
                }
                let mut name = String::from("");
                if parts[0].len() > 0 {
                    name.push_str(&parts[0][1..]);
                }
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

