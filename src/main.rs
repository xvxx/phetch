use phetch::{
    args, gopher,
    ui::{Mode, UI},
};
use std::{env, process};

fn main() {
    process::exit(run())
}

/// Start the app. Returns UNIX exit code.
fn run() -> i32 {
    let str_args = env::args().skip(1).collect::<Vec<String>>();
    let mut cfg = match args::parse(&str_args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    // check for simple modes
    match cfg.mode {
        Mode::Raw => return print_raw(&cfg.start, cfg.tls, cfg.tor),
        Mode::Version => return print_version(),
        Mode::Help => return print_usage(),
        Mode::NoTTY => return print_plain(&cfg.start, cfg.tls, cfg.tor),
        Mode::Print => cfg.wide = true,
        _ => {}
    }

    // load url
    let start = cfg.start.clone();
    let mode = cfg.mode;
    let mut ui = UI::new(cfg);
    if let Err(e) = ui.open(&start, &start) {
        eprintln!("{}", e);
        return 1;
    }

    // print rendered version
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

    // run app
    if let Err(e) = ui.run() {
        eprintln!("{}", e);
        return 1;
    }

    // and scene
    0
}

/// --version
fn print_version() -> i32 {
    println!(
        "phetch - quick lil gopher client (v{version} - {built})",
        built = phetch::BUILD_DATE,
        version = phetch::VERSION
    );
    0
}

/// --help
fn print_usage() -> i32 {
    print_version();
    println!(
        "
Usage:

    phetch [options]       Launch phetch in interactive mode
    phetch [options] url   Open Gopher URL in interactive mode

Options:

    -s, --tls              Try to open Gopher URLs securely w/ TLS
    -o, --tor              Use local Tor proxy to open all pages
    -S, -O                 Disable TLS or Tor
                              
    -r, --raw              Print raw Gopher response only
    -p, --print            Print rendered Gopher response only
    -l, --local            Connect to 127.0.0.1:7070

    -c, --config FILE      Use instead of ~/.config/phetch/phetch.conf
    -C, --no-config        Don't use any config file            
    
    -h, --help             Show this screen
    -v, --version          Show phetch version

Command line options always override options set in phetch.conf.

Once you've launched phetch, use `ctrl-h` to view the on-line help."
    );
    0
}

/// Print just the raw Gopher response.
fn print_raw(url: &str, tls: bool, tor: bool) -> i32 {
    match gopher::fetch_url(url, tls, tor) {
        Ok((_, response)) => {
            println!("{}", response);
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

/// Print a colorless, plain version of the response for a non-tty
/// (like a pipe).
fn print_plain(url: &str, tls: bool, tor: bool) -> i32 {
    let mut out = String::new();
    let (typ, _, _, _) = gopher::parse_url(url);
    match gopher::fetch_url(url, tls, tor) {
        Ok((_, response)) => match typ {
            gopher::Type::Menu => {
                // TODO use parse_line()
                for line in response.trim_end_matches(".\r\n").lines() {
                    let line = line.trim_end_matches('\r');
                    if let Some(desc) = line.splitn(2, '\t').nth(0) {
                        let desc = desc.trim();
                        out.push_str(&desc[1..]);
                        out.push('\n');
                    }
                }
            }
            gopher::Type::Text => println!("{}", response.trim_end_matches(".\r\n")),
            _ => {
                eprintln!("can't print gopher type: {:?}", typ);
                return 1;
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }
    print!("{}", out);
    0
}
