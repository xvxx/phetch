use std::io;
use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use gopher;
use gopher::Type;
use menu::MenuView;
use text::TextView;

pub type Key = termion::event::Key;

pub const SCROLL_LINES: usize = 15;
pub const MAX_COLS: usize = 72;

pub struct UI {
    pages: Vec<Box<dyn View>>,
    page: usize,   // currently focused page
    dirty: bool,   // redraw?
    running: bool, // main ui loop running?
}

#[derive(Debug)]
pub enum Action {
    None,              // do nothing
    Back,              // back in history
    Forward,           // also history
    Open(String),      // url
    Redraw,            // redraw everything
    Quit,              // yup
    Clipboard(String), // copy to clipboard
    Error(String),     // error message
    Unknown,           // handler doesn't know what to do
}

pub trait View {
    fn process_input(&mut self, c: Key) -> Action;
    fn render(&self) -> String;
    fn url(&self) -> String;
    fn set_size(&mut self, cols: usize, rows: usize);
}

macro_rules! status {
    ($e:expr) => { status!("{}", $e); };
    ($e:expr, $($y:expr),*) => {{
        print!("\r{}\x1b[0m\x1b[K", format!($e, $($y),*));
        stdout().flush();
    }}
}

macro_rules! error {
    ($e:expr) => { error!("{}", $e); };
    ($e:expr, $($y:expr),*) => {{
        eprint!("\r\x1b[0;91m{}\x1b[0m\x1b[K", format!($e, $($y),*));
        stdout().flush();
    }}
}

impl UI {
    pub fn new() -> UI {
        UI {
            pages: vec![],
            page: 0,
            dirty: true,
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.draw();
            self.update();
        }
    }

    pub fn draw(&mut self) {
        if self.dirty {
            print!(
                "{}{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
                self.render()
            );
            self.dirty = false;
        }
    }

    pub fn update(&mut self) {
        match self.process_input() {
            Action::Quit => self.running = false,
            Action::Error(e) => error!(e),
            _ => {}
        }
    }

    pub fn render(&mut self) -> String {
        let (cols, rows) = termion::terminal_size().expect("can't get terminal size"); // TODO
        if !self.pages.is_empty() && self.page < self.pages.len() {
            if let Some(page) = self.pages.get_mut(self.page) {
                page.set_size(cols as usize, rows as usize);
                return page.render();
            }
        }
        String::from("N/A")
    }

    pub fn open(&mut self, url: &str) -> io::Result<()> {
        status!("\x1b[90mLoading...");
        let (typ, host, port, sel) = gopher::parse_url(url);
        gopher::fetch(host, port, sel)
            .and_then(|response| match typ {
                Type::Menu | Type::Search => {
                    Ok(self.add_page(MenuView::from(url.to_string(), response)))
                }
                Type::Text | Type::HTML => {
                    Ok(self.add_page(TextView::from(url.to_string(), response)))
                }
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unsupported Gopher Response: {:?}", typ),
                )),
            })
            .map_err(|e| io::Error::new(e.kind(), format!("Error loading {}: {}", url, e)))
    }

    fn add_page<T: View + 'static>(&mut self, view: T) {
        self.dirty = true;
        if !self.pages.is_empty() && self.page < self.pages.len() - 1 {
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
            Action::Redraw => {
                self.dirty = true;
                Action::None
            }
            Action::Open(url) => match self.open(&url) {
                Err(e) => Action::Error(e.to_string()),
                Ok(()) => Action::None,
            },
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
            Action::Clipboard(url) => {
                copy_to_clipboard(&url);
                Action::None
            }
            a => a,
        }
    }

    fn process_page_input(&mut self) -> Action {
        let stdin = stdin();
        let page_opt = self.pages.get_mut(self.page);
        if page_opt.is_none() {
            return Action::None;
        }

        let page = page_opt.unwrap();
        for c in stdin.keys() {
            let key = c.expect("UI error on stdin.keys"); // TODO
            match page.process_input(key) {
                Action::Unknown => match key {
                    Key::Ctrl('q') | Key::Ctrl('c') => return Action::Quit,
                    Key::Left | Key::Backspace => return Action::Back,
                    Key::Right => return Action::Forward,
                    Key::Char('\n') => return Action::Redraw,
                    Key::Ctrl('y') => return Action::Clipboard(page.url()),
                    _ => {}
                },
                action => return action,
            }
        }
        Action::None
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        print!("\x1b[?25h"); // show cursor
    }
}

fn copy_to_clipboard(data: &str) {
    let mut child = spawn_os_clipboard();
    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(data.as_bytes());
}

fn spawn_os_clipboard() -> std::process::Child {
    if cfg!(target_os = "macos") {
        Command::new("pbcopy").stdin(Stdio::piped()).spawn()
    } else {
        Command::new("xclip")
            .args(&["-sel", "clip"])
            .stdin(Stdio::piped())
            .spawn()
    }
    .unwrap() // TODO
}
