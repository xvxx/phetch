use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use types::Type;
use ui::{Action, Key, View};

#[derive(Debug)]
pub struct PageView {
    pub input: String, // user's inputted value
    pub page: Page,    // data
    pub line: usize,   // selected line
    pub scroll: usize, // scrolling offset
}

#[derive(Debug)]
pub struct Page {
    lines: Vec<Line>, // lines
    typ: Type,        // entry type
    raw: String,      // raw gopher response
    url: String,      // gopher url
}

#[derive(Debug)]
pub struct Line {
    pos: usize, // which line in the page
    name: String,
    host: String,
    port: String,
    selector: String,
    typ: Type,
}

impl View for PageView {
    fn process_input(&mut self, key: Key) -> Action {
        match key {
            Key::Char('\n') => return Action::Open,
            Key::Backspace => {
                if self.input.is_empty() {
                    Action::Back
                } else {
                    self.input.pop();
                    Action::None
                }
            }
            Key::Delete => {
                self.input.pop();
                Action::None
            }
            Key::Backspace => {
                if self.input.is_empty() {
                    Action::Back
                } else {
                    self.input.pop();
                    Action::None
                }
            }
            Key::Delete => {
                self.input.pop();
                Action::None
            }
            Key::Ctrl('c') => {
                if self.input.len() > 0 {
                    self.input.clear();
                    Action::None
                } else {
                    Action::Quit
                }
            }
            Key::Char('-') => {
                if self.input.is_empty() {
                    Action::PageUp
                } else {
                    self.input.push('-');
                    Action::None
                }
            }
            Key::Char(' ') => {
                if self.input.is_empty() {
                    return Action::PageDown;
                } else {
                    self.input.push(' ');
                    Action::None
                }
            }
            Key::Char(c) => {
                self.input.push(c);
                for (i, link) in self.page.lines.iter().enumerate() {
                    // jump to number
                    let count = self.page.lines.len();
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
                Action::None
            }
            _ => Action::None,
        }
    }
}

impl PageView {
    pub fn from(page: Page) -> PageView {
        PageView {
            page,
            input: String::new(),
            line: 0,
            scroll: 0,
        }
    }
}

impl Page {
    pub fn from(url: String, gopher_response: String) -> Page {
        Self::parse_menu(url, gopher_response)
    }

    // Loads a Page given a URL.
    pub fn load(host: &str, port: &str, selector: &str) -> io::Result<Page> {
        let url = format!("{}:{}{}", host, port, selector);
        match Self::fetch(host, port, selector) {
            Ok(res) => Ok(Page::from(url, res)),
            Err(e) => Err(e),
        }
    }

    // Fetches a URL and returns a raw Gopher response.
    fn fetch(host: &str, port: &str, selector: &str) -> io::Result<String> {
        let mut body = String::new();
        let stream = TcpStream::connect(format!("{}:{}", host, port))
            .and_then(|mut stream| {
                stream.write(format!("{}\r\n", selector).as_ref());
                Ok(stream)
            })
            .and_then(|mut stream| {
                stream.read_to_string(&mut body);
                Ok(())
            });

        match stream {
            Ok(_) => Ok(body),
            Err(e) => Err(e),
        }
    }

    // Parses the lines in a raw Gopher menu response.
    fn parse_menu(url: String, raw: String) -> Page {
        let mut lines = vec![];
        let mut line = (0, 0, Type::Menu); // (name start pos, name end, type)
        let mut start = true; // are we at beginning of a line?
        let mut count = 0; // which line # are we
        let mut skip_line = false;

        for (i, c) in raw.char_indices() {
            if start {
                line.0 = i + 1;
                match c {
                    '0' => {
                        line.2 = Type::Text;
                    }
                    '1' => {
                        line.2 = Type::Menu;
                    }
                    'h' => {
                        line.2 = Type::HTML;
                    }
                    'i' => {
                        line.2 = Type::Info;
                    }
                    '\n' => continue,
                    _ => {
                        eprintln!("unknown line type: {}", c);
                        skip_line = true;
                    }
                }
                start = false;
            } else if c == '\n' {
                start = true;
                if skip_line {
                    skip_line = false;
                    continue;
                }
                if i > line.0 {
                    line.1 = i;
                    let mut parts = [""; 4];
                    for (j, s) in raw[line.0..line.1].split('\t').enumerate() {
                        if j < parts.len() {
                            parts[j] = s;
                        }
                    }
                    lines.push(Line {
                        name: parts[0].to_string(),
                        selector: parts[1].to_string(),
                        host: parts[2].to_string(),
                        port: parts[3].trim_end_matches('\r').to_string(),
                        typ: line.2,
                        pos: count,
                    });
                    count += 1;
                }
            }
        }

        Page {
            raw,
            url,
            lines,
            typ: Type::Menu,
        }
    }
}
