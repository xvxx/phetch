<p align="center">
    <img src="./img/logo.png">
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

`phetch` is a terminal client designed to quickly navigate the gophersphere.

**features:**

- <1MB executable for linux and mac
- technicolor design (based on [GILD](https://github.com/dvkt/gild))
- no nonsense keyboard navigation

## usage

    phetch                 # Launch and show start page.
    phetch <gopher-url>    # Launch and show Gopherhole at URL.
    phetch -r <gopher-url> # Print raw Gopher response.
    phetch -h              # Show help.
    phetch -v              # Show version.

## installation

Binaries for Linux and Mac are available at https://github.com/dvkt/phetch/releases:

- MacOS: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-macos.zip
- Linux x86_64: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-x86-64.tar.gz
- Linux ARM: https://github.com/dvkt/phetch/releases/download/v0.1.0/phetch-linux-arm.tar.gz

Just unzip/untar the `phetch` program into your $PATH and get going!

## development

    cargo run -- <gopher-url>

## screenies

![Links](./img/links.png)

![DOS Menu](./img/menu.png)

![Game Archive](./img/oldies.png)

## todo

- [ ] bookmarks help file
- [ ] bookmarks no ~/.config/phetch/ error
- [ ] history help file
- [ ] history no ~/.config/phetch/ error
- [ ] new screenshots
- [ ] little GIF screencast in README
- [ ] telnet: gopher://bitreich.org/1/lawn/bbs

## bugs

- [ ] gopher://1436.ninja/1/twit.cgi ("iWritten and performed by Nathaniel" weirdness)

## future features

- [ ] Incremental search in Text views
- [ ] Linked gopher and http links in Text views
- [ ] TLS -- https://dataswamp.org/~solene/2019-03-07-gopher-server-tls.html
- [ ] Fuzzy Find incremental search
