/// The mode our text UI is in. Run mode is the default while
/// printing doesn't show the cursor, among other things.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    Run,
    Print,
    NoTTY,
    Raw,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Run
    }
}
