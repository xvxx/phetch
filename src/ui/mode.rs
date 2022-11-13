/// The mode our text UI is in. Run mode is the default while
/// Print doesn't show the cursor, among other things.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    /// Default, interactive mode.
    ///   phetch URL
    Run,

    /// Just print a rendered version of a URL.
    ///   phetch --print URL
    Print,
    /// NoTTY, ie in a UNIX pipeline.
    ///   phetch --print URL | cat
    NoTTY,
    /// Just print raw Gopher response.
    ///   phetch --raw URL
    Raw,
    /// Show version info.
    ///   phetch --version
    Version,
    /// Show command line help.
    ///   phetch --help
    Help,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Run
    }
}
