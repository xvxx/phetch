/// Encoding of Gopher response. Only UTF8 and CP437 are supported.
#[derive(Copy, Clone)]
pub enum Encoding {
    /// Unicode
    UTF8,
    /// https://en.wikipedia.org/wiki/Code_page_437
    CP437,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::UTF8
    }
}
