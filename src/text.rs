use ui::{Action, Key, View};

pub struct TextView {
    url: String,
    raw: String,
}

impl View for TextView {
    fn process_input(&mut self, c: Key) -> Action {
        Action::None
    }

    fn render(&self, width: u16, height: u16) -> String {
        self.raw.to_string()
    }
}

impl TextView {
    pub fn from(url: String, response: String) -> TextView {
        TextView { url, raw: response }
    }
}
