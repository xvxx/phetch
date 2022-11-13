//! phetch will load `~/.config/phetch/phetch.conf` or a file you
//! specify with the `--config` command line option when it starts.
//!
//! An example default config is provided but unused by this module.

use {
    crate::{encoding::Encoding, phetchdir, ui},
    std::{
        collections::HashMap,
        fs::OpenOptions,
        io::{Read, Result},
        sync::{Arc, RwLock},
    },
};

/// Global, shared config.
pub type SharedConfig = Arc<RwLock<Config>>;

/// phetch will look for this file on load.
const CONFIG_FILE: &str = "phetch.conf";

/// Default start page.
const DEFAULT_START: &str = "gopher://phetch/1/home";

/// Default media player.
const DEFAULT_MEDIA_PLAYER: &str = "mpv";

/// Example of what a default phetch.conf would be.
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

# Program to use to open media files.
media mpv

# Whether to auto play media
autoplay no

# Use emoji indicators for TLS & Tor. (--emoji)
emoji no

# Encoding. Only CP437 and UTF8 are supported.
encoding utf8

# Wrap text at N columns. 0 = off (--wrap)
wrap 0
";

/// Not all the config options are available in the phetch.conf. We
/// also use this struct to keep track of our session's overall state,
/// such as the UI mode (Print, Run, Raw, etc).
#[derive(Debug)]
pub struct Config {
    /// Gopher URL to open on bare launch
    pub start: String,
    /// Whether to use TLS or not
    pub tls: bool,
    /// Using Tor proxy?
    pub tor: bool,
    /// Wide mode
    pub wide: bool,
    /// Render connection status as emoji
    pub emoji: bool,
    /// Media player to use.
    pub media: Option<String>,
    /// Whether to automatically play media
    pub autoplay: bool,
    /// Default encoding
    pub encoding: Encoding,
    /// UI mode. Can't be set in conf file.
    pub mode: ui::Mode,
    /// Column to wrap lines. 0 = off
    pub wrap: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            start: String::from(DEFAULT_START),
            tls: false,
            tor: false,
            wide: false,
            emoji: false,
            media: Some(DEFAULT_MEDIA_PLAYER.into()),
            autoplay: false,
            encoding: Encoding::default(),
            mode: ui::Mode::default(),
            wrap: 0,
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
    let mut reader = OpenOptions::new().read(true).open(path)?;
    let mut file = String::new();
    reader.read_to_string(&mut file)?;
    parse(&file)
}

/// Does the config file exist?
pub fn exists() -> bool {
    phetchdir::exists(CONFIG_FILE)
}

/// Parses a phetch config file into a Config struct.
fn parse(text: &str) -> Result<Config> {
    let mut cfg = Config::default();
    let mut keys: HashMap<&str, bool> = HashMap::new();

    for (mut linenum, line) in text.split_terminator('\n').enumerate() {
        linenum += 1;
        // skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // skip comments
        if let Some('#') = line.chars().next() {
            continue;
        }

        // line format: "KEY VALUE"
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(error!(
                r#"Expected "key value" format on line {}: {:?}"#,
                linenum, line
            ));
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
            "wrap" => {
                if let Ok(num) = val.parse() {
                    cfg.wrap = num;
                } else {
                    return Err(error!(
                        "`wrap` expects a number value on line {}: {}",
                        linenum, val
                    ));
                }
            }
            "media" => {
                cfg.media = match val.to_lowercase().as_ref() {
                    "false" | "none" => None,
                    _ => Some(val.into()),
                }
            }
            "autoplay" => {
                cfg.autoplay = to_bool(val)?
            }
            "encoding" => {
                cfg.encoding = Encoding::from_str(val)
                    .map_err(|e| error!("{} on line {}: {:?}", e, linenum, line))?;
            }
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
        assert_eq!(config.media, Some("mpv".to_string()));
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
    fn test_media() {
        let cfg = parse("media FALSE").unwrap();
        assert_eq!(cfg.media, None);

        let cfg = parse("media None").unwrap();
        assert_eq!(cfg.media, None);

        let cfg = parse("media /path/to/media-player").unwrap();
        assert_eq!(cfg.media, Some("/path/to/media-player".to_string()));

        let cfg = parse("media vlc").unwrap();
        assert_eq!(cfg.media, Some("vlc".to_string()));
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

    #[test]
    fn test_encoding() {
        let cfg = parse("tls true\nwide no\nemoji yes").unwrap();
        assert_eq!(cfg.tls, true);
        assert_eq!(cfg.encoding, Encoding::default());

        let cfg = parse("tls true\nencoding utf8\n").unwrap();
        assert_eq!(cfg.tls, true);
        assert_eq!(cfg.encoding, Encoding::UTF8);

        let cfg = parse("tls true\nencoding CP437\n").unwrap();
        assert_eq!(cfg.tls, true);
        assert_eq!(cfg.encoding, Encoding::CP437);

        let res = parse("tls true\nencoding what\n");
        assert!(res.is_err());
    }
}
