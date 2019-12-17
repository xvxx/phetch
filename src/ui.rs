use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use gopher;
use gopher::Type;
use menu::MenuView;

pub type Key = termion::event::Key;

pub struct UI {
    pages: Vec<Box<dyn View>>,
    page: usize,
    dirty: bool, // redraw?
}

#[derive(Debug)]
pub enum Action {
    None,         // do nothing
    Back,         // back in history
    Forward,      // also history
    Open(String), // url
    Input,        // redraw the input bar
    Quit,         // yup
    Unknown,      // handler doesn't know what to do
}

pub trait View {
    fn process_input(&mut self, c: Key) -> Action;
    fn render(&self) -> String;
}

impl UI {
    pub fn new() -> UI {
        UI {
            pages: vec![],
            page: 0,
            dirty: true,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw();
            self.update();
        }
    }

    pub fn draw(&mut self) {
        if self.dirty {
            print!("\x1b[2J\x1b[H{}", self.render());
            self.dirty = false;
        }
    }

    pub fn update(&mut self) {
        match self.process_input() {
            Action::Quit => std::process::exit(1),
            _ => {}
        }
    }

    pub fn render(&self) -> String {
        // let (cols, rows) = termion::terminal_size().expect("can't get terminal size");
        if self.pages.len() > 0 && self.page < self.pages.len() {
            if let Some(page) = self.pages.get(self.page) {
                return page.render();
            }
        }
        String::from("N/A")
    }

    pub fn open(&mut self, url: &str) {
        self.dirty = true;
        let (typ, host, port, sel) = gopher::parse_url(url);
        let response = gopher::fetch(host, port, sel)
            .map_err(|e| {
                eprintln!("\x1B[91merror loading \x1b[93m{}: \x1B[0m{}", url, e);
                std::process::exit(1);
            })
            .unwrap();

        match typ {
            Type::Menu => self.add_page(MenuView::from(url.to_string(), response)),
            // Type::Text => self.add_page(TextView::from(url, response)),
            _ => panic!("unknown type: {:?}", typ),
        }
    }

    fn add_page<T: View + 'static>(&mut self, view: T) {
        if self.pages.len() > 0 && self.page < self.pages.len() - 1 {
            self.pages.truncate(self.page + 1);
        }
        self.pages.push(Box::from(view));
        if self.pages.len() > 1 {
            self.page += 1;
        }
    }

    fn process_input(&mut self) -> Action {
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        match self.process_page_input() {
            Action::Open(url) => {
                self.open(&url);
                Action::None
            }
            Action::Back => {
                if self.page > 0 {
                    self.dirty = true;
                    self.page -= 1;
                }
                Action::None
            }
            Action::Forward => {
                if self.page < self.pages.len() - 1 {
                    self.dirty = true;
                    self.page += 1;
                }
                Action::None
            }
            a => a,
        }
    }

    fn process_page_input(&mut self) -> Action {
        let stdin = stdin();
        let page = self.pages.get_mut(self.page).expect("expected Page"); // TODO

        for c in stdin.keys() {
            let key = c.expect("UI error on stdin.keys"); // TODO
            match page.process_input(key) {
                Action::Unknown => match key {
                    Key::Ctrl('q') | Key::Ctrl('c') => return Action::Quit,
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
