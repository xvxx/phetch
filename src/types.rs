#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
    Text,          // 0
    Menu,          // 1
    CSOEntity,     // 2
    Error,         // 3
    BinhexFile,    // 4
    DOSFile,       // 5
    UUEncodedFile, // 6
    Search,        // 7
    Telnet,        // 8
    BinaryFile,    // 9
    Mirror,        // +
    GIF,           // g
    Image,         // i
    Telnet3270,    // T
    HTMLFile,      // h
    Information,   // i
    Sound,         // s
    Document,      // d
}
