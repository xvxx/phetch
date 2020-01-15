//! The UI is what drives the interactive phetch application: it
//! spawns threads to fetch Gopher pages and download binary files, it
//! manages the opened pages (Views), it asks the focused View to
//! respond to user input, and it performs actions based on what the
//! View returns - like opening a telnet client, or displaying an
//! error on the status line.
//!
//! The UI also directly responds to user input on its own, such as
//! ctrl-q to quit the app or keyboard entry during an input prompt.
//!
//! Finally, the UI is what prints to the screen - each View just
//! renders its content to a String. The UI is what draws it.

mod action;
mod mode;
mod view;
pub use self::action::Action;
pub use self::mode::Mode;
pub use self::view::View;

use crate::{
    bookmarks, color,
    config::Config,
    gopher::{self, Type},
    help, history,
    menu::Menu,
    text::Text,
    utils, BUG_URL,
};
use std::{
    cell::RefCell,
    io::{stdin, stdout, Result, Stdout, Write},
    process::{self, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};
use termion::{
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

/// Alias for a termion Key event.
pub type Key = termion::event::Key;

/// Alias for either a Menu or Text View.
pub type Page = Box<dyn View>;

/// How many lines to jump by when using page up/down.
pub const SCROLL_LINES: usize = 15;
/// How big the longest line can be, for the purposes of calculating
/// margin sizes. We often draw longer lines than this and allow
/// wrapping in text views.
pub const MAX_COLS: usize = 77;

/// UI is mainly concerned with drawing to the screen, managing the
/// active Views/pages, and responding to user input.
pub struct UI {
    /// Current loaded Gopher views. Menu or Text
    views: Vec<Page>,
    /// Index of currently focused View
    focused: usize,
    /// Does the UI need to be entirely redrawn?
    dirty: bool,
    /// Is the UI running?
    running: bool,
    /// Size of screen (cols, rows)
    pub size: (usize, usize),
    /// Status message to display on screen, if any
    status: String,
    /// User config. Command line options + phetch.conf
    config: Config,
    out: RefCell<RawTerminal<Stdout>>,
}

impl UI {
    /// Create a new phetch application from a user provided config.
    pub fn new(config: Config) -> UI {
        let mut size = (0, 0);
        if let Ok((cols, rows)) = terminal_size() {
            size = (cols as usize, rows as usize);
        };

        // Store raw terminal but don't enable it yet or switch the
        // screen. We don't want to stare at a fully blank screen
        // while waiting for a slow page to load.
        let out = stdout()
            .into_raw_mode()
            .expect("Failed to initialize raw mode.");
        out.suspend_raw_mode();

        UI {
            views: vec![],
            focused: 0,
            dirty: true,
            running: true,
            size,
            config,
            status: String::new(),
            out: RefCell::new(out),
        }
    }

    /// Prepare stdout for writing. Should be used in interactive
    /// mode, eg inside run()
    pub fn startup(&mut self) {
        let mut out = self.out.borrow_mut();
        out.activate_raw_mode();
        write!(out, "{}", termion::screen::ToAlternateScreen);
    }

    /// Clean up after ourselves. Should only be used after running in
    /// interactive mode.
    pub fn shutdown(&mut self) {
        let mut out = self.out.borrow_mut();
        write!(out, "{}", termion::screen::ToMainScreen);
    }

    /// Main loop.
    pub fn run(&mut self) -> Result<()> {
        self.startup();
        while self.running {
            self.draw()?;
            self.update();
        }
        self.shutdown();
        Ok(())
    }

    /// Print the current view to the screen in rendered form.
    pub fn draw(&mut self) -> Result<()> {
        let status = self.render_status();
        if self.dirty {
            let screen = self.render()?;
            let mut out = self.out.borrow_mut();
            write!(
                out,
                "{}{}{}{}",
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide,
                screen,
                status,
            );
            out.flush();
            self.dirty = false;
        } else {
            let mut out = self.out.borrow_mut();
            out.write_all(status.as_ref());
            out.flush();
        }
        Ok(())
    }

    /// Accept user input and update data.
    pub fn update(&mut self) {
        let action = self.process_page_input();
        if !action.is_none() {
            self.status.clear();
        }
        if let Err(e) = self.process_action(action) {
            self.set_status(&format!("{}{}{}", color::Red, e, termion::cursor::Hide));
        }
    }

    /// Open a URL - Gopher, internal, telnet, or something else.
    pub fn open(&mut self, title: &str, url: &str) -> Result<()> {
        // no open loops
        if let Some(page) = self.views.get(self.focused) {
            if page.url() == url {
                return Ok(());
            }
        }

        // telnet
        if url.starts_with("telnet://") {
            return self.telnet(url);
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

    /// Download a binary file. Used by `open()` internally.
    fn download(&mut self, url: &str) -> Result<()> {
        let url = url.to_string();
        let (tls, tor) = (self.config.tls, self.config.tor);
        self.spinner(&format!("Downloading {}", url), move || {
            gopher::download_url(&url, tls, tor)
        })
        .and_then(|res| res)
        .and_then(|(path, bytes)| {
            self.set_status(
                format!(
                    "Download complete! {} saved to {}",
                    utils::human_bytes(bytes),
                    path
                )
                .as_ref(),
            );
            Ok(())
        })
    }

    /// Fetches a URL and returns a View for its content.
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
        let (tls, tor) = (self.config.tls, self.config.tor);
        // don't spin on first ever request
        let (tls, res) = if self.views.is_empty() {
            gopher::fetch_url(&thread_url, tls, tor)?
        } else {
            self.spinner("", move || gopher::fetch_url(&thread_url, tls, tor))??
        };
        let (typ, _, _, _) = gopher::parse_url(&url);
        match typ {
            Type::Menu | Type::Search => Ok(Box::new(Menu::from(url, &res, tls, tor))),
            Type::Text | Type::HTML => Ok(Box::new(Text::from(url, &res, tls, tor))),
            _ => Err(error!("Unsupported Gopher Response: {:?}", typ)),
        }
    }

    /// Get Menu for on-line help, home page, etc, ex: gopher://home/1/help/types
    fn fetch_internal(&mut self, url: &str) -> Result<Page> {
        if let Some(source) = help::lookup(
            &url.trim_start_matches("gopher://phetch/")
                .trim_start_matches("1/"),
        ) {
            Ok(Box::new(Menu::from(url, &source, false, false)))
        } else {
            Err(error!("phetch URL not found: {}", url))
        }
    }

    /// # of visible columns
    fn cols(&self) -> u16 {
        self.size.0 as u16
    }

    /// # of visible row
    fn rows(&self) -> u16 {
        self.size.1 as u16
    }

    /// Set the current columns and rows.
    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    /// Show a spinner while running a thread. Used to make gopher requests or
    /// download files.
    fn spinner<T: Send + 'static, F: 'static + Send + FnOnce() -> T>(
        &mut self,
        label: &str,
        work: F,
    ) -> Result<T> {
        let req = thread::spawn(work);

        let (tx, rx) = mpsc::channel();
        let label = label.to_string();
        let rows = self.rows() as u16;
        thread::spawn(move || loop {
            for i in 0..=3 {
                if rx.try_recv().is_ok() {
                    return;
                }
                print!(
                    "{}{}{}{}{}{}{}",
                    termion::cursor::Goto(1, rows),
                    termion::cursor::Hide,
                    label,
                    ".".repeat(i),
                    termion::clear::UntilNewline,
                    color::Reset,
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

    /// Create a rendered String for the current View in its current state.
    pub fn render(&mut self) -> Result<String> {
        // TODO: only get size on SIGWINCH
        if let Ok((cols, rows)) = terminal_size() {
            self.term_size(cols as usize, rows as usize);
            if !self.views.is_empty() && self.focused < self.views.len() {
                if let Some(page) = self.views.get_mut(self.focused) {
                    page.term_size(cols as usize, rows as usize);
                    return Ok(page.render(&self.config));
                }
            }
            Err(error!(
                "fatal: No focused View. Please file a bug: {}",
                BUG_URL
            ))
        } else {
            Err(error!(
                "fatal: Can't get terminal size. Please file a bug: {}",
                BUG_URL
            ))
        }
    }

    /// Set the status line's content.
    fn set_status(&mut self, status: &str) {
        self.status = status.replace('\n', "\\n").replace('\r', "\\r");
    }

    /// Render the connection status (TLS or Tor).
    fn render_conn_status(&self) -> Option<String> {
        let page = self.views.get(self.focused)?;
        if page.is_tls() {
            let status = color!("TLS", Black, GreenBG);
            return Some(format!(
                "{}{}",
                termion::cursor::Goto(self.cols() - 3, self.rows()),
                if self.config.emoji { "🔐" } else { &status },
            ));
        } else if page.is_tor() {
            let status = color!("TOR", Bold, White, MagentaBG);
            return Some(format!(
                "{}{}",
                termion::cursor::Goto(self.cols() - 3, self.rows()),
                if self.config.emoji { "🧅" } else { &status },
            ));
        }
        None
    }

    /// Render the status line.
    fn render_status(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, self.rows()),
            termion::clear::CurrentLine,
            self.status,
            self.render_conn_status().unwrap_or_else(|| "".into()),
            color::Reset,
        )
    }

    /// Add a View to the app's currently opened Views.
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

    /// Ask user to confirm action with ENTER or Y.
    fn confirm(&self, question: &str) -> bool {
        let rows = self.rows();

        let mut out = self.out.borrow_mut();
        write!(
            out,
            "{}{}{}{} [Y/n]: {}",
            color::Reset,
            termion::cursor::Goto(1, rows),
            termion::clear::CurrentLine,
            question,
            termion::cursor::Show,
        );
        out.flush();

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

    /// Prompt user for input and return what was entered, if anything.
    fn prompt(&self, prompt: &str, value: &str) -> Option<String> {
        let rows = self.rows();
        let mut input = value.to_string();

        let mut out = self.out.borrow_mut();
        write!(
            out,
            "{}{}{}{}{}{}",
            color::Reset,
            termion::cursor::Goto(1, rows),
            termion::clear::CurrentLine,
            prompt,
            input,
            termion::cursor::Show,
        );
        out.flush();

        for k in stdin().keys() {
            if let Ok(key) = k {
                match key {
                    Key::Char('\n') => {
                        write!(
                            out,
                            "{}{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Hide
                        );
                        out.flush();
                        return Some(input);
                    }
                    Key::Char(c) => input.push(c),
                    Key::Esc | Key::Ctrl('c') => {
                        write!(
                            out,
                            "{}{}",
                            termion::clear::CurrentLine,
                            termion::cursor::Hide
                        );
                        out.flush();
                        return None;
                    }
                    Key::Backspace | Key::Delete => {
                        input.pop();
                    }
                    _ => {}
                }
            } else {
                break;
            }

            write!(
                out,
                "{}{}{}{}",
                termion::cursor::Goto(1, rows),
                termion::clear::CurrentLine,
                prompt,
                input,
            );
            out.flush();
        }

        if !input.is_empty() {
            Some(input)
        } else {
            None
        }
    }

    /// Opens an interactive telnet session.
    fn telnet(&mut self, url: &str) -> Result<()> {
        let (_, host, port, _) = gopher::parse_url(url);
        let out = self.out.borrow_mut();
        out.suspend_raw_mode();
        let mut cmd = process::Command::new("telnet")
            .arg(host)
            .arg(port)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;
        cmd.wait();
        out.activate_raw_mode();
        self.dirty = true; // redraw when finished with session
        Ok(())
    }

    /// Asks the current View to process user input and produce an Action.
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

        Action::Error("No Gopher page loaded.".into())
    }

    /// Ctrl-Z: Suspend Unix process w/ SIGTSTP.
    fn suspend(&mut self) {
        let mut out = self.out.borrow_mut();
        write!(out, "{}", termion::screen::ToMainScreen);
        out.flush();
        unsafe { libc::raise(libc::SIGTSTP) };
        write!(out, "{}", termion::screen::ToAlternateScreen);
        out.flush();
        self.dirty = true;
    }

    /// Given an Action from a View in response to user input, do the
    /// action.
    fn process_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::List(actions) => {
                for action in actions {
                    self.process_action(action);
                }
            }
            Action::Keypress(Key::Ctrl('c')) => {
                self.status = "\x1b[90m(Use q to quit)\x1b[0m".into()
            }
            Action::Keypress(Key::Ctrl('z')) => self.suspend(),
            Action::Keypress(Key::Esc) => {}
            Action::Error(e) => return Err(error!(e)),
            Action::Redraw => self.dirty = true,
            Action::Draw(s) => {
                let mut out = self.out.borrow_mut();
                out.write_all(s.as_ref());
                out.flush();
            }
            Action::Status(s) => self.set_status(&s),
            Action::Open(title, url) => self.open(&title, &url)?,
            Action::Prompt(query, fun) => {
                if let Some(response) = self.prompt(&query, "") {
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
                    if let Some(url) = self.prompt("Go to URL: ", "") {
                        self.open(&url, &url)?;
                    }
                }
                'h' => self.open("Help", "gopher://phetch/1/help")?,
                'r' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        let raw = page.raw();
                        let mut text = Text::from(url, raw, page.is_tls(), page.is_tor());
                        text.wide = true;
                        self.add_page(Box::new(text));
                    }
                }
                's' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        match bookmarks::save(&url, &url) {
                            Ok(()) => {
                                let msg = format!("Saved bookmark: {}", url);
                                self.set_status(&msg);
                            }
                            Err(e) => return Err(error!("Save failed: {}", e)),
                        }
                    }
                }
                'u' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let current_url = page.url();
                        if let Some(url) = self.prompt("Current URL: ", &current_url) {
                            if url != current_url {
                                self.open(&url, &url);
                            }
                        }
                    }
                }
                'y' => {
                    if let Some(page) = self.views.get(self.focused) {
                        let url = page.url();
                        copy_to_clipboard(&url)?;
                        let msg = format!("Copied {} to clipboard.", url);
                        self.set_status(&msg);
                    }
                }
                'w' => {
                    self.config.wide = !self.config.wide;
                    self.dirty = true;
                }
                'q' => self.running = false,
                c => return Err(error!("Unknown keypress: {}", c)),
            },
            _ => (),
        }
        Ok(())
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        let mut out = self.out.borrow_mut();
        write!(out, "{}{}", color::Reset, termion::cursor::Show,);
        out.flush();
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
