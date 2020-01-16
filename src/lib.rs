//! phetch is a terminal Gopher client designed to help you quickly
//! navigate the Gophersphere securely. It includes support for
//! browsing via TLS or Tor proxy, but can also be used the old
//! fashioned way. The menu-based interface is driven entirely by your
//! keyboard, and emphasis has been put on swiftness of navigation.
//! Both "jump to link" and incremental search options are available.
//!
//! We don't use any ncurses-style library for drawing but instead do
//! it manually, mainly in `UI`. This is the application class that
//! prints to the screen and manages the loaded Gopher "views".
//! Meanwhile, the bulk of the menu system, responding to user
//! input, and rendering of the colored interface itself takes place
//! in the `Menu` view. The two work hand-in-hand.
//!
//! This crate includes its own Gopher parsing and fetching library,
//! which may eventually get extracted. It seems that most existing
//! Gopher libraries in Rust allow you to parse menu lines
//! individually into items, but phetch needs to know about the links'
//! relationships to each other for its navigation/numbering/cursor
//! system to work. So phetch parses all the lines in a Menu as a
//! whole and knows which link is which.
//!
//! Finally, a note on the code itself: this is not my first Rust
//! program, but you probably wouldn't know that by looking at it!
//! Suggestions and improvements are more than welcome.
//!

#![allow(unused_must_use)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(explicit_outlives_requirements)]
#![warn(unreachable_pub)]
#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![allow(clippy::while_let_on_iterator)]
#![allow(clippy::write_with_newline)]

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
#[cfg(feature = "tls")]
pub const TLS_SUPPORT: bool = true;
#[cfg(not(feature = "tls"))]
pub const TLS_SUPPORT: bool = false;

/// Whether we compiled with Tor support.
#[cfg(feature = "tor")]
pub const TOR_SUPPORT: bool = true;
#[cfg(not(feature = "tor"))]
pub const TOR_SUPPORT: bool = false;
