use ui::{Action, Key, View};

pub struct Stub {
    url: String,
}

impl View for Stub {
    fn respond(&mut self, key: Key) -> Action {
        Action::Keypress(key)
    }
    fn render(&self) -> String {
        self.url()
    }
    fn url(&self) -> String {
        self.url.to_string()
    }
    fn raw(&self) -> String {
        self.url()
    }
    fn term_size(&mut self, _cols: usize, _rows: usize) {}
}

impl Stub {
    pub fn from(url: &str) -> Stub {
        Stub {
            url: url.to_string(),
        }
    }
}
