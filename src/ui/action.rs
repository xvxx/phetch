use crate::ui::Key;

pub enum Action {
    None,                                              // do nothing
    Open(String, String),                              // open(title, url)
    Keypress(Key),                                     // unknown keypress
    Redraw,                                            // redraw everything
    Draw(String),                                      // draw something on screen
    Status(String),                                    // set the "status" line to something
    Prompt(String, Box<dyn FnOnce(String) -> Action>), // query string, callback on success
    Error(String),                                     // error message
}
