use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
    scroll: usize, // offset
    lines: usize,  // # of lines
}

impl View for TextView {
    fn process_input(&mut self, c: Key) -> Action {
        let jump = 15;
        match c {
            Key::Char('t') | Key::Char('g') => {
                self.scroll = 0;
                Action::Redraw
            }
            Key::Char('b') | Key::Char('G') => {
                self.scroll = self.lines - jump;
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
                    self.scroll -= jump;
                    if self.scroll <= 0 {
                        self.scroll = 0;
                    }
                    Action::Redraw
                } else {
                    Action::None
                }
            }
            Key::PageDown | Key::Char(' ') => {
                if self.scroll < self.lines - 1 - jump {
                    self.scroll += jump;
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

    fn render(&self, width: u16, height: u16) -> String {
        let mut out = String::new();
        for (i, line) in self.raw.split_terminator('\n').enumerate() {
            if i > (self.scroll + height as usize) - 2 {
                break;
            }
            if i < self.scroll {
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
        let lines = response.split_terminator('\n').count();
        TextView {
            url,
            raw: response,
            scroll: 0,
            lines,
        }
    }
}
