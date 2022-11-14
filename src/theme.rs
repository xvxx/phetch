//! Terminal color scheme.
//! Provides the Theme struct and functions/macros for making use of it.
use std::fmt;

/// Provides a shortcut to the Reset color code.
pub mod color {
    use std::fmt;

    /// Can be used with fmt calls to reset to terminal defaults.
    pub struct Reset;

    impl fmt::Display for Reset {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\x1b[0m")
        }
    }
}

/// Use with push_str() or something.
macro_rules! reset_color {
    () => {
        "\x1b[0m"
    };
}

/// Color scheme for UI and menu items.
#[derive(Debug)]
pub struct Theme {
    // UI Colors
    /// The * cursor that appears next to the selected menu item.
    pub ui_cursor: String,
    /// The Number that appears to the left of a menu item.
    pub ui_number: String,
    /// The text in a menu.
    pub ui_menu: String,
    /// The color of the text content in a document.
    pub ui_text: String,

    // Menu Item Colors
    /// Text document.
    pub item_text: String,
    /// Another menu.
    pub item_menu: String,
    /// Something went wrong.
    pub item_error: String,
    /// Gopher search prompt
    pub item_search: String,
    /// Telnet item. MUDs and stuff.
    pub item_telnet: String,
    /// External link. HTTP, usually.
    pub item_external: String,
    /// Binary file that can be downloaded to disk.
    pub item_download: String,
    /// Media that can be opened, like an image or mp3.
    pub item_media: String,
    /// An unknown or unsupported Gopher type.
    pub item_unsupported: String,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            ui_cursor: to_color("white bold"),
            ui_number: to_color("magenta"),
            ui_menu: to_color("yellow"),
            ui_text: to_color("white"),

            item_text: to_color("cyan"),
            item_menu: to_color("blue"),
            item_error: to_color("red"),
            item_search: to_color("white"),
            item_telnet: to_color("grey"),
            item_external: to_color("green"),
            item_download: to_color("white underline"),
            item_media: to_color("green underline"),
            item_unsupported: to_color("whitebg red"),
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "# phetch theme
ui.cursor {ui_cursor}
ui.number {ui_number}
ui.menu {ui_menu}
ui.text {ui_text}

item.text {item_text}
item.menu {item_menu}
item.error {item_error}
item.search {item_search}
item.telnet {item_telnet}
item.external {item_external}
item.download {item_download}
item.media {item_media}
item.unsupported {item_unsupported}",
            ui_cursor = to_words(&self.ui_cursor),
            ui_number = to_words(&self.ui_number),
            ui_menu = to_words(&self.ui_menu),
            ui_text = to_words(&self.ui_text),
            item_text = to_words(&self.item_text),
            item_menu = to_words(&self.item_menu),
            item_error = to_words(&self.item_error),
            item_search = to_words(&self.item_search),
            item_telnet = to_words(&self.item_telnet),
            item_external = to_words(&self.item_external),
            item_download = to_words(&self.item_download),
            item_media = to_words(&self.item_media),
            item_unsupported = to_words(&self.item_unsupported),
        )
    }
}

/// Convert a string like "blue underline" or "red" into a color code.
pub fn to_color<S: AsRef<str>>(line: S) -> String {
    let parts = line.as_ref().split(' ').collect::<Vec<_>>();

    if parts.is_empty() {
        return "".into();
    }

    let mut out = String::from("\x1b[");
    let len = parts.len();

    for (i, part) in parts.iter().enumerate() {
        out.push_str(&color_code(part).to_string());
        if i < len - 1 {
            out.push(';');
        }
    }
    out.push('m');

    out
}

/// Convert color code like "\x1b[91m" into something like "red"
pub fn to_words<S: AsRef<str>>(code: S) -> String {
    code.as_ref()
        .replace("\x1b[", "")
        .replace('m', "")
        .split(';')
        .map(color_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn color_code(color: &str) -> usize {
    match color {
        "bold" => 1,
        "underline" => 4,
        "grey" => 90,
        "red" => 91,
        "green" => 92,
        "yellow" => 93,
        "blue" => 94,
        "magenta" => 95,
        "cyan" => 96,
        "white" => 97,
        "black" => 30,
        "darkred" => 31,
        "darkgreen" => 32,
        "darkyellow" => 33,
        "darkblue" => 34,
        "darkmagenta" => 35,
        "darkcyan" => 36,
        "darkwhite" => 37,
        "blackbg" => 40,
        "redbg" => 41,
        "greenbg" => 42,
        "yellowbg" => 43,
        "bluebg" => 44,
        "magentabg" => 45,
        "cyanbg" => 46,
        "whitebg" => 47,
        _ => 0,
    }
}

fn color_word(code: &str) -> &'static str {
    match code {
        "1" => "bold",
        "4" => "underline",
        "90" => "grey",
        "91" => "red",
        "92" => "green",
        "93" => "yellow",
        "94" => "blue",
        "95" => "magenta",
        "96" => "cyan",
        "97" => "white",
        "30" => "black",
        "31" => "darkred",
        "32" => "darkgreen",
        "33" => "darkyellow",
        "34" => "darkblue",
        "35" => "darkmagenta",
        "36" => "darkcyan",
        "37" => "darkwhite",
        "40" => "blackbg",
        "41" => "redbg",
        "42" => "greenbg",
        "43" => "yellowbg",
        "44" => "bluebg",
        "45" => "magentabg",
        "46" => "cyanbg",
        "47" => "whitebg",
        _ => "white",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme() {
        let mut theme = Theme::default();
        theme.ui_cursor = to_color("bold").into();
        theme.ui_menu = to_color("red").into();
        theme.item_menu = to_color("blue underline").into();

        assert_eq!("\u{1b}[1m", theme.ui_cursor);
        assert_eq!("\u{1b}[91m", theme.ui_menu);
        assert_eq!("\u{1b}[94;4m", theme.item_menu);
    }
}
