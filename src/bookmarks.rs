use config;

const BOOKMARKS_FILE: &str = "bookmarks.gph";

pub fn as_raw_menu() -> String {
    let mut out = vec![format!("i** bookmarks **\r\ni")];

    config::load(BOOKMARKS_FILE)
        .and_then(|reader| reader.read(&mut out))
        .map_err(|e| {
            out.push(format!("3{}", e));
            e
        });

    out.join("\r\n")
}

// save a single history entry
pub fn save(label: &str, url: &str) {
    config::append(BOOKMARKS_FILE, label, url);
}
