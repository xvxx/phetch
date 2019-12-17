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

    let url = args.get(1).unwrap();

    if url == "--version" || url == "-v" || url == "-version" {
        print_version();
        return;
    }
    if url == "--help" || url == "-h" || url == "-help" {
        print_usage();
        return;
    }

    let mut ui = ui::UI::new();
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
