//! Helper functions and macros.

/// Debug macro that appends a line to `phetch.log`.
/// Useful for printf-style debugging - add your `log!()` calls,
/// and `tail -f phetch.log` while running phetch to peek inside.
#[allow(unused_macros)]
macro_rules! log {
    ($e:expr) => {{
        if cfg!(debug_assertions) {
            if let Ok(mut file) = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("phetch.log")
        {
            use std::io::prelude::*;
            file.write($e.as_ref());
            file.write(b"\n");
        }
    }
    }};
    ($e:expr, $($y:expr),*) => {
        if cfg!(debug_assertions) {
            log!(format!($e, $($y),*));
        }
    };
}

/// Creates an Other kind of io::Error.
macro_rules! error {
    ($e:expr) => {
        std::io::Error::new(std::io::ErrorKind::Other, $e)
    };
    ($e:expr, $($y:expr),*) => {
        error!(format!($e, $($y),*));
    };
}

/// Number of bytes in a human-ish readable format.
pub fn human_bytes(bytes: usize) -> String {
    let (count, tag) = if bytes < 1000 {
        (bytes, " bytes")
    } else if bytes < 1_000_000 {
        (bytes / 1000, "Kb")
    } else if bytes < 1_000_000_000 {
        (bytes / 1_000_000, "Mb")
    } else {
        (bytes / 1_000_000_000, "Gb")
    };

    format!("{}{}", count, tag)
}
