use phetch::{
    args, color, gopher, menu, terminal,
    ui::{Mode, UI},
};
use std::{
    env,
    error::Error,
    io::{self, stdout, Write},
    panic, process, str,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

/// Start the app. Returns UNIX exit code.
fn run() -> Result<(), Box<dyn Error>> {
    let str_args = env::args().skip(1).collect::<Vec<String>>();
    let mut cfg = args::parse(&str_args)?;

    // check for simple modes
    match cfg.mode {
        Mode::Raw => return print_raw(&cfg.start, cfg.tls, cfg.tor),
        Mode::Version => return print_version(),
        Mode::Help => return print_usage(),
        Mode::NoTTY => return print_plain(&cfg.start, cfg.tls, cfg.tor),
        Mode::Print => cfg.wide = true,
        Mode::Run => {}
    }

    // load url
    let start = cfg.start.clone();
    let mode = cfg.mode;
    let mut ui = UI::new(cfg);
    ui.open(&start, &start)?;

    // print rendered version
    if mode == Mode::Print {
        println!("{}", ui.render()?);
        return Ok(());
    }

    // run app
    setup_terminal();
    ui.run()?;
    cleanup_terminal();

    Ok(())
}

/// --version
fn print_version() -> Result<(), Box<dyn Error>> {
    println!(
        "phetch v{version} ({built})",
        built = phetch::BUILD_DATE,
        version = phetch::VERSION
    );
    Ok(())
}

/// --help
fn print_usage() -> Result<(), Box<dyn Error>> {
    print_version()?;
    println!(
        "
Usage:

    phetch [options]       Launch phetch in interactive mode
    phetch [options] url   Open Gopher URL in interactive mode

Options:

    -s, --tls              Try to open Gopher URLs securely w/ TLS
    -o, --tor              Use local Tor proxy to open all pages
    -S, -O                 Disable TLS or Tor

    -w, --wrap COLUMN      Wrap long lines in \"text\" views at COLUMN.
    -m, --media PROGRAM    Use to open media files. Default: mpv
    -M, --no-media         Just download media files, don't download

    -a, --autoplay         Autoplay media without prompting.
    -A, --no-autoplay      Prompt before playing media.

    -r, --raw              Print raw Gopher response only
    -p, --print            Print rendered Gopher response only
    -l, --local            Connect to 127.0.0.1:7070
    -e, --encoding         Render text documents in CP437 or UTF8.

    -c, --config FILE      Use instead of ~/.config/phetch/phetch.conf
    -C, --no-config        Don't use any config file

    -h, --help             Show this screen
    -v, --version          Show phetch version

Command line options always override options set in phetch.conf.

Once you've launched phetch, use `ctrl-h` to view the on-line help."
    );
    Ok(())
}

/// Print just the raw Gopher response.
fn print_raw(url: &str, tls: bool, tor: bool) -> Result<(), Box<dyn Error>> {
    let (_, out) = gopher::fetch_url(url, tls, tor)?;
    println!("{}", gopher::response_to_string(&out));
    Ok(())
}

/// Print a colorless, plain version of the response for a non-tty
/// (like a pipe).
fn print_plain(url: &str, tls: bool, tor: bool) -> Result<(), Box<dyn Error>> {
    let mut out = String::new();
    let typ = gopher::type_for_url(url);
    let (_, response) = gopher::fetch_url(url, tls, tor)?;
    let response = gopher::response_to_string(&response);
    match typ {
        gopher::Type::Menu => {
            let menu = menu::parse(url, response);
            for line in menu.lines() {
                out.push_str(line.text());
                out.push('\n');
            }
        }
        gopher::Type::Text => println!("{}", response.trim_end_matches(".\r\n")),
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("can't print gopher type: {:?}", typ),
            )));
        }
    };
    print!("{}", out);
    Ok(())
}

/// Put the terminal into raw mode, enter the alternate screen, and
/// setup the panic handler.
fn setup_terminal() {
    let old_handler = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        cleanup_terminal();
        old_handler(info);
    }));

    terminal::enable_raw_mode().expect("Fatal Error entering Raw Mode.");
    write!(stdout(), "{}", terminal::ToAlternateScreen)
        .expect("Fatal Error entering Alternate Mode.");
}

/// Leave raw mode. Need to always do this, even on panic.
fn cleanup_terminal() {
    let mut stdout = stdout();
    write!(
        stdout,
        "{}{}{}{}{}",
        color::Reset,
        terminal::ClearAll,
        terminal::Goto(1, 1),
        terminal::ShowCursor,
        terminal::ToMainScreen
    )
    .expect("Fatal Error cleaning up terminal.");
    stdout.flush().expect("Fatal Error cleaning up terminal.");
    terminal::disable_raw_mode().expect("Fatal Error leaving Raw Mode.");
}
