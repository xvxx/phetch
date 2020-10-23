<!--
      /         /         /
 ___ (___  ___ (___  ___ (___
|   )|   )|___)|    |    |   )
|__/ |  / |__  |__  |__  |  /
|
--> <p align="center"> <img src="./img/logo.png"> <br>
<a href="https://git.io/JveQo">
<img src="https://img.shields.io/github/v/release/xvxx/phetch">
</a>
<a href="https://crates.io/crates/phetch">
<img src="https://img.shields.io/crates/v/phetch">
</a>
<a href="https://aur.archlinux.org/packages/phetch/">
<img src="https://img.shields.io/aur/version/phetch">
</a>
<a href="https://git.io/JvR5g">
<img src="https://github.com/xvxx/phetch/workflows/build/badge.svg">
</a>
</p>

`phetch` is a terminal client designed to help you quickly navigate
the gophersphere.

<hr>

![demo of phetch in action](img/phetch-demo.gif "demo of phetch")

## features

- <1MB executable for Linux, Mac, and NetBSD
- Technicolor design (based on [GILD](https://github.com/xvxx/gild))
- No-nonsense keyboard navigation
- Supports Gopher searches, text and menu pages, and downloads
- Save your favorite Gopher sites with bookmarks
- Opt-in history tracking
- Secure Gopher support (TLS)
- Tor support

## usage

    Usage:

        phetch [options]       Launch phetch in interactive mode
        phetch [options] url   Open Gopher URL in interactive mode

    Options:

        -s, --tls              Try to open Gopher URLs securely w/ TLS
        -o, --tor              Use local Tor proxy to open all pages
        -S, -O                 Disable TLS or Tor

        -w, --wrap COLUMN      Wrap long lines in "text" views at COLUMN.
        -m, --media PROGRAM    Use to open media files. Default: mpv
        -M, --no-media         Just download media files, don't download

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

On Arch Linux, install phetch with your favorite [AUR helper][aur]:

    yay phetch

On NetBSD, phetch is included in the main pkgsrc repo:

    pkgin install phetch

Binaries for Linux, Raspberry Pi, and Mac are available at
https://github.com/xvxx/phetch/releases:

- [phetch-v1.0.7-linux-x86_64.tgz][0]
- [phetch-v1.0.7-linux-armv7.tgz (Raspberry Pi)][1]
- [phetch-v1.0.7-macos.zip][2]

Just unzip/untar the `phetch` program into your `$PATH` and get going!

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

_Pro-tip:_ Run a local gopher server (like [phd]) on `0.0.0.0:7070`
and start phetch with `-l` or `--local` to quickly connect to it.
Useful for debugging!

phetch builds with TLS and Tor support by default. To disable these
features, or to enable only one of them, use the
`--no-default-features` flag:

    cargo build --no-default-features

You can check whether TLS is enabled by visiting the About page:

    cargo run --no-default-features -- gopher://phetch/about

To enable just TLS support, or just Tor support, use `--features`:

    cargo run --no-default-features --features tor -- gopher://phetch/about

## media player support

phetch includes support for opening video files (`;` item type) and
sound files (`s` item type) in [mpv] or an application of your choice
using the `-m` command line flag. To test it out, visit a compatible
Gopher server (maybe one using [Gophor]?). Or check out the "gopher
types" help page by pressing `ctrl-h` then `3` in phetch.

## todo

- [ ] ctrl-c in load() not yet implemented

## bugs

- [ ] telnet IO seems broken after raw_input change (1146f42)

## future features

- [ ] track binary size per release
- [ ] text views are menus when URLs are present (one per line max)
- [ ] Find Text in Text views
- [ ] fuzzy find incremental search
- [ ] persistent history
- [ ] bookmarks: toggle instead of just prepending to the file
- [ ] bookmarks: save the title of the current page

[0]: https://github.com/xvxx/phetch/releases/download/v1.0.7/phetch-v1.0.7-linux-x86_64.tgz
[1]: https://github.com/xvxx/phetch/releases/download/v1.0.7/phetch-v1.0.7-linux-armv7.tgz
[2]: https://github.com/xvxx/phetch/releases/download/v1.0.7/phetch-v1.0.7-macos.zip
[phd]: https://github.com/xvxx/phd
[aur]: https://wiki.archlinux.org/index.php/AUR_helpers
[mpv]: https://github.com/mpv-player/mpv
[gophor]: https://github.com/grufwub/gophor
