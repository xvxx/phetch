use std::fmt;

// Create a color:: struct that can be used with format!.
macro_rules! define_color {
    ($name:ident, $code:literal) => {
        pub struct $name;
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\x1b[{}m", $code)
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
