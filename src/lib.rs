#![allow(unused_must_use)]

extern crate termion;

#[macro_use]
pub mod utils;
pub mod bookmarks;
pub mod config;
pub mod gopher;
pub mod help;
pub mod history;
pub mod menu;
pub mod text;
pub mod ui;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
