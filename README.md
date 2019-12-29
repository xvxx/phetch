<!--
      /         /         /
 ___ (___  ___ (___  ___ (___
|   )|   )|___)|    |    |   )
|__/ |  / |__  |__  |__  |  /
|
--> <p align="center"> <img src="./img/logo.png"> </p>

`phetch` is a terminal client designed to help you quickly navigate the gophersphere.

## features

- <1MB executable for linux and mac
- technicolor design (based on [GILD](https://github.com/dvkt/gild))
- no nonsense keyboard navigation
- supports gopher searches, text and menu pages, and downloads
- save your favorite gopherholes with bookmarks
- opt-in history

## usage

    phetch                           launch and show start page
    phetch <gopher-url>              open gopherhole at url
    phetch -r, --raw <gopher-url>    print raw gopher response
    phetch -h, --help                show this screen
    phetch -v, --version             show phetch version

    Once you've launched phetch, use `ctrl-h` to view the on-line help.

## installation

Binaries for Linux, Mac, and Raspberry Pi are available at https://github.com/dvkt/phetch/releases:

- [phetch-v0.1.4-linux-x86_64.tar.gz](https://github.com/dvkt/phetch/releases/download/v0.1.4/phetch-v0.1.4-linux-x86_64.tar.gz)
- [phetch-v0.1.4-linux-armv7.tar.gz (RPi)](https://github.com/dvkt/phetch/releases/download/v0.1.4/phetch-v0.1.4-linux-armv7.tar.gz)
- [phetch-v0.1.4-macos.zip](https://github.com/dvkt/phetch/releases/download/v0.1.4/phetch-v0.1.4-macos.zip)

Just unzip/untar the `phetch` program into your $PATH and get going!

## development

    cargo run -- <gopher-url>

## screenies

![DOS Archive](./img/dos.png)

![Menu View](./img/menu-view.png)

![Text View](./img/text-view.png)

## todo

- [ ] fork+exec telnet: gopher://bitreich.org/1/lawn/bbs
- [ ] alternate screen support
- [ ] bookmarks: toggle instead of just prepending to the file
- [ ] bookmarks: save the title of the current page

## bugs

- [ ] gopher://1436.ninja/1/twit.cgi ("iWritten and performed by Nathaniel" weirdness)
- [ ] screen flicker in win10 WSL
- [ ] NUM entry and Find entry shouldn't use same buffer
- [ ] page down / spacebar when last link selected & there is more content

## future features

- [ ] Incremental search in Text views
- [ ] Linked gopher and http URLs in Text views
- [ ] TLS -- https://dataswamp.org/~solene/2019-03-07-gopher-server-tls.html
- [ ] Fuzzy Find incremental search
