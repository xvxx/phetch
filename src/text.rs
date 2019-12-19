use ui::{Action, Key, View, MAX_COLS, SCROLL_LINES};

pub struct TextView {
    url: String,
    raw_response: String,
    scroll: usize,        // offset
    lines: usize,         // # of lines
    longest: usize,       // longest line
    size: (usize, usize), // cols, rows
}

impl View for TextView {
    fn url(&self) -> String {
        self.url.to_string()
    }

    fn raw(&self) -> String {
        self.raw_response.to_string()
    }

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    fn respond(&mut self, c: Key) -> Action {
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
            _ => Action::Keypress(c),
        }
    }

    fn render(&self) -> String {
        let (cols, rows) = self.size;
        let mut out = String::new();
        let longest = if self.longest > MAX_COLS {
            MAX_COLS
        } else {
            self.longest
        };
        let indent = if cols >= longest && cols - longest <= 6 {
            String::from("")
        } else if cols >= longest {
            let left = (cols - longest) / 2;
            " ".repeat(left)
        } else {
            String::from("")
        };
        let iter = self
            .raw_response
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
            raw_response: response,
            scroll: 0,
            lines,
            longest,
            size: (0, 0),
        }
    }
}
