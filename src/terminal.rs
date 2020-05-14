//! The terminal module mostly provides terminal escape sequences for
//! things like clearing the screen or going into alternate mode, as
//! well as raw mode borrowed from crossterm.

use lazy_static::lazy_static;
use libc::{cfmakeraw, tcgetattr, tcsetattr, termios as Termios, STDIN_FILENO, TCSANOW};
use std::{io, sync::Mutex};
use termion;

pub use termion::cursor::Goto;
pub use termion::cursor::Hide as HideCursor;
pub use termion::cursor::Show as ShowCursor;

pub use termion::screen::ToAlternateScreen;
pub use termion::screen::ToMainScreen;

pub use termion::clear::AfterCursor as ClearAfterCursor;
pub use termion::clear::All as ClearAll;
pub use termion::clear::CurrentLine as ClearCurrentLine;
pub use termion::clear::UntilNewline as ClearUntilNewline;

type Result<T> = std::result::Result<T, io::Error>;

lazy_static! {
    // Some(Termios) -> we're in the raw mode and this is the previous mode
    // None -> we're not in the raw mode
    static ref TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<Option<Termios>> = Mutex::new(None);
}

/// Are we in raw mode?
pub fn is_raw_mode_enabled() -> bool {
    TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap().is_some()
}

/// Go into "raw" mode, courtesy of crossterm.
pub fn enable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if original_mode.is_some() {
        return Ok(());
    }

    let mut ios = get_terminal_attr()?;
    let original_mode_ios = ios;

    raw_terminal_attr(&mut ios);
    set_terminal_attr(&ios)?;

    // Keep it last - set the original mode only if we were able to switch to the raw mode
    *original_mode = Some(original_mode_ios);

    Ok(())
}

/// Back it up.
pub fn disable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();

    if let Some(original_mode_ios) = original_mode.as_ref() {
        set_terminal_attr(original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }

    Ok(())
}

// Transform the given mode into an raw mode (non-canonical) mode.
fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

fn get_terminal_attr() -> Result<Termios> {
    unsafe {
        let mut termios = std::mem::zeroed();
        wrap_with_result(tcgetattr(STDIN_FILENO, &mut termios))?;
        Ok(termios)
    }
}

fn set_terminal_attr(termios: &Termios) -> Result<bool> {
    wrap_with_result(unsafe { tcsetattr(STDIN_FILENO, TCSANOW, termios) })
}

fn wrap_with_result(result: i32) -> Result<bool> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(true)
    }
}
