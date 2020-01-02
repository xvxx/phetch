use crate::ui::Key;
use std::fmt;

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

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::None => write!(f, "None"),
            Action::Open(title, url) => write!(f, "Open: {}, {}", title, url),
            Action::Keypress(key) => write!(f, "Keypress: {:?}", key),
            Action::Redraw => write!(f, "Redraw"),
            Action::Draw(s) => write!(f, "Draw: {:?}", s),
            Action::Status(s) => write!(f, "Status: {}", s),
            Action::Prompt(s, _) => write!(f, "Prompt: {}", s),
            Action::List(li) => {
                write!(f, "List: \n");
                for a in li {
                    write!(f, "{:?}", a);
                }
                Ok(())
            }
            Action::Error(s) => write!(f, "Error: {}", s),
        }
    }
}
