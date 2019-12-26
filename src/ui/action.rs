use crate::ui::Key;

pub enum Action {
    None,                                              // do nothing
    Open(String, String),                              // open(title, url)
    Keypress(Key),                                     // unknown keypress
    Redraw,                                            // redraw everything
    Prompt(String, Box<dyn FnOnce(String) -> Action>), // query string, callback on success
    Error(String),                                     // error message
}

impl Action {
    pub fn key_or_ctrl(&self, target: char) -> bool {
        if let Action::Keypress(Key::Ctrl(c)) | Action::Keypress(Key::Char(c)) = self {
            *c == target
        } else {
            false
        }
    }
}
