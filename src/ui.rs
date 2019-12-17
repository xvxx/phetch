use std::io;
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use gopher;
use gopher::Type;
use menu::{Menu, MenuView};

pub type Key = termion::event::Key;
pub type Error = io::Error;

pub struct UI {
    pages: Vec<Box<dyn View>>,
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
    Quit,
    Unknown,
    FollowLink(usize),
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
        print!("{}", self.render());
        // print!("{:#?}", self);
    }

    pub fn render(&self) -> String {
        // let (cols, rows) = termion::terminal_size().expect("can't get terminal size");
        String::new()
    }

    pub fn load(&mut self, url: String) {
        let (typ, host, port, sel) = gopher::parse_url(&url);
        let response = gopher::fetch(host, port, sel)
            .map_err(|e| {
                eprintln!("\x1B[91merror loading \x1b[93m{}: \x1B[0m{}", url, e);
                std::process::exit(1);
            })
            .unwrap();

        match typ {
            Type::Menu => self.add_view(MenuView::from(url, response)),
            // Type::Text => self.add_view(TextView::from(url, response)),
            _ => panic!("unknown type"),
        }
    }

    fn add_view<T: View + 'static>(&mut self, view: T) {
        self.pages.push(Box::from(view));
        if self.pages.len() > 1 {
            self.page += 1;
        }
    }

    fn respond_to_user(&mut self) {
        match self.process_input() {
            Action::Quit => std::process::exit(1),
            _ => {}
        }
    }

    fn process_input(&mut self) -> Action {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
        let page = self.pages.get_mut(self.page).expect("expected Page"); // TODO

        for c in stdin.keys() {
            let key = c.expect("UI error on stdin.keys"); // TODO
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
