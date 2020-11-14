//! A View representing a Gopher text entry.
//! Responds to user input by producing an Action which is then handed
//! to the main UI to perform.

use crate::{
    config::SharedConfig as Config,
    encoding::Encoding,
    terminal,
    ui::{self, Action, Key, View, MAX_COLS, SCROLL_LINES},
};
use std::{fmt, str};

/// The Text View holds the raw Gopher response as well as information
/// about which lines should currently be displayed on screen.
pub struct Text {
    /// Ref to our global config
    config: Config,
    /// Gopher URL
    url: String,
    /// Gopher response
    raw_response: Vec<u8>,
    /// Encoded response
    encoded_response: String,
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
    /// Text Encoding of Response
    encoding: Encoding,
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
        str::from_utf8(&self.raw_response).unwrap_or_default()
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

    fn encoding(&self) -> Encoding {
        self.encoding
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
            Key::Ctrl('e') => self.toggle_encoding(),
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
        let wrap = self.config.read().unwrap().wrap;
        let longest = if self.longest > MAX_COLS {
            MAX_COLS
        } else if wrap < self.longest {
            wrap
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

        let iter = wrap_text(&self.encoded_response, wrap)
            .into_iter()
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
            out.push_str(&format!("{}", terminal::ClearUntilNewline));

            out.push_str("\r\n");
        }

        // clear remainder of screen
        out.push_str(&format!("{}", terminal::ClearAfterCursor));

        out
    }
}

impl Text {
    /// Create a Text View from a raw Gopher response and a few options.
    pub fn from(url: &str, response: Vec<u8>, config: Config, tls: bool) -> Text {
        let mode = config.read().unwrap().mode;
        let tor = config.read().unwrap().tor;
        let encoding = config.read().unwrap().encoding;
        let wide = config.read().unwrap().wide;

        let mut new = Text {
            config,
            url: url.into(),
            encoded_response: String::new(),
            raw_response: response,
            scroll: 0,
            lines: 0,
            longest: 0,
            size: (0, 0),
            mode,
            tls,
            tor,
            encoding,
            wide,
        };
        new.encode_response();
        new
    }

    /// Toggle between our two encodings.
    fn toggle_encoding(&mut self) -> Action {
        if matches!(self.encoding, Encoding::UTF8) {
            self.encoding = Encoding::CP437;
        } else {
            self.encoding = Encoding::UTF8;
        }
        self.config.write().unwrap().encoding = self.encoding;
        self.encode_response();
        Action::Redraw
    }

    /// Convert the response to a Rust String and cache metadata like
    /// the number of lines.
    fn encode_response(&mut self) {
        self.encoded_response = self.encoding.encode(&self.raw_response).into();
        let wrapped = wrap_text(
            self.encoded_response.as_ref(),
            self.config.read().unwrap().wrap,
        );
        self.lines = wrapped.len();
        self.longest = wrapped.iter().map(|line| line.len()).max().unwrap_or(0) as usize;
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

/// Splits a chunk of text into a vector of strings with at most
/// `wrap` characters each. Tries to be smart and wrap at punctuation,
/// otherwise just wraps at `wrap`.
fn wrap_text(lines: &str, wrap: usize) -> Vec<&str> {
    if wrap == 0 {
        return lines.split('\n').collect();
    }

    let mut out = vec![];
    for mut line in lines.lines() {
        let mut len = line.chars().count();
        if len > wrap {
            while len > wrap {
                let (end, _) = line.char_indices().take(wrap + 1).last().unwrap();

                if !matches!(&line[end - 1..end], " " | "-" | "," | "." | ":") {
                    if let Some(&(end, _)) = line
                        .char_indices()
                        .take(wrap + 1)
                        .collect::<Vec<_>>()
                        .iter()
                        .rev()
                        .skip(1)
                        .find(|(_, c)| matches!(c, ' ' | '-' | ',' | '.' | ':'))
                    {
                        out.push(&line[..=end]);
                        if end + 1 < line.len() {
                            line = &line[end + 1..];
                            len -= end;
                        } else {
                            len = 0;
                            break;
                        }
                        continue;
                    }
                }

                out.push(&line[..end]);
                line = &line[end..];
                len -= wrap;
            }
            if len > 0 {
                out.push(line);
            }
        } else {
            out.push(line);
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cp437() {
        let body = include_bytes!("../tests/CP437.txt");
        let mut text = Text::from("", body.to_vec(), Config::default(), false);
        text.mode = ui::Mode::Print;

        let res = text.render();
        assert!(!res.contains("╟"));
        assert!(!res.contains("≈"));
        assert!(!res.contains("Ω"));
        assert!(!res.contains("Θ"));

        text.toggle_encoding();
        let res = text.render();
        assert!(res.contains("╟"));
        assert!(res.contains("≈"));
        assert!(res.contains("Ω"));
        assert!(res.contains("Θ"));
    }

    #[test]
    fn test_wrapping() {
        let text = "regular line
really really really really really really really really really really long line
super duper extra scooper hoopa loopa double doopa maxi paxi giga baxi very very long line
Qua nova re oblata omnis administratio belliconsistit militesque aversi a proelio ad studium audiendi et cognoscendi feruntur ubi hostes ad legatosexercitumque pervenerunt universi se ad pedes proiciunt orant ut adventus Caesaris expectetur captamsuam urbem videre...
really really really really really really really really really kinda-but-not-really long line
another regular line
";

        let lines = wrap_text(text, 70);

        assert_eq!("regular line", lines[0]);
        assert_eq!(
            "really really really really really really really really really really",
            lines[1].trim()
        );
        assert_eq!("long line", lines[2].trim());
        assert_eq!(
            "super duper extra scooper hoopa loopa double doopa maxi paxi giga",
            lines[3].trim()
        );
        assert_eq!("baxi very very long line", lines[4].trim());

        assert_eq!(
            "Qua nova re oblata omnis administratio belliconsistit militesque",
            lines[5].trim()
        );
        assert_eq!(
            "aversi a proelio ad studium audiendi et cognoscendi feruntur ubi",
            lines[6].trim()
        );
        assert_eq!(
            "hostes ad legatosexercitumque pervenerunt universi se ad pedes",
            lines[7].trim()
        );
        assert_eq!(
            "really really really really really really really really really kinda-",
            lines[10].trim()
        );
        assert_eq!("but-not-really long line", lines[11].trim());

        assert_eq!(13, lines.len());
    }
}
