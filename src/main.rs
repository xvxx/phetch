extern crate phetch;

use phetch::gopher;
use phetch::ui::UI;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = if args.len() < 2 {
        "gopher://phetch/1/home"
    } else {
        args.get(1).unwrap()
    };

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

    let mut ui = UI::new();
    if let Err(e) = ui.open(url, url) {
        eprintln!("{}", e);
        exit(1);
    }
    ui.run();
}

fn print_version() {
    println!("phetch v{}", phetch::VERSION);
}

fn print_usage() {
    println!(
        "Usage: 
    phetch                           # Launch and show start page.
    phetch <gopher-url>              # Open Gopherhole at URL.
    phetch -r, --raw <gopher-url>    # Print raw Gopher response.
    phetch -h, --help                # Show this screen.
    phetch -v, --version             # Show phetch version."
    );
}

fn print_raw(url: &str) {
    let _ = gopher::fetch_url(url)
        .and_then(|x| {
            println!("{}", x);
            Ok(())
        })
        .map_err(|e| {
            eprintln!("{}", e);
            exit(1);
        });
}
