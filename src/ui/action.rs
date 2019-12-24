use crate::ui::Key;

#[derive(Debug)]
pub enum Action {
    None,                 // do nothing
    Open(String, String), // open(title, url)
    Keypress(Key),        // unknown keypress
    Redraw,               // redraw everything
    Error(String),        // error message
}
