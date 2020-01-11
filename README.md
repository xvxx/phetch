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

        phetch [options]          Launch phetch in interactive mode
        phetch [options] [url]    Open Gopher URL in interactive mode

    Options:

        -s, --tls                 Try to open Gopher URLs securely w/ TLS
        -o, --tor                 Use local Tor proxy to open all pages
                                  Set the TOR_PROXY env variable to use
                                  an address other than the default :9050
        -r, --raw                 Print raw Gopher response only
        -p, --print               Print rendered Gopher response only
        -l, --local               Connect to 127.0.0.1:7070

        -h, --help                Show this screen
        -v, --version             Show phetch version

    Once you've launched phetch, use `ctrl-h` to view the on-line help.

## installation

On macOS you can install with [Homebrew](https://brew.sh/):

    brew install xvxx/code/phetch

Binaries for Linux, Raspberry Pi, and Mac are available at
https://github.com/xvxx/phetch/releases:

- [phetch-v0.1.12-linux-x86_64.tgz][0]
- [phetch-v0.1.12-linux-armv7.tgz (Raspberry Pi)][1]
- [phetch-v0.1.12-macos.zip][2]

Just unzip/untar the `phetch` program into your $PATH and get going!

You can also build and install from source:

    git clone https://github.com/xvxx/phetch
    cd phetch
    env PREFIX=/usr/local make install

## updates

To check for new versions of `phetch`, use the on-line help system in
the app (`ctrl-h`) or visit:

    gopher://phkt.io/1/phetch/latest

## development

    cargo run -- <gopher-url>

*Pro-tip:* Run a local gopher server on `127.0.0.1:7070` and start
phetch with `-l` or `--local` to quickly connect to it.

To build with TLS support on **Linux**, you need `openssl` and
`pkg-config`:

    sudo apt install -y pkg-config libssl-dev

To build without TLS support, build with the `disable-tls` feature:

    cargo build --features disable-tls

You can check whether TLS is enabled by visiting the About page:

    cargo run --features disable-tls -- gopher://phetch/about

## screenies

|![DOS Archive](./img/dos.png)|![Floodgap](./img/menu-view.png)|
|:-:|:-:|
| DOS Archive | Floodgap |

## bugs

- [ ] unknown keypress: [ during status messages
- [ ] new status doesn't replace old (download complete -> copy url)

## v1.0

- [ ] Changelog generation (for gopher and github)
- [ ] GIF screencast

## future features

- [ ] track binary size per release
- [ ] text views are menus when URLs are present (one per line max)
- [ ] Find Text in Text views
- [ ] fuzzy find incremental search
- [ ] persistent history
- [ ] bookmarks: toggle instead of just prepending to the file
- [ ] bookmarks: save the title of the current page

[0]: https://github.com/xvxx/phetch/releases/download/v0.1.12/phetch-v0.1.12-linux-x86_64.tgz
[1]: https://github.com/xvxx/phetch/releases/download/v0.1.12/phetch-v0.1.12-linux-armv7.tgz
[2]: https://github.com/xvxx/phetch/releases/download/v0.1.12/phetch-v0.1.12-macos.zip
