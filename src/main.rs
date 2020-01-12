use atty;
use phetch::{
    config, gopher,
    ui::{Mode, UI},
};
use std::process::exit;

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
    let mut set_tls = false;
    let mut set_notls = false;
    let mut set_tor = false;
    let mut set_notor = false;
    let mut set_cfg = false;
    let mut set_nocfg = false;
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
            "-C" => {
                if set_cfg {
                    eprintln!("can't mix --config and --no-config");
                    return 1;
                }
                set_nocfg = true;
                cfg = config::default();
            }
            "-c" | "--config" | "-config" => {
                if set_nocfg {
                    eprintln!("can't mix --config and --no-config");
                    return 1;
                }
                set_cfg = true;
                if let Some(arg) = iter.next() {
                    cfg = match config::load_file(arg) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("error loading config: {}", e);
                            return 1;
                        }
                    };
                } else {
                    eprintln!("need a config file");
                    return 1;
                }
            }
            arg if arg.starts_with("--config=") || arg.starts_with("-config=") => {
                if set_nocfg {
                    eprintln!("can't mix --config and --no-config");
                    return 1;
                }
                set_cfg = true;
                let mut parts = arg.splitn(2, '=');
                if let Some(file) = parts.nth(1) {
                    cfg = match config::load_file(file) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("error loading config: {}", e);
                            return 1;
                        }
                    };
                } else {
                    eprintln!("need a config file");
                    return 1;
                }
            }
            "-s" | "--tls" | "-tls" => {
                if set_notls {
                    eprintln!("can't set both --tls and --no-tls");
                    return 1;
                }
                set_tls = true;
                cfg.tls = true;
                if cfg!(feature = "disable-tls") {
                    eprintln!("phetch was compiled without TLS support");
                    return 1;
                }
            }
            "-S" | "--no-tls" | "-no-tls" => {
                if set_tls {
                    eprintln!("can't set both --tls and --no-tls");
                    return 1;
                }
                set_notls = true;
                cfg.tls = false;
            }
            "-o" | "--tor" | "-tor" => {
                if set_notor {
                    eprintln!("can't set both --tor and --no-tor");
                    return 1;
                }
                set_tor = true;
                cfg.tor = true;
            }
            "-O" | "--no-tor" | "-no-tor" => {
                if set_tor {
                    eprintln!("can't set both --tor and --no-tor");
                    return 1;
                }
                set_notor = true;
                cfg.tor = false;
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

    if cfg.tor && cfg.tls {
        eprintln!("can't set both --tor and --tls.");
        return 1;
    }

    if mode == Mode::Raw {
        print_raw(&cfg.start, cfg.tls, cfg.tor);
        return 0;
    }

    if mode == Mode::Print && !atty::is(atty::Stream::Stdout) {
        // not a tty so print an almost-raw version of the response
        return print_plain(&cfg.start, cfg.tls, cfg.tor);
    }

    if mode == Mode::Print {
        cfg.cursor = false;
        cfg.wide = true;
    }

    let start = cfg.start.clone();
    cfg.mode = mode;
    let mut ui = UI::new(cfg);
    if let Err(e) = ui.open(&start, &start) {
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
}

fn print_raw(url: &str, tls: bool, tor: bool) {
    match gopher::fetch_url(url, tls, tor) {
        Ok((_, response)) => println!("{}", response),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
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
                    if let Some(desc) = line.splitn(2, "\t").nth(0) {
                        let desc = desc.trim();
                        out.push_str(&desc.chars().skip(1).collect::<String>());
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
