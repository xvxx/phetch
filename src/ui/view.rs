use std::fmt;
use ui;

pub trait View: fmt::Display {
    fn respond(&mut self, key: ui::Key) -> ui::Action;
    fn render(&self) -> String;
    fn url(&self) -> String;
    fn raw(&self) -> String;
    fn term_size(&mut self, cols: usize, rows: usize);
}
