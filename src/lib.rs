#![allow(unused_must_use)]

#[macro_use]
pub mod utils;
#[macro_use]
pub mod color;
pub mod args;
pub mod bookmarks;
pub mod config;
pub mod gopher;
pub mod help;
pub mod history;
pub mod menu;
pub mod phetchdir;
pub mod text;
pub mod ui;

/// Current version of phetch.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Current OS. Used to check for updates.
pub const PLATFORM: &str = env!("PLATFORM");
/// Git SHA of current build.
pub const GIT_REF: &str = env!("GIT_REF");
/// Date when this release was built.
pub const BUILD_DATE: &str = env!("BUILD_DATE");
/// Where to file issues. We try to catch and display all errors
/// nicely, but if we have to crash we will try to show this.
pub const BUG_URL: &str = "https://github.com/xvxx/phetch/issues/new";

/// Whether we compiled with TLS support.
#[cfg(not(feature = "disable-tls"))]
pub const TLS_SUPPORT: &str = "enabled";
#[cfg(feature = "disable-tls")]
pub const TLS_SUPPORT: &str = "not enabled";
