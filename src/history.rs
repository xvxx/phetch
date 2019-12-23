use gopher;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};

const CONFIG_DIR: &str = "~/.config/phetch/";
const HISTORY_FILE: &str = "history.gph";

/// History file is saved in ~/.config/phetch/history.gph if the phetch
/// config directory exists.
pub fn load_as_raw_menu() -> String {
    let mut out = vec![];

    match load() {
        Ok(reader) => {
            let mut lines = reader.lines();
            while let Some(Ok(line)) = lines.next() {
                out.insert(0, line); // reverse order
            }
        }
        Err(e) => out.push(format!("3{}", e)),
    }

    out.insert(0, format!("i{}{}:\r\ni", CONFIG_DIR, HISTORY_FILE));
    out.join("\r\n")
}

pub fn load() -> Result<BufReader<File>> {
    let dotdir = config_dir_path();
    if dotdir.is_none() {
        return Err(error!("{} directory doesn't exist", CONFIG_DIR));
    }
    let history = dotdir.unwrap().join(HISTORY_FILE);
    if let Ok(file) = std::fs::OpenOptions::new().read(true).open(&history) {
        return Ok(BufReader::new(file));
    }
    Err(error!("Couldn't open {:?}", history))
}

// save a single history entry
pub fn save(url: &str, label: &str) {
    let dotdir = config_dir_path();
    if dotdir.is_none() {
        return;
    }
    let dotdir = dotdir.unwrap();
    let history = dotdir.join(HISTORY_FILE);
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(history)
    {
        let (t, host, port, sel) = gopher::parse_url(&url);
        // ignore internal URLs
        if host == "help" {
            return;
        }
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

// Returns None if the config dir doesn't exist.
pub fn config_dir_path() -> Option<std::path::PathBuf> {
    let homevar = std::env::var("HOME");
    if homevar.is_err() {
        return None;
    }

    let dotdir = CONFIG_DIR.replace('~', &homevar.unwrap());
    let dotdir = std::path::Path::new(&dotdir);
    if dotdir.exists() {
        Some(std::path::PathBuf::from(dotdir))
    } else {
        None
    }
}
