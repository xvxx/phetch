use config;

use std::io::Read;

const BOOKMARKS_FILE: &str = "bookmarks.gph";

pub fn as_raw_menu() -> String {
    let mut out = format!("i** bookmarks **\r\ni\r\n");

    config::load(BOOKMARKS_FILE)
        .and_then(|mut reader| reader.read_to_string(&mut out))
        .map_err(|e| {
            out = format!("3{}", e);
            e
        });
    out
}

// save a single history entry
pub fn save(label: &str, url: &str) {
    config::append(BOOKMARKS_FILE, label, url);
}
