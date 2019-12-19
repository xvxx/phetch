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

`phetch` is a terminal gopher client designed for quick keyboard navigation. It is the spiritual successor to [GILD](https://github.com/dvkt/gild).

## features

- small (<1MB) executable for linux and macos
- technicolor design
- no nonsense keyboard navigation

## usage

    phetch <gopher-url>    # Show GopherHole at URL
    phetch -r <gopher-url> # Print raw Gopher response.
    phetch -h              # Show this screen.
    phetch -v              # Show phetch version.

## installation

`phetch` is available as a standalone executable for MacOS and Linux:

- MacOS: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-macos.zip
- Linux x86_64: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-x86-64.tar.gz
- Linux ARM: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-arm.tar.gz

## development

    cargo run -- <gopher-url>

## resources

- [rfc 1346](https://tools.ietf.org/html/rfc1436)
- http://ascii-table.com/ansi-escape-sequences.php
- http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html

## TODO

### Basics
- [ ] TLS
- [ ] `?` to show all keyboard shortcuts
- [ ] download to ~/Downloads
- [ ] save history to file
- [ ] load history from file
- [ ] load most recent URL when opening without args
- [ ] async gopher request
- [ ] open local files
### Bonus
- [ ] fuzzy find search links
    - https://github.com/stewart/rff
    - https://github.com/Schlechtwetterfront/fuzzy-rs
- [ ] detect SIGWINCH
    - https://github.com/BurntSushi/chan-signal

## Screenies

![Links](./img/links.png)

![DOS Menu](./img/menu.png)

![Game Archive](./img/oldies.png)