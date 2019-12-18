use ui::{Action, Key, View, MAX_COLS, SCROLL_LINES};

pub struct TextView {
    url: String,
    raw: String,
    scroll: usize,  // offset
    lines: usize,   // # of lines
    longest: usize, // longest line
}

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
            Key::Down | Key::Ctrl('n') | Key::Char('j') => {
                if self.lines > SCROLL_LINES && self.scroll < (self.lines - SCROLL_LINES) {
                    self.scroll += 1;
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::Up | Key::Ctrl('p') | Key::Char('k') => {
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
        let longest = if self.longest > MAX_COLS {
            MAX_COLS
        } else {
            self.longest
        };
        let indent = if longest > cols {
            String::from("")
        } else {
            let left = (cols - longest) / 2;
            " ".repeat(left)
        };
        let iter = self
            .raw
            .split_terminator('\n')
            .skip(self.scroll)
            .take(rows - 1);

        for line in iter {
            if line == ".\r" {
                continue;
            }
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
