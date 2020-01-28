use crate::ui;
use std::fmt;

/// Views represent what's on screen, a Gopher Menu/Text/etc item.
pub trait View: fmt::Display {
    /// Respond to a user action, specifically a keypress, by
    /// returning an Action enum.
    fn respond(&mut self, key: ui::Key) -> ui::Action;
    /// Create a String of the current view's state that can be
    /// printed to the screen.
    fn render(&mut self) -> String;
    /// Was this View's content fetched using TLS?
    fn is_tls(&self) -> bool;
    /// Was this View's content fetched over Tor?
    fn is_tor(&self) -> bool;
    /// The Gopher URL this View represents.
    fn url(&self) -> &str;
    /// The raw Gopher representation of this View.
    fn raw(&self) -> &str;
    /// Set wide mode on this view.
    fn set_wide(&mut self, wide: bool);
    /// In wide mode?
    fn wide(&mut self) -> bool;
    /// Set the current screen size.
    fn term_size(&mut self, cols: usize, rows: usize);
}
