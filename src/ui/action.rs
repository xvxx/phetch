use ui::Key;

#[derive(Debug)]
pub enum Action {
    None,          // do nothing
    Open(String),  // open url
    Keypress(Key), // unknown keypress
    Redraw,        // redraw everything
    Error(String), // error message
}
