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
pub use self::{action::Action, mode::Mode, view::View};

use crate::{
    bookmarks,
    config::{Config, SharedConfig},
    encoding::Encoding,
    gopher::{self, Type},
    help, history,
    menu::Menu,
    terminal,
    text::Text,
    theme, utils, BUG_URL,
};
use std::{
    io::{stdin, stdout, Result, Write},
    process::{self, Stdio},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread,
    time::Duration,
};
use termion::{input::TermRead, terminal_size};

/// Alias for a termion Key event.
pub type Key = termion::event::Key;

/// Channel to receive Key events on.
pub type KeyReceiver = Arc<Mutex<Receiver<Key>>>;

/// How big the longest line can be, for the purposes of calculating
/// margin sizes. We often draw longer lines than this and allow
/// wrapping in text views.
pub const MAX_COLS: usize = 77;

/// Fatal errors. In general we want to try and catch any errors
/// (network, parsing gopher response, etc) and just show an error
/// message in the status bar, but if we can't write to STDOUT or
/// control the screen, we need to just crash.
const ERR_SCREEN: &str = "Fatal Error using Alternate Screen.";
const ERR_STDOUT: &str = "Fatal Error writing to STDOUT.";

lazy_static! {
    /// Channel to send SIGWINCH (resize) events on, once received.
    static ref RESIZE_SENDER: Arc<Mutex<Option<Sender<Key>>>> = Arc::new(Mutex::new(None));
}

/// Raw resize handler that is called when SIGWINCH is received.
fn resize_handler(_: i32) {
    if let Some(sender) = &*RESIZE_SENDER.lock().unwrap() {
        sender.send(Key::F(5)).unwrap();
    }
}

/// No-op INT handler that is called when SIGINT (ctrl-c) is
/// received in child processes (like `telnet`).
fn sigint_handler(_: i32) {}

/// Handler for when the application is resumed after ctrl-z.
fn sigcont_handler(_: i32) {
    terminal::enable_raw_mode().expect("Fatal Error entering Raw Mode.");
}

/// UI is mainly concerned with drawing to the screen, managing the
/// active views, and responding to user input.
pub struct UI {
    /// Current loaded Gopher views. Menu or Text
    views: Vec<Box<dyn View>>,
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
    config: SharedConfig,
    /// Channel where UI events are sent.
    keys: KeyReceiver,
}

impl UI {
    /// Create a new phetch application from a user provided config.
    pub fn new(config: Config) -> UI {
        let mut size = (0, 0);
        if let Ok((cols, rows)) = terminal_size() {
            size = (cols as usize, rows as usize);
        };

        UI {
            views: vec![],
            focused: 0,
            dirty: true,
            running: true,
            size,
            config: Arc::new(RwLock::new(config)),
            status: String::new(),
            keys: Self::spawn_keyboard_listener(),
        }
    }

    /// Main loop.
    pub fn run(&mut self) -> Result<()> {
        while self.running {
            self.draw()?;
            self.update();
        }
        Ok(())
    }

    /// Print the current view to the screen in rendered form.
    pub fn draw(&mut self) -> Result<()> {
        let status = self.render_status();
        let mut out = stdout();
        if self.dirty {
            let screen = self.render()?;
            write!(
                out,
                "{}{}{}{}",
                terminal::Goto(1, 1),
                terminal::HideCursor,
                screen,
                status,
            )?;
            out.flush()?;
            self.dirty = false;
        } else {
            out.write_all(status.as_ref())?;
            out.flush()?;
        }
        Ok(())
    }

    /// Accept user input and update data.
    pub fn update(&mut self) {
        let action = self.process_view_input();
        if !action.is_none() {
            self.status.clear();
        }
        if let Err(e) = self.process_action(action) {
            self.set_status(&format!(
                "{}{}{}",
                &self.config.read().unwrap().theme.item_error,
                e,
                terminal::HideCursor
            ));
        }
    }

    /// Reload the currently focused view while preserving history.
    pub fn reload(&mut self, title: &str, url: &str) -> Result<()> {
        let mut rest = if self.views.len() > self.focused + 1 {
            self.views.drain(self.focused..).collect()
        } else {
            vec![self.views.remove(self.views.len() - 1)]
        };
        if self.focused > 0 {
            self.focused -= 1;
        }
        self.open(title, url)?;
        if rest.len() > 1 {
            rest.remove(0); // drop the view we're reloading
            self.views.append(&mut rest);
        }
        Ok(())
    }

    /// Open a URL - Gopher, internal, telnet, or something else.
    pub fn open(&mut self, title: &str, url: &str) -> Result<()> {
        if let Some(view) = self.views.get(self.focused) {
            if view.url() == url {
                return self.reload(title, url);
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
                utils::open_external(url)
            } else {
                Ok(())
            };
        }

        // binary downloads
        let typ = gopher::type_for_url(url);

        if typ.is_media() && self.config.read().unwrap().media.is_some() {
            self.dirty = true;
            return if self.config.read().unwrap().autoplay
                || self.confirm(&format!("Open in media player? {}", url))
            {
                utils::open_media(self.config.read().unwrap().media.as_ref().unwrap(), url)
            } else {
                Ok(())
            };
        }

        if typ.is_download() {
            self.dirty = true;
            return if self.confirm(&format!("Download {}?", url)) {
                self.download(url)
            } else {
                Ok(())
            };
        }

        self.load(title, url).map(|view| {
            self.add_view(view);
        })
    }

    /// Used to download content of the current view with a provided filename
    fn download_file_with_filename(&mut self, url: &str, filename: String) -> Result<()> {
        let url = url.to_string();
        let (tls, tor) = (
            self.config.read().unwrap().tls,
            self.config.read().unwrap().tor,
        );
        let chan = self.keys.clone();
        self.spinner(&format!("Downloading {}", url), move || {
            gopher::download_url_with_filename(&url, tls, tor, chan, &filename)
        })
        .and_then(|res| res)
        .map(|(path, bytes)| {
            self.set_status(
                format!(
                    "Download complete! {} saved to {}",
                    utils::human_bytes(bytes),
                    path
                )
                .as_ref(),
            );
        })
    }


    /// Download a binary file. Used by `open()` internally.
    fn download(&mut self, url: &str) -> Result<()> {
        let url = url.to_string();
        let (tls, tor) = (
            self.config.read().unwrap().tls,
            self.config.read().unwrap().tor,
        );
        let chan = self.keys.clone();
        self.spinner(&format!("Downloading {}", url), move || {
            gopher::download_url(&url, tls, tor, chan)
        })
        .and_then(|res| res)
        .map(|(path, bytes)| {
            self.set_status(
                format!(
                    "Download complete! {} saved to {}",
                    utils::human_bytes(bytes),
                    path
                )
                .as_ref(),
            );
        })
    }

    /// Fetches a URL and returns a View for its content.
    fn load(&mut self, title: &str, url: &str) -> Result<Box<dyn View>> {
        // on-line help
        if url.starts_with("gopher://phetch/") {
            return self.load_internal(url);
        }
        // record history urls
        let hurl = url.to_string();
        let hname = title.to_string();
        thread::spawn(move || history::save(&hname, &hurl));
        // request thread
        let thread_url = url.to_string();
        let (tls, tor) = (
            self.config.read().unwrap().tls,
            self.config.read().unwrap().tor,
        );
        // don't spin on first ever request
        let (tls, res) = if self.views.is_empty() {
            gopher::fetch_url(&thread_url, tls, tor)?
        } else {
            self.spinner("", move || gopher::fetch_url(&thread_url, tls, tor))??
        };
        let typ = gopher::type_for_url(url);
        match typ {
            Type::Menu | Type::Search => Ok(Box::new(Menu::from(
                url,
                gopher::response_to_string(&res),
                self.config.clone(),
                tls,
            ))),
            Type::Text | Type::HTML => Ok(Box::new(Text::from(url, res, self.config.clone(), tls))),
            _ => Err(error!("Unsupported Gopher Response: {:?}", typ)),
        }
    }

    /// Get Menu for on-line help, home page, etc, ex: gopher://phetch/1/help/types
    fn load_internal(&mut self, url: &str) -> Result<Box<dyn View>> {
        if let Some(source) = help::lookup(
            url.trim_start_matches("gopher://phetch/")
                .trim_start_matches("1/"),
        ) {
            Ok(Box::new(Menu::from(
                url,
                source,
                self.config.clone(),
                false,
            )))
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

        let (tx, rx) = channel();
        let label = label.to_string();
        let rows = self.rows() as u16;
        thread::spawn(move || loop {
            for i in 0..=3 {
                if rx.try_recv().is_ok() {
                    return;
                }
                print!(
                    "{}{}{}{}{}{}{}",
                    terminal::Goto(1, rows),
                    terminal::HideCursor,
                    label,
                    ".".repeat(i),
                    terminal::ClearUntilNewline,
                    theme::color::Reset,
                    terminal::ShowCursor,
                );
                stdout().flush().expect(ERR_STDOUT);
                thread::sleep(Duration::from_millis(500));
            }
        });

        let result = req.join();
        tx.send(true).expect("Fatal Error in Spinner channel."); // stop spinner
        self.dirty = true;
        result.map_err(|e| error!("Spinner error: {:?}", e))
    }

    /// Create a rendered String for the current View in its current state.
    pub fn render(&mut self) -> Result<String> {
        // TODO: only get size on SIGWINCH
        if let Ok((cols, rows)) = terminal_size() {
            self.term_size(cols as usize, rows as usize);
            if !self.views.is_empty() && self.focused < self.views.len() {
                if let Some(view) = self.views.get_mut(self.focused) {
                    view.term_size(cols as usize, rows as usize);
                    return Ok(view.render());
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
        let view = self.views.get(self.focused)?;
        let mut status = vec![];

        if matches!(view.encoding(), Encoding::CP437) {
            status.push("CP439");
        }

        if view.is_tls() {
            if self.config.read().unwrap().emoji {
                status.push("🔐");
            } else {
                status.push("TLS");
            }
        } else if view.is_tor() {
            if self.config.read().unwrap().emoji {
                status.push("🧅");
            } else {
                status.push("TOR");
            }
        }

        if status.is_empty() {
            None
        } else {
            let len = status.iter().fold(0, |a, s| a + s.len());
            let len = len + status.len();
            Some(format!(
                "{}{}",
                terminal::Goto(self.cols() - len as u16, self.rows()),
                status
                    .iter()
                    .map(|s| theme::to_color("bold white") + s + reset_color!())
                    .collect::<Vec<_>>()
                    .join(" "),
            ))
        }
    }

    /// Render the status line.
    fn render_status(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            terminal::HideCursor,
            terminal::Goto(1, self.rows()),
            terminal::ClearCurrentLine,
            self.status,
            self.render_conn_status().unwrap_or_else(|| "".into()),
            theme::color::Reset,
        )
    }

    /// Add a View to the app's currently opened Views.
    fn add_view(&mut self, view: Box<dyn View>) {
        self.dirty = true;
        if !self.views.is_empty() && self.focused < self.views.len() - 1 {
            self.views.truncate(self.focused + 1);
        }
        self.views.push(view);
        if self.views.len() > 1 {
            self.focused += 1;
        }
    }

    /// Ask user to confirm action with ENTER or Y.
    fn confirm(&self, question: &str) -> bool {
        let rows = self.rows();

        let mut out = stdout();
        write!(
            out,
            "{}{}{}{} [Y/n]: {}",
            theme::color::Reset,
            terminal::Goto(1, rows),
            terminal::ClearCurrentLine,
            question,
            terminal::ShowCursor,
        )
        .expect(ERR_STDOUT);
        out.flush().expect(ERR_STDOUT);

        if let Ok(key) = self.keys.lock().unwrap().recv() {
            matches!(key, Key::Char('\n') | Key::Char('y') | Key::Char('Y'))
        } else {
            false
        }
    }

    /// Prompt user for input and return what was entered, if anything.
    fn prompt(&self, prompt: &str, value: &str) -> Option<String> {
        let rows = self.rows();
        let mut input = value.to_string();

        let mut out = stdout();
        write!(
            out,
            "{}{}{}{}{}{}",
            theme::color::Reset,
            terminal::Goto(1, rows),
            terminal::ClearCurrentLine,
            prompt,
            input,
            terminal::ShowCursor,
        )
        .expect(ERR_STDOUT);
        out.flush().expect(ERR_STDOUT);

        let keys = self.keys.lock().unwrap();
        for key in keys.iter() {
            match key {
                Key::Char('\n') => {
                    write!(
                        out,
                        "{}{}",
                        terminal::ClearCurrentLine,
                        terminal::HideCursor
                    )
                    .expect(ERR_STDOUT);
                    out.flush().expect(ERR_STDOUT);
                    return Some(input);
                }
                Key::Char(c) => input.push(c),
                Key::Esc | Key::Ctrl('c') => {
                    write!(
                        out,
                        "{}{}",
                        terminal::ClearCurrentLine,
                        terminal::HideCursor
                    )
                    .expect(ERR_STDOUT);
                    out.flush().expect(ERR_STDOUT);
                    return None;
                }
                Key::Backspace | Key::Delete => {
                    input.pop();
                }
                _ => {}
            }

            write!(
                out,
                "{}{}{}{}",
                terminal::Goto(1, rows),
                terminal::ClearCurrentLine,
                prompt,
                input,
            )
            .expect(ERR_STDOUT);
            out.flush().expect(ERR_STDOUT);
        }

        if !input.is_empty() {
            Some(input)
        } else {
            None
        }
    }

    /// Opens an interactive telnet session.
    fn telnet(&mut self, url: &str) -> Result<()> {
        let gopher::Url { host, port, .. } = gopher::parse_url(url);

        terminal::disable_raw_mode()?;
        let mut cmd = process::Command::new("telnet")
            .arg(host)
            .arg(port)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;
        cmd.wait()?;
        terminal::enable_raw_mode()?;
        self.dirty = true; // redraw when finished with session

        Ok(())
    }

    /// Asks the current View to process user input and produce an Action.
    fn process_view_input(&mut self) -> Action {
        if let Some(view) = self.views.get_mut(self.focused) {
            if let Ok(key) = self.keys.lock().unwrap().recv() {
                return view.respond(key);
            }
        }

        Action::Error("No Gopher page loaded.".into())
    }

    /// Listen for keyboard events and send them along.
    fn spawn_keyboard_listener() -> KeyReceiver {
        let (sender, receiver) = channel();

        // Give our resize handler a channel to send events on.
        *RESIZE_SENDER.lock().unwrap() = Some(sender.clone());
        unsafe {
            libc::signal(libc::SIGWINCH, resize_handler as usize);
            libc::signal(libc::SIGINT, sigint_handler as usize);
            libc::signal(libc::SIGCONT, sigcont_handler as usize);
        }

        thread::spawn(move || {
            for key in stdin().keys().flatten() {
                sender.send(key).unwrap();
            }
        });

        Arc::new(Mutex::new(receiver))
    }

    /// Ctrl-Z: Suspend Unix process w/ SIGTSTP.
    fn suspend(&mut self) {
        terminal::disable_raw_mode().expect("Fatal Error disabling Raw Mode");
        let mut out = stdout();
        write!(out, "{}", terminal::ToMainScreen).expect(ERR_SCREEN);
        out.flush().expect(ERR_STDOUT);
        unsafe { libc::raise(libc::SIGTSTP) };
        write!(out, "{}", terminal::ToAlternateScreen).expect(ERR_SCREEN);
        out.flush().expect(ERR_STDOUT);
        self.dirty = true;
    }

    /// Given an Action from a View in response to user input, do the
    /// action.
    fn process_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::List(actions) => {
                for action in actions {
                    self.process_action(action)?;
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
                let mut out = stdout();
                out.write_all(s.as_ref())?;
                out.flush()?;
            }
            Action::Status(s) => self.set_status(&s),
            Action::Open(title, url) => self.open(&title, &url)?,
            Action::Prompt(query, fun) => {
                if let Some(response) = self.prompt(&query, "") {
                    self.process_action(fun(response))?;
                }
            }
            // F5 = redraw the display on resize
            Action::Keypress(Key::F(5)) => self.dirty = true,
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
                'c' => {
                    let url = match self.views.get(self.focused) {
                        Some(view)=> String::from(view.url()),
                        None => {return Err(error!("Could not get url from view"));},
                    };

                    let u = gopher::parse_url(&url);
                    let default_filename = u
                        .sel
                        .split_terminator('/')
                        .rev()
                        .next()
                        .unwrap_or("");
                    if let Some(filename) = self.prompt("Provide a filepath: ", default_filename){
                        match self.download_file_with_filename(url.as_str(), String::from(filename)){
                            Ok(()) => (),
                            Err(e) => return Err(error!("Save failed: {}", e)),
                        }
                    }
                }
                'g' => {
                    if let Some(url) = self.prompt("Go to URL: ", "") {
                        self.open(&url, &url)?;
                    }
                }
                'h' => self.open("Help", "gopher://phetch/1/help")?,
                'r' => {
                    if let Some(view) = self.views.get(self.focused) {
                        let url = view.url();
                        let mut text =
                            Text::from(url, view.raw().into(), self.config.clone(), view.is_tls());
                        text.wide = true;
                        self.add_view(Box::new(text));
                    }
                }
                'R' => {
                    if let Some(view) = self.views.get(self.focused) {
                        let url = view.url().to_owned();
                        self.open(&url, &url)?;
                    }
                }
                's' => {
                    if let Some(view) = self.views.get(self.focused) {
                        let url = view.url();
                        match bookmarks::save(url, url) {
                            Ok(()) => {
                                let msg = format!("Saved bookmark: {}", url);
                                self.set_status(&msg);
                            }
                            Err(e) => return Err(error!("Save failed: {}", e)),
                        }
                    }
                }
                'u' => {
                    if let Some(view) = self.views.get(self.focused) {
                        let current_url = view.url();
                        if let Some(url) = self.prompt("Current URL: ", current_url) {
                            self.open(&url, &url)?;
                        }
                    }
                }
                'y' => {
                    if let Some(view) = self.views.get(self.focused) {
                        let url = view.url();
                        utils::copy_to_clipboard(url)?;
                        let msg = format!("Copied {} to clipboard.", url);
                        self.set_status(&msg);
                    }
                }
                'w' => {
                    let wide = self.config.read().unwrap().wide;
                    self.config.write().unwrap().wide = !wide;
                    if let Some(view) = self.views.get_mut(self.focused) {
                        let w = view.wide();
                        view.set_wide(!w);
                        self.dirty = true;
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
