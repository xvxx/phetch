mod action;
mod view;
pub use self::action::Action;
pub use self::view::View;

use std::io;
use std::io::{stdin, stdout, Write};
use std::process;
use std::process::Stdio;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::color;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use gopher;
use gopher::io_error;
use gopher::Type;
use help;
use menu::Menu;
use text::Text;

pub type Key = termion::event::Key;
pub type Page = Box<dyn View>;

pub const SCROLL_LINES: usize = 15;
pub const MAX_COLS: usize = 72;

pub struct UI {
    views: Vec<Page>,         // loaded views
    focused: usize,           // currently focused view
    dirty: bool,              // redraw?
    running: bool,            // main ui loop running?
    pub size: (usize, usize), // cols, rows
    error: String,            // error string, if any
}

impl UI {
    pub fn new() -> UI {
        let mut size = (0, 0);
        if let Ok((cols, rows)) = terminal_size() {
            size = (cols as usize, rows as usize);
        }
        UI {
            views: vec![],
            focused: 0,
            dirty: true,
            running: true,
            size,
            error: String::new(),
        }
    }

    pub fn run(&mut self) {
        self.startup();
        while self.running {
            self.draw();
            self.update();
        }
        self.shutdown();
    }

    pub fn draw(&mut self) {
        if self.dirty {
            let error = if self.error.is_empty() {
                "".into()
            } else {
                let e = format!(
                    "{}{}{}{}{}",
                    color::Fg(color::LightRed),
                    termion::cursor::Goto(1, self.rows()),
                    termion::clear::CurrentLine,
                    self.error,
                    color::Fg(color::Reset)
                );
                self.error.clear();
                e
            };
            print!(
                "{}{}{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
                self.render(),
                error
            );

            self.dirty = false;
        }
    }

    pub fn update(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        let action = self.process_page_input();
        if let Err(e) = self.process_action(action) {
            self.error = e.to_string();
        }
    }

    pub fn open(&mut self, url: &str) -> io::Result<()> {
        // no open loops
        if let Some(page) = self.views.get(self.focused) {
            if &page.url() == url {
                return Ok(());
            }
        }

        // non-gopher URL
        if url.contains("://") && !url.starts_with("gopher://") {
            return open_external(url);
        }

        self.fetch(url).and_then(|page| {
            self.add_page(page);
            Ok(())
        })
    }

    fn fetch(&mut self, url: &str) -> io::Result<Page> {
        // on-line help
        if url.starts_with("gopher://help/") {
            return self.fetch_help(url);
        }

        // request thread
        let thread_url = url.to_string();
        let req = thread::spawn(move || match gopher::fetch_url(&thread_url) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        });

        // spinner thead
        let (spintx, spinrx) = mpsc::channel();
        thread::spawn(move || loop {
            for i in 0..=3 {
                if let Ok(_) = spinrx.try_recv() {
                    return;
                }
                print!(
                    "\r{}{}{}{}",
                    termion::cursor::Hide,
                    color::Fg(color::LightBlack),
                    ".".repeat(i),
                    termion::clear::AfterCursor
                );
                stdout().flush();
                thread::sleep(Duration::from_millis(350));
            }
        });

        let work = req.join();
        spintx.send(true); // stop spinner
        self.dirty = true;
        let res = match work {
            Ok(opt) => match opt {
                Ok(body) => body,
                Err(e) => return Err(e),
            },
            Err(_) => return Err(io_error("Connection error".into())),
        };

        let (typ, _, _, _) = gopher::parse_url(url);
        match typ {
            Type::Menu | Type::Search => Ok(Box::new(Menu::from(url.to_string(), res))),
            Type::Text | Type::HTML => Ok(Box::new(Text::from(url.to_string(), res))),
            _ => Err(io_error(format!("Unsupported Gopher Response: {:?}", typ))),
        }
    }

    // get Menu for on-line help url, ex: gopher://help/1/types
    fn fetch_help(&mut self, url: &str) -> io::Result<Page> {
        if let Some(source) = help::lookup(
            &url.trim_start_matches("gopher://help/")
                .trim_start_matches("1/"),
        ) {
            Ok(Box::new(Menu::from(url.to_string(), source.to_string())))
        } else {
            Err(gopher::io_error(format!("Help file not found: {}", url)))
        }
    }

    pub fn render(&mut self) -> String {
        if let Ok((cols, rows)) = terminal_size() {
            self.term_size(cols as usize, rows as usize);
            if !self.views.is_empty() && self.focused < self.views.len() {
                if let Some(page) = self.views.get_mut(self.focused) {
                    page.term_size(cols as usize, rows as usize);
                    return page.render();
                }
            }
            String::from("No content to display.")
        } else {
            format!(
                "Error getting terminal size. Please file a bug: {}",
                "https://github.com/dvkt/phetch/issues/new"
            )
        }
    }

    fn rows(&self) -> u16 {
        self.size.1 as u16
    }

    fn startup(&mut self) {
        self.load_history();
    }

    fn shutdown(&self) {
        self.save_history();
    }

    fn config_dir_path(&self) -> Option<std::path::PathBuf> {
        let homevar = std::env::var("HOME");
        if homevar.is_err() {
            return None;
        }

        let dotdir = "~/.config/phetch".replace('~', &homevar.unwrap());
        let dotdir = std::path::Path::new(&dotdir);
        if dotdir.exists() {
            Some(std::path::PathBuf::from(dotdir))
        } else {
            None
        }
    }

    fn load_history(&mut self) {
        // let dotdir = self.config_dir_path();
        // if dotdir.is_none() {
        //     return;
        // }
        // let history = dotdir.unwrap().join("history");
        // if let Ok(file) = std::fs::OpenOptions::new().read(true).open(history) {
        //     let buffered = BufReader::new(file);
        //     let mut lines = buffered.lines();
        //     while let Some(Ok(url)) = lines.next() {}
        // }
    }

    fn save_history(&self) {
        let dotdir = self.config_dir_path();
        if dotdir.is_none() {
            return;
        }
        let dotdir = dotdir.unwrap();
        let mut out = String::new();
        for page in &self.views {
            let url = page.url();
            if url.starts_with("gopher://help/") {
                continue;
            }
            out.push_str(&page.url());
            out.push('\n');
        }
        let history = dotdir.join("history");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(history)
        {
            file.write_all(out.as_ref());
        }
    }

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    fn add_page(&mut self, page: Page) {
        self.dirty = true;
        if !self.views.is_empty() && self.focused < self.views.len() - 1 {
            self.views.truncate(self.focused + 1);
        }
        self.views.push(page);
        if self.views.len() > 1 {
            self.focused += 1;
        }
    }

    fn process_page_input(&mut self) -> Action {
        if let Some(page) = self.views.get_mut(self.focused) {
            if let Ok(key) = stdin()
                .keys()
                .nth(0)
                .ok_or(Action::Error("stdin.keys() error".to_string()))
            {
                if let Ok(key) = key {
                    return page.respond(key);
                }
            }
        }

        Action::None
    }

    fn process_action(&mut self, action: Action) -> io::Result<()> {
        match action {
            Action::Quit | Action::Keypress(Key::Ctrl('q')) | Action::Keypress(Key::Ctrl('c')) => {
                self.running = false
            }
            Action::Error(e) => return Err(io_error(e)),
            Action::Redraw => self.dirty = true,
            Action::Open(url) => self.open(&url)?,
            Action::Keypress(Key::Left) | Action::Keypress(Key::Backspace) => {
                if self.focused > 0 {
                    self.dirty = true;
                    self.focused -= 1;
                }
            }
            Action::Keypress(Key::Right) => {
                if self.focused < self.views.len() - 1 {
                    self.dirty = true;
                    self.focused += 1;
                }
            }
            Action::Keypress(Key::Ctrl('r')) => {
                if let Some(page) = self.views.get(self.focused) {
                    let url = page.url().to_string();
                    let raw = page.raw().to_string();
                    self.add_page(Box::new(Text::from(url, raw)));
                }
            }
            Action::Keypress(Key::Ctrl('g')) => {
                if let Some(url) = prompt("Go to URL: ") {
                    if !url.contains("://") && !url.starts_with("gopher://") {
                        self.open(&format!("gopher://{}", url))?;
                    } else {
                        self.open(&url)?;
                    }
                }
            }
            Action::Keypress(Key::Ctrl('h')) => self.open("gopher://help/")?,
            Action::Keypress(Key::Ctrl('u')) => {
                if let Some(page) = self.views.get(self.focused) {
                    status(&format!("Current URL: {}", page.url()));
                }
            }
            Action::Keypress(Key::Ctrl('y')) => {
                if let Some(page) = self.views.get(self.focused) {
                    copy_to_clipboard(&page.url())?;
                    status(&format!("Copied {} to clipboard.", page.url()));
                }
            }
            _ => (),
        }
        Ok(())
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        print!("\x1b[?25h"); // show cursor
    }
}

fn copy_to_clipboard(data: &str) -> io::Result<()> {
    spawn_os_clipboard()
        .and_then(|mut child| {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(data.as_bytes())
        })
        .map_err(|e| io_error(format!("Clipboard error: {}", e)))
}

fn spawn_os_clipboard() -> io::Result<process::Child> {
    if cfg!(target_os = "macos") {
        process::Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
    } else {
        process::Command::new("xclip")
            .args(&["-sel", "clip"])
            .stdin(Stdio::piped())
            .spawn()
    }
}

// runs the `open` shell command
fn open_external(url: &str) -> io::Result<()> {
    process::Command::new("open")
        .arg(url)
        .output()
        .and_then(|_| Ok(()))
}

/// Prompt user for input and return what was entered, if anything.
pub fn prompt(prompt: &str) -> Option<String> {
    let (_cols, rows) = terminal_size().unwrap();

    print!(
        "{}{}{}{}{}",
        color::Fg(color::Reset),
        termion::cursor::Goto(1, rows),
        termion::clear::CurrentLine,
        prompt,
        termion::cursor::Show,
    );
    stdout().flush();

    let mut input = String::new();
    for k in stdin().keys() {
        if let Ok(key) = k {
            match key {
                Key::Char('\n') => {
                    print!("{}{}", termion::clear::CurrentLine, termion::cursor::Hide);
                    stdout().flush();
                    return Some(input);
                }
                Key::Char(c) => input.push(c),
                Key::Esc | Key::Ctrl('c') => {
                    if input.is_empty() {
                        print!("{}{}", termion::clear::CurrentLine, termion::cursor::Hide);
                        stdout().flush();
                        return None;
                    } else {
                        input.clear();
                    }
                }
                Key::Backspace | Key::Delete => {
                    input.pop();
                }
                _ => {}
            }
        } else {
            break;
        }

        print!(
            "{}{}{}{}",
            termion::cursor::Goto(1, rows),
            termion::clear::CurrentLine,
            prompt,
            input,
        );
        stdout().flush();
    }

    if !input.is_empty() {
        Some(input)
    } else {
        None
    }
}

// Display a status message to the user.
pub fn status(s: &str) {
    let (_cols, rows) = terminal_size().unwrap();
    print!(
        "{}{}{}{}",
        termion::cursor::Goto(1, rows),
        termion::clear::CurrentLine,
        s,
        color::Fg(color::Reset)
    );
    stdout().flush();
}
