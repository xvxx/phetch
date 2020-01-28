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
    /// Gopher URL
    url: String,
    /// Gopher response
    raw_response: String,
    /// Current scroll offset, in rows
    scroll: usize,
    /// Number of lines
    lines: usize,
    /// Size of longest line
    longest: usize,
    /// Current screen size, cols and rows
    size: (usize, usize),
    /// Was this page retrieved view TLS?
    pub tls: bool,
    /// Retrieved via Tor?
    pub tor: bool,
    /// UI mode. Interactive (Run), Printing, Raw mode...
    mode: ui::Mode,
    /// Currently in wide mode?
    pub wide: bool,
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

    fn url(&self) -> &str {
        self.url.as_ref()
    }

    fn raw(&self) -> &str {
        self.raw_response.as_ref()
    }

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    fn set_wide(&mut self, wide: bool) {
        self.wide = wide;
    }

    fn wide(&mut self) -> bool {
        self.wide
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

    fn render(&mut self) -> String {
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
        let limit = if self.mode == ui::Mode::Run {
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
    /// Create a Text View from a raw Gopher response and a few options.
    pub fn from(url: &str, response: String, config: &Config, tls: bool) -> Text {
        let mut lines = 0;
        let mut longest = 0;
        for line in response.split_terminator('\n') {
            lines += 1;
            let count = line.chars().count();
            if count > longest {
                longest = count;
            }
        }

        Text {
            url: url.into(),
            raw_response: response,
            scroll: 0,
            lines,
            longest,
            size: (0, 0),
            mode: config.mode,
            tls,
            tor: config.tor,
            wide: config.wide,
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
