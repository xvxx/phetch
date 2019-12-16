#![allow(unused_must_use)]

extern crate termion;
use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod fetch;
mod page;
mod types;
mod ui;
use ui::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    let host = args.get(1).unwrap();
    let port = "70".to_string();
    let port = args.get(2).unwrap_or(&port);
    let selector = "/".to_string();
    let selector = args.get(3).unwrap_or(&selector);
    if host == "--help" || host == "-h" || host == "-help" {
        print_usage();
        return;
    }

    let mut ui = UI::new();
    ui.load(host, port, selector);
    ui.run();
}

fn print_usage() {
    println!("\x1B[93;1musage:\x1B[0m ");
    println!("\t$ phetch host [port [selector]]");
}
