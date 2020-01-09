use crate::phetchdir;
use std::io::{BufRead, Result};

/// History only works if you've created ~/.config/phetch/history.gph manually.
const HISTORY_FILE: &str = "history.gph";

macro_rules! file_missing_fmt {
    () => {
        "i\r\ni\r
i\r\ni\x1b[91m{error}\x1b[0m
i\r\niHistory is only saved if {file} exists.
i\r\niRun this in your terminal to activate automatic history saving:
i\r\nimkdir -p {dir} && touch {file}"
    };
}

/// Returns history as a Gophermap.
pub fn as_raw_menu() -> String {
    let homepath = format!("{}{}", phetchdir::DIR, HISTORY_FILE);
    let path = phetchdir::path();
    if let Err(error) = path {
        return format!(
            file_missing_fmt!(),
            file = homepath,
            dir = phetchdir::DIR,
            error = error
        );
    }
    let path = path.unwrap();
    let file = path.join(HISTORY_FILE);
    if !file.exists() {
        return format!(
            file_missing_fmt!(),
            file = homepath,
            dir = phetchdir::DIR,
            error = "No history file found."
        );
    }

    let mut out = vec![format!("i{}:\r\ni", homepath)];
    phetchdir::load(HISTORY_FILE)
        .and_then(|reader| {
            let mut lines = reader.lines();
            while let Some(Ok(line)) = lines.next() {
                out.insert(1, line);
            }
            Ok(())
        })
        .map_err(|e| {
            out.push(format!("3{}", e));
            e
        });

    if out.len() == 1 {
        out.insert(1, "iNo history entries yet.".to_string());
    }
    out.join("\r\n")
}

/// Save a single history entry if the history file exists.
pub fn save(label: &str, url: &str) -> Result<()> {
    if let Err(e) = phetchdir::path() {
        return Err(error!("History file doesn't exist: {}", e));
    }

    phetchdir::append(HISTORY_FILE, label, url)
}
