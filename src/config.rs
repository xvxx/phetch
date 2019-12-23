use gopher;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Result, Write};

pub const DIR: &str = "~/.config/phetch/";

// Loads a file in the config directory for reading.
pub fn load(filename: &str) -> Result<BufReader<File>> {
    if let Some(dotdir) = path() {
        let path = dotdir.join(filename);
        if let Ok(file) = OpenOptions::new().read(true).open(&path) {
            Ok(BufReader::new(file))
        } else {
            Err(error!("Couldn't open {:?}", path))
        }
    } else {
        Err(error!("{} directory doesn't exist", DIR))
    }
}

// Append a menu item as a line to a file in the config dir.
pub fn append(filename: &str, label: &str, url: &str) {
    if let Some(dotdir) = path() {
        let filename = dotdir.join(filename);
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
        {
            let (t, host, port, sel) = gopher::parse_url(&url);
            file.write_all(
                format!(
                    "{}{}\t{}\t{}\t{}\r\n",
                    gopher::char_for_type(t).unwrap_or('i'),
                    label,
                    sel,
                    host,
                    port
                )
                .as_ref(),
            );
        }
    }
}

// PathBuf to config dir if it exists.
// None if the config dir doesn't exist.
pub fn path() -> Option<std::path::PathBuf> {
    let homevar = std::env::var("HOME");
    if homevar.is_err() {
        return None;
    }

    let dotdir = DIR.replace('~', &homevar.unwrap());
    let dotdir = std::path::Path::new(&dotdir);
    if dotdir.exists() {
        Some(std::path::PathBuf::from(dotdir))
    } else {
        None
    }
}
