use config;
use std::io::{Read, Result};

// Bookmarks only work if you've created a ~/.config/phetch/ manually.
const BOOKMARKS_FILE: &str = "bookmarks.gph";

macro_rules! dir_missing_fmt {
    () => {
        "i\r\ni\r
i\r\ni\x1b[91m{error}\x1b[0m\r
i\r\niBookmarks can only be saved if {dir} exists.\r
i\r\niRun this in your terminal to enable bookmarking:\r
i\r\nimkdir -p {dir}"
    };
}

pub fn as_raw_menu() -> String {
    let path = config::path();
    if let Err(e) = path {
        return format!(dir_missing_fmt!(), error = e, dir = config::DIR);
    }

    let mut out = format!("i{}{}:\r\ni\r\n", config::DIR, BOOKMARKS_FILE);

    let path = path.unwrap().join(BOOKMARKS_FILE);
    if !path.exists() {
        out.push_str("iNo bookmarks yet.\r\ni\r\niUse <ctrl-s> to bookmark a page.\r\n");
        return out;
    }

    config::load(BOOKMARKS_FILE)
        .and_then(|mut reader| reader.read_to_string(&mut out))
        .map_err(|e| {
            out = format!("3{}", e);
            e
        });
    out
}

// save a single bookmark entry
pub fn save(label: &str, url: &str) -> Result<()> {
    config::append(BOOKMARKS_FILE, label.trim_start_matches("gopher://"), url)
}
