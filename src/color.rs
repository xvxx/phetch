//! Terminal colors.
//! Provides a macro to color text as well as sturcts to get their
//! raw ansi codes.

use std::fmt;

/// Shortcut to produce a String colored with one or more colors.
/// Example:
/// ```
///   let s = color!("Red string", Red);
///   let x = color!("Hyperlink-ish", Blue, Underline);
macro_rules! color {
    ($s:expr, $( $color:ident ),+) => {{
        use crate::color;
        let codes = [$( color::$color.as_ref() ),+];
        if codes.len() > 1 {
            format!("\x1b[{}m{}{}", codes.join(";"), $s, color::Reset)
        } else {
            format!("\x1b[{}m{}{}", codes[0], $s, color::Reset)
        }
    }};
}

/// Create a color:: struct that can be used with format!.
/// Example:
/// ```
///   define_color(Red, 91);
///   define_color(Reset, 0);
///
///   println!("{}Error: {}{}", color::Red, msg, color::Reset);
macro_rules! define_color {
    ($color:ident, $code:literal) => {
        #[allow(missing_docs)]
        pub struct $color;
        impl fmt::Display for $color {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\x1b[{}m", self.as_ref())
            }
        }
        impl AsRef<str> for $color {
            #[inline]
            fn as_ref(&self) -> &'static str {
                concat!($code)
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
    #[test]
    fn test_colors() {
        assert_eq!(color!("Error", Red), "\x1b[91mError\x1b[0m");
        assert_eq!(
            color!("Fancy Pants", Blue, Underline),
            "\x1b[94;4mFancy Pants\x1b[0m"
        );
        assert_eq!(
            color!("Super-duper-fancy-pants", Magenta, Underline, Bold, BlueBG),
            "\x1b[95;4;1;44mSuper-duper-fancy-pants\x1b[0m"
        )
    }
}
