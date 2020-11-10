/// Encoding of Gopher response. Only UTF8 and CP437 are supported.
pub(crate) enum Encoding {
    /// Unicode
    UTF8,
    /// https://en.wikipedia.org/wiki/Code_page_437
    CP437,
}
