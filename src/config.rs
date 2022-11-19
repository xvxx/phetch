//! phetch will load `~/.config/phetch/phetch.conf` or a file you
//! specify with the `--config` command line option when it starts.
//!
//! An example default config is provided but unused by this module.

use {
    crate::{
        encoding::Encoding,
        phetchdir,
        theme::{to_color, Theme},
        ui,
    },
    std::{
        collections::HashMap,
        fs::OpenOptions,
        io::{self, Read, Result},
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

# How many lines to page up/down by? 0 = full screen
scroll 0

# Path to theme file, if any
# theme ~/.config/phetch/pink.theme

# Inline Theme
ui.cursor white bold
ui.number magenta
ui.menu yellow
ui.text white
item.text cyan
item.menu blue
item.error red
item.search white
item.telnet grey
item.external green
item.download white underline
item.media green underline
item.unsupported whitebg red
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
    /// Scroll by how many lines? 0 = full screen
    pub scroll: usize,
    /// Color Scheme
    pub theme: Theme,
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
            scroll: 0,
            theme: Theme::default(),
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
            "scroll" => {
                if let Ok(num) = val.parse() {
                    cfg.scroll = num;
                } else {
                    return Err(error!(
                        "`scroll` expects a number value on line {}: {}",
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
            "autoplay" => cfg.autoplay = to_bool(val)?,
            "encoding" => {
                cfg.encoding = Encoding::from_str(val)
                    .map_err(|e| error!("{} on line {}: {:?}", e, linenum, line))?;
            }

            "theme" => {
                let homevar = std::env::var("HOME");
                if homevar.is_err() && val.contains('~') {
                    return Err(error!("$HOME not set, can't decode `~`"));
                }
                cfg.theme = match load_file(&val.replace('~', &homevar.unwrap())) {
                    Ok(cfg) => cfg.theme,
                    Err(e) => {
                        if matches!(e.kind(), io::ErrorKind::NotFound) {
                            return Err(error!(
                                "error loading theme: File not found on line {}: {}",
                                linenum, val
                            ));
                        } else {
                            return Err(error!("error loading theme: {:?}", e));
                        }
                    }
                };
            }

            // color scheme
            "ui.cursor" => cfg.theme.ui_cursor = to_color(val),
            "ui.number" => cfg.theme.ui_number = to_color(val),
            "ui.menu" => cfg.theme.ui_menu = to_color(val),
            "ui.text" => cfg.theme.ui_text = to_color(val),

            "item.text" => cfg.theme.item_text = to_color(val),
            "item.menu" => cfg.theme.item_menu = to_color(val),
            "item.error" => cfg.theme.item_error = to_color(val),
            "item.search" => cfg.theme.item_search = to_color(val),
            "item.telnet" => cfg.theme.item_telnet = to_color(val),
            "item.external" => cfg.theme.item_external = to_color(val),
            "item.download" => cfg.theme.item_download = to_color(val),
            "item.media" => cfg.theme.item_media = to_color(val),
            "item.unsupported" => cfg.theme.item_unsupported = to_color(val),

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

    #[test]
    fn test_missing_theme() {
        if let Err(e) = parse("theme /dont/exists.txt") {
            assert_eq!(
                format!("{}", e),
                "error loading theme: File not found on line 1: /dont/exists.txt"
            );
        }
    }

    #[test]
    fn test_theme_file() {
        use crate::theme::to_words;

        let cfg = parse("theme ./tests/pink.theme").unwrap();
        assert_eq!(to_words(cfg.theme.item_text), "magenta");
        assert_eq!(to_words(cfg.theme.item_menu), "magenta");
        assert_eq!(to_words(cfg.theme.item_error), "red");
        assert_eq!(to_words(cfg.theme.ui_menu), "cyan");
    }

    #[test]
    fn test_theme() {
        use crate::theme::to_words;

        let cfg = parse("item.text green\nitem.download red underline").unwrap();
        assert_eq!(to_words(cfg.theme.item_text), "green");
        assert_eq!(to_words(cfg.theme.item_download), "red underline");
    }
}
