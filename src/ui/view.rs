use crate::{config::Config, ui};
use std::fmt;

/// Views represent what's on screen, a Gopher Menu/Text/etc item.
pub trait View: fmt::Display {
    fn respond(&mut self, key: ui::Key) -> ui::Action;
    fn render(&mut self, cfg: &Config) -> String;
    fn is_tls(&self) -> bool;
    fn is_tor(&self) -> bool;
    fn url(&self) -> String;
    fn raw(&self) -> String;
    fn term_size(&mut self, cols: usize, rows: usize);
}
