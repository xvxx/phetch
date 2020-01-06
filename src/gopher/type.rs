#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text,       // 0 | 96 | cyan
    Menu,       // 1 | 94 | blue
    CSOEntity,  // 2 |    | white background
    Error,      // 3 | 91 | red
    Binhex,     // 4 |  4 | white underline
    DOSFile,    // 5 |  4 | white underline
    UUEncoded,  // 6 |  4 | white underline
    Search,     // 7 |  0 | white
    Telnet,     // 8 | 90 | gray underline
    Binary,     // 9 |  4 | white underline
    Mirror,     // + |    | white background
    GIF,        // g |  4 | white underline
    Telnet3270, // T |    | white background
    HTML,       // h | 92 | green
    Image,      // I |  4 | white underline
    PNG,        // p |  4 | white underline
    Info,       // i | 93 | yellow
    Sound,      // s |  4 | white underline
    Document,   // d |  4 | white underline
}

impl Type {
    /// Is this an info line?
    pub fn is_info(self) -> bool {
        self == Type::Info
    }

    /// Is this a link, ie something we can navigate to or open?
    pub fn is_link(self) -> bool {
        match self {
            Type::Menu | Type::Search | Type::Telnet | Type::HTML => true,
            e if e.is_download() => true,
            _ => false,
        }
    }

    /// Is this something we can download?
    pub fn is_download(self) -> bool {
        match self {
            Type::Binhex
            | Type::DOSFile
            | Type::UUEncoded
            | Type::Binary
            | Type::GIF
            | Type::Image
            | Type::PNG
            | Type::Sound
            | Type::Document => true,
            _ => false,
        }
    }

    /// Is this a type phetch supports?
    pub fn is_supported(self) -> bool {
        match self {
            Type::CSOEntity | Type::Mirror | Type::Telnet3270 => false,
            _ => true,
        }
    }

    /// Gopher Item Type to RFC char.
    pub fn to_char(self) -> Option<char> {
        Some(match self {
            Type::Text => '0',
            Type::Menu => '1',
            Type::CSOEntity => '2',
            Type::Error => '3',
            Type::Binhex => '4',
            Type::DOSFile => '5',
            Type::UUEncoded => '6',
            Type::Search => '7',
            Type::Telnet => '8',
            Type::Binary => '9',
            Type::Mirror => '+',
            Type::GIF => 'g',
            Type::Telnet3270 => 'T',
            Type::HTML => 'h',
            Type::Image => 'I',
            Type::PNG => 'p',
            Type::Info => 'i',
            Type::Sound => 's',
            Type::Document => 'd',
        })
    }

    /// Create a Gopher Item Type from its RFC char code.
    pub fn from(c: char) -> Option<Type> {
        Some(match c {
            '0' => Type::Text,
            '1' => Type::Menu,
            '2' => Type::CSOEntity,
            '3' => Type::Error,
            '4' => Type::Binhex,
            '5' => Type::DOSFile,
            '6' => Type::UUEncoded,
            '7' => Type::Search,
            '8' => Type::Telnet,
            '9' => Type::Binary,
            '+' => Type::Mirror,
            'g' => Type::GIF,
            'T' => Type::Telnet3270,
            'h' => Type::HTML,
            'I' => Type::Image,
            'p' => Type::PNG,
            'i' => Type::Info,
            's' => Type::Sound,
            'd' => Type::Document,
            _ => return None,
        })
    }
}
