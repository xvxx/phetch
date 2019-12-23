use config;
use std::io::BufRead;

const HISTORY_FILE: &str = "history.gph";

pub fn as_raw_menu() -> String {
    let mut out = vec![format!("i{}{}:\r\ni", config::DIR, HISTORY_FILE)];

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

    out.join("\r\n")
}

// save a single history entry
pub fn save(label: &str, url: &str) {
    config::append(HISTORY_FILE, label, url);
}
