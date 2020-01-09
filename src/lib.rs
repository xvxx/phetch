#![allow(unused_must_use)]

#[macro_use]
pub mod utils;
pub mod bookmarks;
pub mod config;
pub mod gopher;
pub mod help;
pub mod history;
pub mod menu;
pub mod phetchdir;
pub mod text;
pub mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PLATFORM: &str = env!("PLATFORM");
pub const GIT_REF: &str = env!("GIT_REF");
pub const BUILD_DATE: &str = env!("BUILD_DATE");
pub const BUG_URL: &str = "https://github.com/dvkt/phetch/issues/new";

#[cfg(not(feature = "disable-tls"))]
pub const TLS_SUPPORT: &str = "enabled";
#[cfg(feature = "disable-tls")]
pub const TLS_SUPPORT: &str = "not enabled";
