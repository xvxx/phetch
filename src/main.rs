#![allow(unused_must_use)]

extern crate termion;

mod fetch;
mod gopher;
mod menu;
mod types;
mod ui;

use gopher::Type;

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
    let mut ui = ui::UI::new();
    let url = format!("{}:{}/1/{}", host, port, selector);
    ui.load(url);
    ui.run();
}

fn print_usage() {
    println!("\x1B[93;1musage:\x1B[0m phetch <gopher-url>        # Show GopherHole at URL");
    println!("       phetch -raw <gopher-url>   # Print raw Gopher response.");
    println!("       phetch -help               # Show this screen.");
    println!("       phetch -version            # Show phetch version.");
}
