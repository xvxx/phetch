//! Terminal colors.
//! Provides a macro to color text as well as sturcts to get their
//! raw ansi codes.

use std::fmt;

/// Shortcut to produce a String colored with one or more colors.
/// Example:
/// ```ignore
///   let s = color_string!("Red string", Red);
///   let x = color_string!("Hyperlink-ish", Blue, Underline);
macro_rules! color_string {
    ($s:expr, $( $color:ident ),+) => {{
        if *crate::NO_COLOR {
            $s.to_string()
        } else {
            let mut out = String::from("\x1b[");
            $( out.push_str(crate::color::$color::code()); out.push_str(";"); )+
            out.push('m');
            out.push_str(&$s);
            out.push_str(crate::color::Reset.as_ref());
            out.replace(";m", "m")
        }
    }};
}

/// Shortcut to produce a color's ANSI escape code. Don't forget to Reset!
/// ```ignore
///   let mut o = String::new();
///   o.push_str(color!(Blue));
///   o.push_str(color!(Underline));
///   o.push_str("Hyperlinkish.");
///   o.push_str(color!(Reset));
macro_rules! color {
    ($color:ident) => {
        if *crate::NO_COLOR {
            ""
        } else {
            crate::color::$color.as_ref()
        }
    };
}

/// Create a color:: struct that can be used with format!.
/// Example:
/// ```ignore
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
                write!(f, "{}", self.as_ref())
            }
        }
        impl $color {
            #[allow(missing_docs)]
            #[inline]
            pub fn code() -> &'static str {
                concat!($code)
            }
        }
        impl AsRef<str> for $color {
            #[inline]
            fn as_ref(&self) -> &'static str {
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
    #[test]
    fn test_colors() {
        assert_eq!(color_string!("Error", Red), "\x1b[91mError\x1b[0m");
        assert_eq!(
            color_string!("Fancy Pants", Blue, Underline),
            "\x1b[94;4mFancy Pants\x1b[0m"
        );
        assert_eq!(
            color_string!("Super-duper-fancy-pants", Magenta, Underline, Bold, BlueBG),
            "\x1b[95;4;1;44mSuper-duper-fancy-pants\x1b[0m"
        )
    }
}
