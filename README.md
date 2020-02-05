<!--
      /         /         /
 ___ (___  ___ (___  ___ (___
|   )|   )|___)|    |    |   )
|__/ |  / |__  |__  |__  |  /
|
--> <p align="center"> <img src="./img/logo.png"> <br>
<a href="https://git.io/JveQo">
<img src="https://img.shields.io/github/v/release/xvxx/phetch?include_prereleases">
</a>
</p>

`phetch` is a terminal client designed to help you quickly navigate
the gophersphere.

## features

- <1MB executable for Linux and Mac
- Technicolor design (based on [GILD](https://github.com/xvxx/gild))
- No-nonsense keyboard navigation
- Supports Gopher searches, text and menu pages, and downloads
- Save your favorite Gopher sites with bookmarks
- Opt-in history tracking
- Secure Gopher support (TLS)
- Tor support

## usage

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

    Once you've launched phetch, use `ctrl-h` to view the on-line help.

## installation

If you already have a Gopher client, download `phetch` here:

    gopher://phkt.io/1/phetch/latest

On macOS you can install with [Homebrew](https://brew.sh/):

    brew install xvxx/code/phetch

Binaries for Linux, Raspberry Pi, and Mac are available at
https://github.com/xvxx/phetch/releases:

- [phetch-v0.9.0-linux-x86_64.tgz][0]
- [phetch-v0.9.0-linux-armv7.tgz (Raspberry Pi)][1]
- [phetch-v0.9.0-macos.zip][2]

Just unzip/untar the `phetch` program into your $PATH and get going!

You can also build and install from source if you have `cargo`,
`make`, and the other dependencies described in the next section:

    git clone https://github.com/xvxx/phetch
    cd phetch
    env PREFIX=/usr/local make install

## development

To build with TLS support on **Linux**, you need `openssl` and
`pkg-config`:

    sudo apt install -y pkg-config libssl-dev

Regular development uses `cargo`:

    cargo run -- <gopher-url>

*Pro-tip:* Run a local gopher server (like [phd][phd]) on
`127.0.0.1:7070` and start phetch with `-l` or `--local` to quickly
connect to it.

phetch builds with TLS and Tor support by default. To disable these
features, or to enable only one of them, use the
`--no-default-features` flag:

    cargo build --no-default-features

You can check whether TLS is enabled by visiting the About page:

    cargo run --no-default-features -- gopher://phetch/about

To enable just TLS support, or just Tor support, use `--features`:

    cargo run --no-default-features --features tor -- gopher://phetch/about

## screenies

|![DOS Archive](./img/dos.png)|![Floodgap](./img/menu-view.png)|
|:-:|:-:|
| DOS Archive | Floodgap |

## todo

- [ ] catch SIGWINCH

## bugs

- [ ] ctrl-c while telneting kills phetch
- [ ] ctrl-c in load() not yet implemented
- [ ] ctrl-c in download fails to return to listening state 
      because of termion bug:
      https://gitlab.redox-os.org/redox-os/termion/issues/168

## v1.0

- [ ] GIF screencast

## future features

- [ ] track binary size per release
- [ ] text views are menus when URLs are present (one per line max)
- [ ] Find Text in Text views
- [ ] fuzzy find incremental search
- [ ] persistent history
- [ ] bookmarks: toggle instead of just prepending to the file
- [ ] bookmarks: save the title of the current page

[0]: https://github.com/xvxx/phetch/releases/download/v0.9.0/phetch-v0.9.0-linux-x86_64.tgz
[1]: https://github.com/xvxx/phetch/releases/download/v0.9.0/phetch-v0.9.0-linux-armv7.tgz
[2]: https://github.com/xvxx/phetch/releases/download/v0.9.0/phetch-v0.9.0-macos.zip
[phd]: https://github.com/xvxx/phd
