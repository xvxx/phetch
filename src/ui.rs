use page::{Page, PageView};

use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub type Key = termion::event::Key;
pub type Error = io::Error;

#[derive(Debug)]
pub struct UI {
    pages: Vec<PageView>,
    page: usize,
}

#[derive(Debug)]
pub enum Action {
    None,
    Up,
    Down,
    PageUp,
    PageDown,
    Back,
    Forward,
    Open,
    FollowLink(usize),
    Quit,
    Unknown,
}

pub trait View {
    fn process_input(&mut self, c: Key) -> Action;
}

impl UI {
    pub fn new() -> UI {
        UI {
            pages: vec![],
            page: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.print();
            self.respond_to_user();
        }
    }

    pub fn print(&self) {
        // print!("{}", self.render());
        print!("{:#?}", self);
    }

    pub fn render(&self) -> String {
        // let (cols, rows) = termion::terminal_size().expect("can't get terminal size");
        String::new()
    }

    pub fn load(&mut self, host: &str, port: &str, selector: &str) {
        match Page::load(host, port, selector) {
            Ok(page) => self.add_page(PageView::from(page)),
            Err(e) => {
                eprintln!(
                    "\x1B[91merror loading \x1b[93m{}:{}{}: \x1B[0m{}",
                    host, port, selector, e
                );
                std::process::exit(1);
            }
        }
    }

    fn add_page(&mut self, page: PageView) {
        self.pages.push(page);
        if self.pages.len() > 1 {
            self.page += 1;
        }
    }

    // Get a mutable reference to the currently loaded view.
    fn mut_view(&mut self) -> Option<&mut PageView> {
        if self.pages.len() > 0 && self.page < self.pages.len() {
            Some(self.pages.get_mut(self.page).unwrap())
        } else {
            None
        }
    }

    fn respond_to_user(&mut self) {
        self.process_input();
    }

    fn process_input(&mut self) -> Action {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
        let mut page = self.mut_view().expect("expected page to be loaded");

        for c in stdin.keys() {
            let key = c.expect("UI error on stdin.keys");
            match page.process_input(key) {
                Action::Unknown => match key {
                    Key::Ctrl('q') => return Action::Quit,
                    Key::Up | Key::Ctrl('p') => return Action::Up,
                    Key::Down | Key::Ctrl('n') => return Action::Down,
                    Key::Left => return Action::Back,
                    Key::Right => return Action::Forward,
                    _ => {}
                },
                action => return action,
            }
        }
        Action::None
    }
}
