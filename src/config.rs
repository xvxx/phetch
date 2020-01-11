use crate::phetchdir;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Read, Result},
};

/// phetch will look for this file on load.
const CONFIG_FILE: &str = "phetch.conf";

/// Default start page.
const DEFAULT_START: &str = "gopher://phetch/1/home";

/// Default config
pub const DEFAULT_CONFIG: &str = "## default config file for the phetch gopher client
## gopher://phkt.io/1/phetch

# Page to load when launched with no URL argument.
start gopher://phetch/1/home

# Always use TLS mode. (--tls)
tls no

# Connect using local Tor proxy. (--tor)
tor no

# Always start in wide mode. (--wide)
wide no

# Use emoji indicators for TLS & Tor. (--emoji)
emoji no
";

#[derive(Debug)]
pub struct Config {
    pub start: String,
    pub tls: bool,
    pub tor: bool,
    pub wide: bool,
    pub emoji: bool,
    pub cursor: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            start: String::from(DEFAULT_START),
            tls: false,
            tor: false,
            wide: false,
            emoji: false,
            cursor: true,
        }
    }
}

/// Returns the config phetch uses when launched with no flags or
/// config file modification.
pub fn default() -> Config {
    Default::default()
}

/// Attempt to load ~/.config/phetch/phetch.conf from disk.
pub fn load() -> Result<Config> {
    let mut reader = phetchdir::load(CONFIG_FILE)?;
    let mut file = String::new();
    reader.read_to_string(&mut file)?;
    parse(&file)
}

/// Attempt to load a config from disk.
pub fn load_file(path: &str) -> Result<Config> {
    let mut reader = OpenOptions::new().read(true).open(&path)?;
    let mut file = String::new();
    reader.read_to_string(&mut file)?;
    parse(&file)
}

/// Does the config file exist?
pub fn exists() -> bool {
    phetchdir::exists(CONFIG_FILE)
}

/// Parses a phetch config file into a Config struct.
pub fn parse(text: &str) -> Result<Config> {
    let mut linenum = 0;
    let mut cfg = default();
    let mut keys: HashMap<&str, bool> = HashMap::new();

    for line in text.split_terminator('\n') {
        linenum += 1;

        // skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // skip comments
        if let Some('#') = line.chars().nth(0) {
            continue;
        }

        // line format: "KEY VALUE"
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(error!("Wrong format for line {}: {:?}", linenum, line));
        }
        let (key, val) = (parts[0], parts[1]);
        if keys.contains_key(key) {
            return Err(error!("Duplicate key on line {}: {}", linenum, key));
        }
        match key {
            "start" => cfg.start = val.into(),
            "emoji" => cfg.emoji = to_bool(val)?,
            "tls" => cfg.tls = to_bool(val)?,
            "tor" => cfg.tor = to_bool(val)?,
            "wide" => cfg.wide = to_bool(val)?,
            _ => return Err(error!("Unknown key on line {}: {}", linenum, key)),
        }
        keys.insert(key, true);
    }

    Ok(cfg)
}

/// Converts a config file's boolean value like "yes" or "false" to a
/// real bool.
fn to_bool(val: &str) -> Result<bool> {
    match val.to_lowercase().as_ref() {
        "yes" | "true" | "y" => Ok(true),
        "no" | "false" | "n" => Ok(false),
        _ => Err(error!("Not a boolean: {}", val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default() {
        let config = parse(DEFAULT_CONFIG).expect("Couldn't parse config");
        assert_eq!(config.tls, false);
        assert_eq!(config.tor, false);
        assert_eq!(config.wide, false);
        assert_eq!(config.emoji, false);
        assert_eq!(config.start, "gopher://phetch/1/home");
    }

    #[test]
    fn test_bad_key() {
        let res = parse("random-key yes");
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_new_start() {
        let cfg = parse("start bitreich.org/1/lawn").unwrap();
        assert_eq!(cfg.start, "bitreich.org/1/lawn");
    }

    #[test]
    fn test_comments_ignored() {
        let cfg = parse("# wide yes\ntls yes").unwrap();
        assert_eq!(cfg.wide, false);
        assert_eq!(cfg.tls, true);
    }

    #[test]
    fn test_yes_or_true() {
        let cfg = parse("tls yes\nwide true").unwrap();
        assert_eq!(cfg.tls, true);
        assert_eq!(cfg.wide, true);
    }

    #[test]
    fn test_no_or_false() {
        let cfg = parse("tls false\nwide no\ntor n").unwrap();
        assert_eq!(cfg.tls, false);
        assert_eq!(cfg.tor, false);
        assert_eq!(cfg.wide, false);
    }
    #[test]
    fn test_no_dupe_keys() {
        let res = parse("tls false\nwide no\nemoji yes\ntls yes");
        assert_eq!(res.is_err(), true);
        let e = res.unwrap_err();
        assert_eq!(format!("{}", e), "Duplicate key on line 4: tls");
    }
}
