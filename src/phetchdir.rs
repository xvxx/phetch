//! The phetchdir is `DIR`, or `~/.config/phetch` by default. There is
//! currently no way to change it. Bookmarks, user history, and the
//! `phetch.conf` all live in this directory in a fully loaded
//! installation of phetch.
//!
//! This module provides helpers for working with the phetchdir:
//! checking its existence, saving to files inside it, and the sort.

use crate::gopher;
use std::{
    fs::{File, OpenOptions},
    io::{prelude::*, BufReader, Result, Write},
};

/// The directory where phetch stores its files. Ex: bookmarks file
/// If you want the full, expanded path, use `path()`.
pub const DIR: &str = "~/.config/phetch/";

/// Check if a file exists within the phetchdir.
pub fn exists(filename: &str) -> bool {
    if let Ok(path) = path() {
        path.join(filename).exists()
    } else {
        false
    }
}

/// Loads a file from the phetchdir for reading.
pub fn load(filename: &str) -> Result<BufReader<File>> {
    path().and_then(|dotdir| {
        let path = dotdir.join(filename);
        if let Ok(file) = OpenOptions::new().read(true).open(&path) {
            Ok(BufReader::new(file))
        } else {
            Err(error!("Couldn't open {:?}", path))
        }
    })
}

/// Append a menu item as a line to a file in the phetchdir.
pub fn append(filename: &str, label: &str, url: &str) -> Result<()> {
    path().and_then(|dotdir| {
        let path = dotdir.join(filename);
        if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(path) {
            let u = gopher::parse_url(url);
            write!(
                file,
                "{}{}\t{}\t{}\t{}\r\n",
                u.typ.to_char(),
                label,
                u.sel,
                u.host,
                u.port
            )?;
            Ok(())
        } else {
            Err(error!("Can't open file for writing: {:?}", filename))
        }
    })
}

/// Add a menu item as the first line in a file in the phetchdir.
pub fn prepend(filename: &str, label: &str, url: &str) -> Result<()> {
    path().and_then(|dotdir| {
        let path = dotdir.join(filename);
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
        {
            let url = gopher::parse_url(url);
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            file.seek(std::io::SeekFrom::Start(0))?;
            write!(
                file,
                "{}{}\t{}\t{}\t{}\r\n",
                url.typ.to_char(),
                label,
                url.sel,
                url.host,
                url.port
            )?;
            file.write_all(&buf)?;
            Ok(())
        } else {
            Err(error!("Can't open file for writing: {:?}", filename))
        }
    })
}

/// Returns the full, expanded PathBuf of the phetchdir only if it exists.
/// Returns None otherwise.
/// If you just want the phetchdir path whether or not it exists, use
/// the DIR constant directly.
pub fn path() -> Result<std::path::PathBuf> {
    let homevar = std::env::var("HOME");
    if homevar.is_err() {
        return Err(error!("$HOME not set, can't decode `~`"));
    }

    let dotdir = DIR.replace('~', &homevar.unwrap());
    let dotdir = std::path::Path::new(&dotdir);
    if dotdir.exists() {
        Ok(std::path::PathBuf::from(dotdir))
    } else {
        Err(error!("Config dir not found: {}", DIR))
    }
}
