use phetch::{gopher, ui::UI};
use std::process::exit;

fn main() {
    exit(run())
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    let mut url = "gopher://phetch/1/home";
    let mut praw = false;
    let mut tls = false;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_ref() {
            "-v" | "--version" | "-version" => {
                print_version();
                return 0;
            }
            "-h" | "--help" | "-help" => {
                print_usage();
                return 0;
            }
            "-r" | "--raw" | "-raw" => {
                if args.len() > 2 {
                    praw = true;
                } else {
                    eprintln!("--raw needs gopher-url");
                    return 1;
                }
            }
            "-l" | "--local" | "-local" => url = "gopher://127.0.0.1:7070",
            "-t" | "--tls" | "-tls" => tls = true,
            arg => {
                if arg.starts_with('-') {
                    eprintln!("unknown flag: {}\n", url);
                    print_usage();
                    return 1;
                } else {
                    url = arg;
                }
            }
        }
    }

    if praw {
        print_raw(url, tls);
        return 0;
    }

    let mut ui = UI::new(tls);
    if let Err(e) = ui.open(url, url).and_then(|_| ui.run()) {
        eprintln!("{}", e);
        return 1;
    }
    0
}

fn print_version() {
    println!("phetch v{}", phetch::VERSION);
}

fn print_usage() {
    println!(
        "Usage:
    phetch                           launch and show start page
    phetch <gopher-url>              open gopherhole at url
    phetch -t, --tls <gopher-url>    attempt to open w/ tls
    phetch -r, --raw <gopher-url>    print raw gopher response
    phetch -l, --local               connect to 127.0.0.1:7070
    phetch -h, --help                show this screen
    phetch -v, --version             show phetch version

Once you've launched phetch, use `ctrl-h` to view the on-line help."
    );
}

fn print_raw(url: &str, try_tls: bool) {
    match gopher::fetch_url(url, try_tls) {
        Ok(response) => println!("{}", response),
        Err(e) => {
            eprintln!("{}", e);
            exit(0)
        }
    }
}
