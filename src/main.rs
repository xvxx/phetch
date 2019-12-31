use phetch::{gopher, ui::UI};
use std::process::exit;

fn main() {
    exit(run())
}

fn run() -> i32 {
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
            return 0;
        } else {
            eprintln!("--raw needs gopher-url");
            return 1;
        }
    }

    if url == "--version" || url == "-v" || url == "-version" {
        print_version();
        return 0;
    }

    if url == "--help" || url == "-h" || url == "-help" {
        print_usage();
        return 0;
    }

    if !url.is_empty() && url.starts_with('-') {
        eprintln!("unknown flag: {}\n", url);
        print_usage();
        return 1;
    }

    let mut ui = UI::new();
    if let Err(e) = ui.open(url, url) {
        eprintln!("{}", e);
        return 1;
    }
    ui.run();
    return 0;
}

fn print_version() {
    println!("phetch v{}", phetch::VERSION);
}

fn print_usage() {
    println!(
        "Usage:
    phetch                           launch and show start page
    phetch <gopher-url>              open gopherhole at url
    phetch -r, --raw <gopher-url>    print raw gopher response
    phetch -h, --help                show this screen
    phetch -v, --version             show phetch version

Once you've launched phetch, use `ctrl-h` to view the on-line help."
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
