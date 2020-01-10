use phetch::{config, gopher, ui::UI};
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
    let mut cfg = if config::exists() {
        match config::load() {
            Err(e) => {
                eprintln!("Config error: {}", e.into_inner().unwrap());
                return 1;
            }
            Ok(c) => c,
        }
    } else {
        config::default()
    };

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut mode = Mode::Run;
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
            "-l" | "--local" | "-local" => cfg.start = "gopher://127.0.0.1:7070".into(),
            "-t" | "--tls" | "-tls" => {
                cfg.tls = true;
                if cfg!(feature = "disable-tls") {
                    eprintln!("phetch was compiled without TLS support");
                    return 1;
                }
            }
            arg => {
                if arg.starts_with('-') {
                    print_version();
                    eprintln!("unknown flag: {}", arg);
                    return 1;
                } else if got_url {
                    print_version();
                    eprintln!("unknown argument: {}", arg);
                    return 1;
                } else {
                    got_url = true;
                    cfg.start = arg.into();
                }
            }
        }
    }

    if mode == Mode::Raw {
        print_raw(&cfg.start, cfg.tls);
        return 0;
    }

    let mut ui = UI::new(cfg.tls);
    if let Err(e) = ui.open(&cfg.start, &cfg.start) {
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
    println!(
        "phetch - quick lil gopher client (v{version} - {built})",
        built = phetch::BUILD_DATE,
        version = phetch::VERSION
    );
}

fn print_usage() {
    print_version();
    println!(
        "
Usage: 

    phetch [options]          Launch phetch in interactive mode
    phetch [options] [url]    Open Gopher URL in interactive mode

Options:

    -t, --tls                 Try to open all pages w/ TLS
    -r, --raw                 Print raw Gopher response only
    -p, --print               Print rendered Gopher response only
    -l, --local               Connect to 127.0.0.1:7070

    -h, --help                Show this screen
    -v, --version             Show phetch version

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
