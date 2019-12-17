use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
    scroll: isize, // offset
    lines: isize,  // # of lines
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

    fn render(&self, _cols: u16, rows: u16) -> String {
        let mut out = String::new();
        for (i, line) in self.raw.split_terminator('\n').enumerate() {
            if i as isize > (self.scroll + rows as isize) - 2 {
                break;
            }
            if i < self.scroll as usize {
                continue;
            }
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

impl TextView {
    pub fn from(url: String, response: String) -> TextView {
        let lines = response.split_terminator('\n').count() as isize;
        TextView {
            url,
            raw: response,
            scroll: 0,
            lines,
        }
    }
}
