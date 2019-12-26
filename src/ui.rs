mod action;
mod view;
pub use self::action::Action;
pub use self::view::View;

use crate::{
    bookmarks,
    gopher::{self, Type},
    help, history,
    menu::Menu,
    text::Text,
    utils,
};
use std::{
    io::{stdin, stdout, Result, Write},
    process,
    process::Stdio,
    sync::mpsc,
    thread,
    time::Duration,
};
use termion::{color, input::TermRead, raw::IntoRawMode, terminal_size};

pub type Key = termion::event::Key;
pub type Page = Box<dyn View>;

pub const SCROLL_LINES: usize = 15;
pub const MAX_COLS: usize = 77;

pub struct UI {
    views: Vec<Page>,         // loaded views
    focused: usize,           // currently focused view
    dirty: bool,              // redraw?
    running: bool,            // main ui loop running?
    pub size: (usize, usize), // cols, rows
    status: String,           // status message, if any
}

impl UI {
    pub fn new() -> UI {
        UI {
            views: vec![],
            focused: 0,
            dirty: true,
            running: true,
            size: (0, 0),
            status: String::new(),
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
            print!(
                "{}{}{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
                self.render(),
                self.render_status().unwrap_or_else(|| "".into()),
            );

            self.dirty = false;
        }
    }

    pub fn update(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();

        let action = self.process_page_input();
        if let Err(e) = self.process_action(action) {
            self.set_status(format!("{}{}", color::Fg(color::LightRed), e));
        }
    }

    pub fn open(&mut self, title: &str, url: &str) -> Result<()> {
        // no open loops
        if let Some(page) = self.views.get(self.focused) {
            if page.url() == url {
                return Ok(());
            }
        }

        // non-gopher URL
        if url.contains("://") && !url.starts_with("gopher://") {
            self.dirty = true;
            return if self.confirm(&format!("Open external URL? {}", url)) {
                open_external(url)
            } else {
                Ok(())
            };
        }

        // binary downloads
        let (typ, _, _, _) = gopher::parse_url(url);
        if typ.is_download() {
            self.dirty = true;
            return if self.confirm(&format!("Download {}?", url)) {
                self.download(url)
            } else {
                Ok(())
            };
        }

        self.fetch(title, url).and_then(|page| {
            self.add_page(page);
            Ok(())
        })
    }

    fn download(&mut self, url: &str) -> Result<()> {
        let url = url.to_string();
        self.spinner(&format!("Downloading {}", url), move || {
            gopher::download_url(&url)
        })
        .and_then(|res| res)
        .and_then(|(path, bytes)| {
            self.set_status(format!(
                "Download complete! {} saved to {}",
                utils::human_bytes(bytes),
                path
            ));
            Ok(())
        })
    }

    fn fetch(&mut self, title: &str, url: &str) -> Result<Page> {
        // on-line help
        if url.starts_with("gopher://phetch/") {
            return self.fetch_internal(url);
        }
        // record history urls
        let hurl = url.to_string();
        let hname = title.to_string();
        thread::spawn(move || history::save(&hname, &hurl));
        // request thread
        let thread_url = url.to_string();
        let res = self.spinner("", move || gopher::fetch_url(&thread_url))??;
        let (typ, _, _, _) = gopher::parse_url(&url);
        match typ {
            Type::Menu | Type::Search => Ok(Box::new(Menu::from(url.to_string(), res))),
            Type::Text | Type::HTML => Ok(Box::new(Text::from(url.to_string(), res))),
            _ => Err(error!("Unsupported Gopher Response: {:?}", typ)),
        }
    }

    // get Menu for on-line help, home page, etc, ex: gopher://home/1/help/types
    fn fetch_internal(&mut self, url: &str) -> Result<Page> {
        if let Some(source) = help::lookup(
            &url.trim_start_matches("gopher://phetch/")
                .trim_start_matches("1/"),
        ) {
            Ok(Box::new(Menu::from(url.to_string(), source)))
        } else {
            Err(error!("phetch URL not found: {}", url))
        }
    }

    fn rows(&self) -> u16 {
        self.size.1 as u16
    }

    fn startup(&mut self) {}

    fn shutdown(&self) {}

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    // Show a spinner while running a thread. Used to make gopher requests or
    // download files.
    fn spinner<T: Send + 'static, F: 'static + Send + FnOnce() -> T>(
        &mut self,
        label: &str,
        work: F,
    ) -> Result<T> {
        let req = thread::spawn(work);

        let (tx, rx) = mpsc::channel();
        let label = label.to_string();
        thread::spawn(move || loop {
            for i in 0..=3 {
                if rx.try_recv().is_ok() {
                    return;
                }
                print!(
                    "\r{}{}{}{}{}{}",
                    termion::cursor::Hide,
                    label,
                    ".".repeat(i),
                    termion::clear::AfterCursor,
                    color::Fg(color::Reset),
                    termion::cursor::Show,
                );
                stdout().flush();
                thread::sleep(Duration::from_millis(500));
            }
        });

        let result = req.join();
        tx.send(true); // stop spinner
        self.dirty = true;
        result.map_err(|e| error!("Spinner error: {:?}", e))
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

    fn set_status(&mut self, status: String) {
        self.status = status;
        self.dirty = true;
    }

    fn render_status(&self) -> Option<String> {
        if self.status.is_empty() {
            None
        } else {
            Some(format!(
                "{}{}{}{}",
                termion::cursor::Goto(1, self.rows()),
                termion::clear::CurrentLine,
                self.status,
                color::Fg(color::Reset)
            ))
        }
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

    // Ask user to confirm action with ENTER or Y.
    fn confirm(&self, question: &str) -> bool {
        let rows = self.rows();

        print!(
            "{}{}{}{} [Y/n]: {}",
            color::Fg(color::Reset),
            termion::cursor::Goto(1, rows),
            termion::clear::CurrentLine,
            question,
            termion::cursor::Show,
        );
        stdout().flush();

        if let Some(Ok(key)) = stdin().keys().next() {
            match key {
                Key::Char('\n') => true,
                Key::Char('y') | Key::Char('Y') => true,
                _ => false,
            }
        } else {
            false
        }
    }

    // Prompt user for input and return what was entered, if anything.
    fn prompt(&self, prompt: &str) -> Option<String> {
        let rows = self.rows();

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

    fn process_page_input(&mut self) -> Action {
        if let Some(page) = self.views.get_mut(self.focused) {
            if let Ok(key) = stdin()
                .keys()
                .nth(0)
                .ok_or_else(|| Action::Error("stdin.keys() error".to_string()))
            {
                if let Ok(key) = key {
                    return page.respond(key);
                }
            }
        }

        Action::None
    }

    fn process_action(&mut self, action: Action) -> Result<()> {
        // track if the status line was cleared in this update cycle
        let cleared = if !self.status.is_empty() {
            self.status.clear();
            self.dirty = true;
            true
        } else {
            false
        };

        match action {
            Action::Keypress(Key::Ctrl('c')) => {
                if !cleared {
                    self.running = false
                }
            }
            Action::Keypress(Key::Esc) => {}
            Action::Error(e) => return Err(error!(e)),
            Action::Redraw => self.dirty = true,
            Action::Open(title, url) => self.open(&title, &url)?,
            Action::Prompt(query, fun) => {
                if let Some(response) = self.prompt(&query) {
                    self.process_action(fun(response));
                }
            }
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
            Action::Keypress(Key::Char(key)) | Action::Keypress(Key::Ctrl(key)) => match key {
                'a' => self.open("History", "gopher://phetch/1/history")?,
                'b' => self.open("Bookmarks", "gopher://phetch/1/bookmarks")?,
                'g' => {
                    if let Some(url) = self.prompt("Go to URL: ") {
                        if !url.contains("://") && !url.starts_with("gopher://") {
                            self.open(&url, &format!("gopher://{}", url))?;
                        } else {
                            self.open(&url, &url)?;
                        }
                    }
                }
                'h' => self.open("Help", "gopher://phetch/1/help")?,
                'r' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        let raw = page.raw();
                        let mut text = Text::from(url, raw);
                        text.wide = true;
                        self.add_page(Box::new(text));
                    }
                }
                's' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        match bookmarks::save(&url, &url) {
                            Ok(()) => self.set_status(format!("Saved bookmark: {}", url)),
                            Err(e) => return Err(error!("Save failed: {}", e)),
                        }
                    }
                }
                'u' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        self.set_status(format!("Current URL: {}", url));
                    }
                }
                'y' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        copy_to_clipboard(&url)?;
                        self.set_status(format!("Copied {} to clipboard.", url));
                    }
                }
                'q' => self.running = false,
                c => return Err(error!("Unknown keypress: {}", c)),
            },
            _ => (),
        }
        Ok(())
    }
}

impl Default for UI {
    fn default() -> Self {
        UI::new()
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        print!("\x1b[?25h"); // show cursor
        stdout().flush();
    }
}

fn copy_to_clipboard(data: &str) -> Result<()> {
    spawn_os_clipboard().and_then(|mut child| {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(data.as_bytes())
    })
}

fn spawn_os_clipboard() -> Result<process::Child> {
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
fn open_external(url: &str) -> Result<()> {
    let output = process::Command::new("open").arg(url).output()?;
    if output.stderr.is_empty() {
        Ok(())
    } else {
        Err(error!(
            "`open` error: {}",
            String::from_utf8(output.stderr)
                .unwrap_or_else(|_| "?".into())
                .trim_end()
        ))
    }
}
