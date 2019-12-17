use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
}

impl View for TextView {
    fn process_input(&mut self, c: Key) -> Action {
        Action::Unknown
    }

    fn render(&self, width: u16, height: u16) -> String {
        let mut out = String::new();
        for (i, line) in self.raw.split_terminator('\n').enumerate() {
            if i as u16 > height - 4 {
                break;
            }
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

impl TextView {
    pub fn from(url: String, response: String) -> TextView {
        TextView { url, raw: response }
    }
}
