use std::{borrow::Cow, io::Result};

/// Encoding of Gopher response. Only UTF8 and CP437 are supported.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl Encoding {
    /// Accepts a string like "UTF8" or "CP437" and returns the
    /// appropriate `Encoding`, or an `Err`.
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_ref() {
            "utf8" | "utf-8" | "utf 8" => Ok(Encoding::UTF8),
            "cp437" | "cp-437" | "cp 437" | "pc8" | "pc-8" | "oem us" | "oem-us" => {
                Ok(Encoding::CP437)
            }
            _ => Err(error!("Expected CP437 or UTF8 encoding")),
        }
    }

    /// Convert `response` into a String according to `encoding`.
    pub fn encode<'res>(&self, response: &'res [u8]) -> Cow<'res, str> {
        if matches!(self, Encoding::CP437) {
            let mut converted = String::with_capacity(response.len());
            for b in response {
                converted.push_str(cp437::convert_byte(b));
            }
            Cow::from(converted)
        } else {
            String::from_utf8_lossy(response)
        }
    }
}
