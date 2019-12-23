use config;
use std::io::{BufRead, Result};

// History only works if you've created ~/.config/phetch/history.gph manually.
const HISTORY_FILE: &str = "history.gph";

macro_rules! file_missing_fmt {
    () => {
        "i\r\ni\r
i\r\ni\x1b[91m{error}\x1b[0m\r
i\r\niHistory is only saved if {file} exists.\r
i\r\niRun this in your terminal to activate automatic history saving:\r
i\r\nimkdir -p {dir} && touch {file}"
    };
}

pub fn as_raw_menu() -> String {
    let homepath = format!("{}{}", config::DIR, HISTORY_FILE);
    let path = config::path();
    if path.is_err() {
        return format!(
            file_missing_fmt!(),
            file = homepath,
            dir = config::DIR,
            error = path.unwrap_err()
        );
    }
    let path = path.unwrap();
    let file = path.join(HISTORY_FILE);
    if !file.exists() {
        return format!(
            file_missing_fmt!(),
            file = homepath,
            dir = config::DIR,
            error = "No history file found."
        );
    }

    let mut out = vec![format!("i{}:\r\ni", file.to_string_lossy())];
    config::load(HISTORY_FILE)
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

// save a single history entry if the history file exists
pub fn save(label: &str, url: &str) -> Result<()> {
    let file = format!("{}{}", config::DIR, HISTORY_FILE);
    if !std::path::Path::new(&file).exists() {
        return Err(error!("History file doesn't exist: {}", file));
    }

    config::append(HISTORY_FILE, label, url)
}
