use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
    scroll: isize,  // offset
    lines: isize,   // # of lines
    longest: usize, // longest line
}

const SCROLL_LINES: isize = 15;

impl View for TextView {
    fn url(&self) -> String {
        self.url.to_string()
    }

    fn process_input(&mut self, c: Key) -> Action {
        match c {
            Key::Char('t') | Key::Char('g') => {
                self.scroll = 0;
                Action::Redraw
            }
            Key::Char('b') | Key::Char('G') => {
                self.scroll = self.lines - SCROLL_LINES;
                Action::Redraw
            }
            Key::Down => {
                if self.scroll < self.lines - 1 {
                    self.scroll += 1;
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::Up => {
                if self.scroll > 0 {
                    self.scroll -= 1;
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::PageUp | Key::Char('-') => {
                if self.scroll > 0 {
                    self.scroll -= SCROLL_LINES;
                    if self.scroll < 0 {
                        self.scroll = 0;
                    }
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::PageDown | Key::Char(' ') => {
                if self.scroll < self.lines - 1 - SCROLL_LINES {
                    self.scroll += SCROLL_LINES;
                    if self.scroll >= self.lines {
                        self.scroll = self.lines - 1;
                    }
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            _ => Action::Unknown,
        }
    }

    fn render(&self, cols: u16, rows: u16) -> String {
        let mut out = String::new();
        let indent = if self.longest > cols as usize {
            String::from("")
        } else {
            " ".repeat((cols as usize - self.longest) / 2)
        };
        let iter = self
            .raw
            .split_terminator('\n')
            .skip(self.scroll as usize)
            .take(rows as usize - 1);

        for line in iter {
            out.push_str(&indent);
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

impl TextView {
    pub fn from(url: String, response: String) -> TextView {
        let mut lines = 0;
        let mut longest = 0;
        for line in response.split_terminator('\n') {
            lines += 1;
            if line.len() > longest {
                longest = line.len();
            }
        }

        TextView {
            url,
            raw: response,
            scroll: 0,
            lines,
            longest,
        }
    }
}
