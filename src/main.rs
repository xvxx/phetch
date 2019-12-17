#![allow(unused_must_use)]

extern crate termion;

mod fetch;
mod gopher;
mod menu;
mod types;
mod ui;

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
    if host == "--version" || host == "-v" || host == "-version" {
        print_version();
        return;
    }
    if host == "--help" || host == "-h" || host == "-help" {
        print_usage();
        return;
    }
    let mut ui = ui::UI::new();
    let url = format!("{}:{}{}", host, port, selector); // TODO: url on cmdline
    ui.load(url);
    ui.run();
}

fn print_version() {
    println!("\x1b[93;1mphetch v0.0.1-dev\x1b[m");
}

fn print_usage() {
    println!(
        "\x1B[93;1mUsage:\x1B[0m 
    phetch <gopher-url>        # Show GopherHole at URL
    phetch -raw <gopher-url>   # Print raw Gopher response.
    phetch -help               # Show this screen.
    phetch -version            # Show phetch version."
    );
}
