//! A View representing a Gopher text entry.
//! Responds to user input by producing an Action which is then handed
//! to the main UI to perform.

use crate::{
    config::Config,
    ui::{self, Action, Key, View, MAX_COLS, SCROLL_LINES},
};
use std::fmt;
use termion::clear;

/// The Text View holds the raw Gopher response as well as information
/// about which lines should currently be displayed on screen.
pub struct Text {
    url: String,
    raw_response: String,
    scroll: usize,        // offset
    lines: usize,         // # of lines
    longest: usize,       // longest line
    size: (usize, usize), // cols, rows
    pub tls: bool,        // retrieved via tls?
    pub tor: bool,        // retrieved via tor?
    pub wide: bool,       // in wide mode? turns off margins
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

impl View for Text {
    fn is_tls(&self) -> bool {
        self.tls
    }

    fn is_tor(&self) -> bool {
        self.tor
    }

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

    fn render(&mut self, cfg: &Config) -> String {
        self.wide = cfg.wide;
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
            " ".repeat((cols - longest) / 2)
        } else {
            String::from("")
        };
        let limit = if cfg.mode == ui::Mode::Run {
            rows - 1
        } else {
            self.lines
        };
        let iter = self
            .raw_response
            .split_terminator('\n')
            .skip(self.scroll)
            .take(limit);

        for line in iter {
            // Check for Gopher's weird "end of response" line.
            if line == ".\r" || line == "." {
                continue;
            }
            if !self.wide {
                out.push_str(&indent);
            }
            let line = line.trim_end_matches('\r').replace('\t', "    ");
            out.push_str(&line);

            // clear rest of line
            out.push_str(&format!("{}", clear::UntilNewline));

            out.push_str("\r\n");
        }

        // clear remainder of screen
        out.push_str(&format!("{}", clear::AfterCursor));

        out
    }
}

impl Text {
    pub fn from(url: String, response: String, tls: bool, tor: bool) -> Text {
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
            tls,
            tor,
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
