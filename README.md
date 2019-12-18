<p align="center">
    <img src="./phetch.png">
    <br> <br>
    <a href="LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blueviolet?style=flat-square">
    </a>
    <a href="https://github.com/dvkt/phetch/releases/tag/v0.0.0">
        <img src="https://img.shields.io/badge/current_release-0.0.0-brightgreen.svg?style=flat-square">
    </a>
    <a href="https://github.com/dvkt/phetch">
        <img src="https://img.shields.io/badge/dev_version-0.1.0--dev-lightgrey.svg?style=flat-square">
    </a>
</p>

`phetch` is a terminal gopher client designed for quick keyboard navigation. It is the spiritual success to [GILD](https://github.com/dvkt/gild).

## features

- small (<1MB) executable for linux and macos
- technicolor design
- no nonsense keyboard navigation

## usage

    phetch <gopher-url>        # Show GopherHole at URL
    phetch -raw <gopher-url>   # Print raw Gopher response.
    phetch -help               # Show this screen.
    phetch -version            # Show phetch version.

## installation

MacOS:

    wget https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-macos.zip
    unzip phetch-macos.zip
    ./phetch -h

Linux x86_64:

    wget https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-x86-64.zip
    unzip phetch-linux-x86-64.zip
    ./phetch -h

Linux ARM:

    wget https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-arm.zip
    unzip phetch-linux-arm.zip
    ./phetch -h

## development

    cargo run -- <gopher-url>

## resources

- [rfc 1346](https://tools.ietf.org/html/rfc1436)
- http://ascii-table.com/ansi-escape-sequences.php

## TODO

### Basics
- [ ] MENU: up/down scroll when next link out of view
- [ ] MENU: page up/page down show next page, highlight first link
- [ ] status() helper
- [ ] show errors in status()
- [ ] replace all panic! with errors
- [ ] replace all unwrap/expect with errors
- [ ] TLS
- [ ] MENU: open HTML link in browser
- [ ] `?` to show all keyboard shortcuts
- [ ] search functionality
- [ ] download to ~/Downloads
- [ ] save history to file
- [ ] load history from file
- [ ] load most recent URL when opening without args
### Bonus
- [ ] fuzzy find search links
    - https://github.com/stewart/rff
    - https://github.com/Schlechtwetterfront/fuzzy-rs
- [ ] detect SIGWINCH
    - https://github.com/BurntSushi/chan-signal
