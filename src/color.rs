use std::fmt;

/// Shortcut to produce a String of a certain color that resets.
/// Example:
///   let s = color!(Red, "Red string");
macro_rules! color {
    ($color:ident, $s:expr) => {{
        use crate::color;
        format!("{}{}{}", color::$color, $s, color::Reset)
    }};
}

// Create a color:: struct that can be used with format!.
macro_rules! define_color {
    ($color:ident, $code:literal) => {
        pub struct $color;
        impl fmt::Display for $color {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }
        impl AsRef<str> for $color {
            fn as_ref(&self) -> &str {
                concat!("\x1b[", $code, "m")
            }
        }
    };
}

define_color!(Reset, 0);
define_color!(Bold, 1);
define_color!(Underline, 4);

define_color!(Grey, 90);
define_color!(Red, 91);
define_color!(Green, 92);
define_color!(Yellow, 93);
define_color!(Blue, 94);
define_color!(Magenta, 95);
define_color!(Cyan, 96);
define_color!(White, 97);

define_color!(Black, 30);
define_color!(DarkRed, 31);
define_color!(DarkGreen, 32);
define_color!(DarkYellow, 33);
define_color!(DarkBlue, 34);
define_color!(DarkMagenta, 35);
define_color!(DarkCyan, 36);
define_color!(DarkWhite, 37);

define_color!(BlackBG, 40);
define_color!(RedBG, 41);
define_color!(GreenBG, 42);
define_color!(YellowBG, 43);
define_color!(BlueBG, 44);
define_color!(MagentaBG, 45);
define_color!(CyanBG, 46);
define_color!(WhiteBG, 47);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        assert_eq!(color!(Red, "Error"), "\x1b[91mError\x1b[0m");
        assert_eq!(
            color!(Blue, color!(Underline, "Fancy Pants")),
            "\x1b[94m\x1b[4mFancy Pants\x1b[0m\x1b[0m"
        );
    }
}
