use phetch::{gopher, ui::UI};
use std::process::exit;

#[derive(PartialEq)]
enum Mode {
    Run,
    Print,
    Raw,
}

fn main() {
    exit(run())
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut url = "gopher://phetch/1/home";
    let mut mode = Mode::Run;
    let mut tls = false;
    let mut iter = args.iter();
    let mut got_url = false;
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
                if args.len() > 1 {
                    mode = Mode::Raw;
                } else {
                    eprintln!("--raw needs gopher-url");
                    return 1;
                }
            }
            "-p" | "--print" | "-print" => {
                mode = Mode::Print;
            }
            "-l" | "--local" | "-local" => url = "gopher://127.0.0.1:7070",
            "-t" | "--tls" | "-tls" => {
                tls = true;
                if cfg!(feature = "disable-tls") {
                    eprintln!("phetch was compiled without TLS support");
                    return 1;
                }
            }
            arg => {
                if arg.starts_with('-') {
                    eprintln!("unknown flag: {}", url);
                    return 1;
                } else if got_url {
                    eprintln!("unknown argument: {}", url);
                    return 1;
                } else {
                    got_url = true;
                    url = arg;
                }
            }
        }
    }

    if mode == Mode::Raw {
        print_raw(url, tls);
        return 0;
    }

    let mut ui = UI::new(tls);
    if let Err(e) = ui.open(url, url) {
        eprintln!("{}", e);
        return 1;
    }

    if mode == Mode::Print {
        return match ui.render() {
            Ok(screen) => {
                println!("{}", screen);
                0
            }
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        };
    }

    if let Err(e) = ui.run() {
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
    phetch -t, --tls <gopher-url>    try to open all pages w/ tls
    phetch -r, --raw <gopher-url>    print raw gopher response
    phetch -p, --print <gopher-url>  print rendered gopher response
    phetch -l, --local               connect to 127.0.0.1:7070
    phetch -h, --help                show this screen
    phetch -v, --version             show phetch version

Once you've launched phetch, use `ctrl-h` to view the on-line help."
    );
}

fn print_raw(url: &str, try_tls: bool) {
    match gopher::fetch_url(url, try_tls) {
        Ok((_, response)) => println!("{}", response),
        Err(e) => {
            eprintln!("{}", e);
            exit(0)
        }
    }
}
