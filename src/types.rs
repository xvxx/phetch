#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text = '0' as isize,        // 0
    Menu,                       // 1
    CSOEntity,                  // 2
    Error,                      // 3
    Binhex,                     // 4
    DOSFile,                    // 5
    UUEncoded,                  // 6
    Search,                     // 7
    Telnet,                     // 8
    Binary,                     // 9
    Mirror = '+' as isize,      // +
    GIF = 'g' as isize,         // g
    Telnet3270 = 'T' as isize,  // T
    HTML = 'h' as isize,        // h
    Information = 'i' as isize, // i
    Sound = 's' as isize,       // s
    Document = 'd' as isize,    // d
}
