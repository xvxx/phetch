#![allow(unused_must_use)]
#![allow(unused_imports)]

extern crate termion;

use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod page;
mod types;
mod ui;
use ui::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }
    let host = args.get(1).unwrap();
    let port = String::from("70");
    let selector = String::from("/");
    let port = args.get(2).unwrap_or(&port);
    let selector = args.get(3).unwrap_or(&selector);
    if host == "--help" || host == "-h" || host == "-help" {
        usage();
        return;
    }
}

fn usage() {
    println!("\x1B[93;1musage:\x1B[0m ");
    println!("\t$ phetch host [port [selector]]");
}
