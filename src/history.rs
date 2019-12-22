use gopher;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn load_as_raw_menu() -> Option<String> {
    let mut out = vec![];

    if let Some(reader) = load() {
        let mut lines = reader.lines();
        while let Some(Ok(url)) = lines.next() {
            let (t, host, port, sel) = gopher::parse_url(&url);
            out.insert(
                0,
                format!(
                    "{}{}\t{}\t{}\t{}",
                    gopher::char_for_type(t).unwrap_or('i'),
                    url,
                    sel,
                    host,
                    port
                ),
            );
        }
    }

    out.insert(0, "i~/.config/phetch/history:\r\ni".into());
    Some(out.join("\r\n"))
}

pub fn load() -> Option<BufReader<File>> {
    let dotdir = config_dir_path();
    if dotdir.is_none() {
        return None;
    }
    let history = dotdir.unwrap().join("history");
    if let Ok(file) = std::fs::OpenOptions::new().read(true).open(history) {
        return Some(BufReader::new(file));
    }
    None
}

pub fn save(urls: &[impl std::fmt::Display]) {
    let dotdir = config_dir_path();
    if dotdir.is_none() {
        return;
    }
    let dotdir = dotdir.unwrap();
    let mut out = String::new();
    for url in urls {
        out.push_str(url.to_string().as_ref());
        out.push('\n');
    }
    let history = dotdir.join("history");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(history)
    {
        file.write_all(out.as_ref());
    }
}

pub fn config_dir_path() -> Option<std::path::PathBuf> {
    let homevar = std::env::var("HOME");
    if homevar.is_err() {
        return None;
    }

    let dotdir = "~/.config/phetch".replace('~', &homevar.unwrap());
    let dotdir = std::path::Path::new(&dotdir);
    if dotdir.exists() {
        Some(std::path::PathBuf::from(dotdir))
    } else {
        None
    }
}
