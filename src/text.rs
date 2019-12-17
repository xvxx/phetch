use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
    scroll: usize,  // offset
    lines: usize,   // # of lines
    longest: usize, // longest line
}

const SCROLL_LINES: usize = 15;

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
                if self.lines >= SCROLL_LINES {
                    self.scroll = self.lines - SCROLL_LINES;
                    Action::Redraw
                } else {
                    Action::None
                }
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
                    if self.scroll >= SCROLL_LINES {
                        self.scroll -= SCROLL_LINES;
                    } else {
                        self.scroll = 0;
                    }
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::PageDown | Key::Char(' ') => {
                let lines = self.lines - 1;
                if lines > SCROLL_LINES {
                    if self.scroll < lines - SCROLL_LINES {
                        self.scroll += SCROLL_LINES;
                        if self.scroll >= lines {
                            self.scroll = lines;
                        }
                    }
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            _ => Action::Unknown,
        }
    }

    fn render(&self, cols: usize, rows: usize) -> String {
        let mut out = String::new();
        let indent = if self.longest > cols {
            String::from("")
        } else {
            let left = (cols - self.longest) / 2;
            if left > 6 {
                " ".repeat(left - 6)
            } else {
                String::from("")
            }
        };
        let iter = self
            .raw
            .split_terminator('\n')
            .skip(self.scroll)
            .take(rows - 1);

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
