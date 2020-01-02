use crate::ui::{Action, Key, View, MAX_COLS, SCROLL_LINES};
use std::fmt;

pub struct Text {
    url: String,
    raw_response: String,
    scroll: usize,        // offset
    lines: usize,         // # of lines
    longest: usize,       // longest line
    size: (usize, usize), // cols, rows
    pub wide: bool,       // in wide mode? turns off margins
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

impl View for Text {
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
            Key::Home => {
                self.scroll = 0;
                Action::Redraw
            }
            Key::End => {
                self.scroll = self.final_scroll();
                Action::Redraw
            }
            Key::Char('w') | Key::Ctrl('w') => {
                self.wide = !self.wide;
                Action::Redraw
            }
            Key::Down | Key::Ctrl('n') | Key::Char('n') | Key::Ctrl('j') | Key::Char('j') => {
                if self.scroll < self.final_scroll() {
                    self.scroll += 1;
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::Up | Key::Ctrl('p') | Key::Char('p') | Key::Ctrl('k') | Key::Char('k') => {
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
                self.scroll += SCROLL_LINES;
                if self.scroll > self.final_scroll() {
                    self.scroll = self.final_scroll();
                }
                Action::Redraw
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

        let mut lines = 0;

        for line in iter {
            lines += 1;
            if line == ".\r" {
                continue;
            }
            let mut line_size = 0;
            if !self.wide {
                out.push_str(&indent);
                line_size += indent.len();
            }
            out.push_str(line);
            line_size += line.len();

            // clear rest of line
            out.push_str(&" ".repeat(cols - line_size)); // fill line

            out.push_str("\r\n");
        }

        // clear remainder of screen
        let blank_line = " ".repeat(cols);
        for _ in 0..rows - lines - 1 {
            out.push_str(&blank_line);
            out.push_str(&"\r\n");
        }

        out
    }
}

impl Text {
    pub fn from(url: String, response: String) -> Text {
        let mut lines = 0;
        let mut longest = 0;
        for line in response.split_terminator('\n') {
            lines += 1;
            if line.len() > longest {
                longest = line.len();
            }
        }

        Text {
            url,
            raw_response: response,
            scroll: 0,
            lines,
            longest,
            size: (0, 0),
            wide: false,
        }
    }

    /// Final `self.scroll` value.
    fn final_scroll(&self) -> usize {
        let padding = (self.size.1 as f64 * 0.9) as usize;
        if self.lines > padding {
            self.lines - padding
        } else {
            0
        }
    }
}
