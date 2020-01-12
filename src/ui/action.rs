use crate::ui::Key;
use std::fmt;

/// Views generate Actions in response to user input, which are
/// processed by the UI.
pub enum Action {
    /// Do nothing. Eg: Hit the down arrow but can't go down.
    None,
    /// Open a URL: open(title, url)
    Open(String, String),
    /// If the View doesn't know how to react, it returns the keypress.
    Keypress(Key),
    /// Redraw the screen. Can cause a flicker
    Redraw,
    /// Draw something on screen. This assumes you're using Goto(X,Y)
    /// to control where exactly you're drawing it. Nothing else will
    /// be redrawn.
    Draw(String),
    /// Set the "status" line to some text.
    Status(String),
    /// Show a prompt and ask for the user to input text.
    /// The callback is passed what the user entered, if they type
    /// something in and hit enter. If they cancel, the callback is
    /// not run.
    /// Prompt(Prompt Query, Callback)
    Prompt(String, Box<dyn FnOnce(String) -> Action>),
    /// Do more than one action.
    List(Vec<Action>),
    /// Display an error message.
    Error(String),
}

impl Action {
    /// Is it Action::None?
    pub fn is_none(&self) -> bool {
        if let Action::None = self {
            true
        } else {
            false
        }
    }
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
                writeln!(f, "List: ");
                for a in li {
                    write!(f, "{:?}", a);
                }
                Ok(())
            }
            Action::Error(s) => write!(f, "Error: {}", s),
        }
    }
}
