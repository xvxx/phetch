//! Helper functions and macros.
use std::{
    borrow::Cow,
    io::{Result, Write},
    process::{self, Stdio},
};

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
            file.write($e.as_ref()).unwrap();
            file.write(b"\n").unwrap();
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
        error!(format!($e, $($y),*))
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

/// Copies data to the system clipboard, if possible.
/// Uses `pbcopy` on macOS or `xclip -sel clip` on Linux.
pub fn copy_to_clipboard(data: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    let mut cmd = process::Command::new("pbcopy");
    #[cfg(not(target_os = "macos"))]
    let mut cmd = process::Command::new("xclip");
    #[cfg(not(target_os = "macos"))]
    let cmd = cmd.args(&["-sel", "clip"]);

    cmd.stdin(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(data.as_bytes())
        })
        .map_err(|e| error!("Clipboard error: {}", e))
}

/// Used to open non-Gopher URLs.
/// Runs `open` command on macOS or `xdg-open` on Linux.
pub fn open_external(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    let cmd = "open";
    #[cfg(not(target_os = "macos"))]
    let cmd = "xdg-open";

    let output = process::Command::new(cmd)
        .arg(url)
        .output()
        .map_err(|e| error!("`open` error: {}", e))?;

    if output.stderr.is_empty() {
        Ok(())
    } else {
        Err(error!(
            "`open` error: {}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        ))
    }
}

/// Opens a media file with `mpv` or `--media`.
pub fn open_media(program: &str, url: &str) -> Result<()> {
    use {crate::terminal, std::io};

    // mpv only supports /9/
    let url = if program.ends_with("mpv") {
        Cow::from(url.replace("/;/", "/9/").replace("/s/", "/9/"))
    } else {
        Cow::from(url)
    };

    // support URL: selectors
    let url = if let Some(idx) = url.find("URL:") {
        url.split_at(idx).1.trim_start_matches("URL:")
    } else {
        &url
    };

    let errfn = |e| {
        terminal::enable_raw_mode().unwrap();
        error!("Media player error: {}", e)
    };

    // clear screen first
    let mut stdout = io::stdout();
    write!(stdout, "{}{}", terminal::ClearAll, terminal::Goto(1, 1))?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;
    let mut cmd = process::Command::new(program)
        .arg(url)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(errfn)?;
    cmd.wait().map_err(errfn)?;
    terminal::enable_raw_mode()?;

    Ok(())
}
