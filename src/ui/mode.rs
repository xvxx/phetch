/// The mode our text UI is in. Run mode is the default while
/// Print doesn't show the cursor, among other things.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    Run,     // url
    Print,   // --print url
    NoTTY,   // --print url | cat
    Raw,     // --raw
    Version, // --version
    Help,    // --help
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Run
    }
}
