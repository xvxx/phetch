#![allow(unused_must_use)]

extern crate termion;

mod gopher;
mod help;
mod menu;
mod text;
mod ui;
use std::process::exit;
use ui::UI;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let url = args.get(1).unwrap();

    if url == "--raw" || url == "-r" || url == "-raw" {
        if args.len() > 2 {
            let url = args.get(2).unwrap();
            print_raw(url);
        } else {
            eprintln!("--raw needs gopher-url");
            exit(1);
        }
        return;
    }

    if url == "--version" || url == "-v" || url == "-version" {
        print_version();
        return;
    }

    if url == "--help" || url == "-h" || url == "-help" {
        print_usage();
        return;
    }

    if !url.is_empty() && url.starts_with('-') {
        eprintln!("unknown flag: {}\n", url);
        print_usage();
        exit(1);
    }

    let url = if url.starts_with("gopher://") {
        url.to_string()
    } else {
        format!("gopher://{}", url)
    };

    let mut ui = UI::new();
    if let Err(e) = ui.open(&url) {
        ui::error(&e.to_string());
        exit(1);
    }
    ui.run();
}

fn print_version() {
    println!("\x1b[93;1mphetch v0.0.1-dev\x1b[m");
}

fn print_usage() {
    println!(
        "\x1B[93;1mUsage:\x1B[0m 
    phetch <gopher-url>              # Open Gopherhole at URL.
    phetch -r, --raw <gopher-url>    # Print raw Gopher response.
    phetch -h, --help                # Show this screen.
    phetch -v, --version             # Show phetch version."
    );
}

fn print_raw(url: &str) {
    let url = if url.starts_with("gopher://") {
        url.to_string()
    } else {
        format!("gopher://{}", url)
    };

    gopher::fetch_url(&url)
        .and_then(|x| {
            println!("{}", x);
            Ok(())
        })
        .map_err(|e| {
            eprintln!("{}", e);
            exit(1);
        });
}
