//! The terminal module mostly provides terminal escape sequences for
//! things like clearing the screen or going into alternate mode.
//!
//! It wraps termion for now, but we may move away from termion in the
//! future and this will help.

use termion;

pub use termion::cursor::Goto;
pub use termion::cursor::Hide as HideCursor;
pub use termion::cursor::Show as ShowCursor;

pub use termion::screen::ToAlternateScreen;
pub use termion::screen::ToMainScreen;

pub use termion::clear::AfterCursor as ClearAfterCursor;
pub use termion::clear::CurrentLine as ClearCurrentLine;
pub use termion::clear::UntilNewline as ClearUntilNewline;
