<!--
      /         /         /
 ___ (___  ___ (___  ___ (___
|   )|   )|___)|    |    |   )
|__/ |  / |__  |__  |__  |  /
|
--> <p align="center"> <img src="./img/logo.png"> </p>

`phetch` is a terminal client designed to help you quickly navigate
the gophersphere.

## features

- <1MB executable for linux and mac
- technicolor design (based on [GILD](https://github.com/dvkt/gild))
- no-nonsense keyboard navigation
- supports gopher searches, text and menu pages, and downloads
- save your favorite gopherholes with bookmarks
- opt-in history

## usage

    phetch                           launch and show start page
    phetch <gopher-url>              open gopher url
    phetch -r, --raw <gopher-url>    print raw gopher response
    phetch -h, --help                show this screen
    phetch -v, --version             show phetch version

    once you've launched phetch, use `ctrl-h` to view the on-line help.

## installation

binaries for linux, mac, and raspberry pi are available
at https://github.com/dvkt/phetch/releases:

- [phetch-v0.1.9-linux-x86_64.tar.gz][0]
- [phetch-v0.1.9-linux-armv7.tar.gz (RPi)][1]
- [phetch-v0.1.9-macos.zip][2]

just unzip/untar the `phetch` program into your $PATH and get going!

## development

    cargo run -- <gopher-url>

## screenies

![DOS Archive](./img/dos.png)

![Menu View](./img/menu-view.png)

![Text View](./img/text-view.png)

## todo

- [ ] fork+exec telnet: gopher://bitreich.org/1/lawn/bbs
- [ ] TLS -- https://dataswamp.org/~solene/2019-03-07-gopher-server-tls.html
- [ ] ~/.config/phetch/phetch.conf

## bugs

- [ ] gopher://1436.ninja/1/twit.cgi ("iWritten and performed by
  Nathaniel" weirdness) (kitty only)
- [ ] gopherpedia 'recent entries' weirdness (also kitty only)
- [ ] ctrl-z (suspend) doesn't work
- [ ] gopher://alexschroeder.ch/2020-01-02_This_Gopher_Hole/menu

## v1.0

- [ ] check for updates over gopher
- [ ] Changelog generation (for gopher and github)
- [ ] GIF screencast
- [ ] man page (small one)

## future features

- [ ] track binary size per release
- [ ] text views are menus when URLs are present (one per line max)
- [ ] Find Text in Text views
- [ ] fuzzy find incremental search
- [ ] persistent history
- [ ] bookmarks: toggle instead of just prepending to the file
- [ ] bookmarks: save the title of the current page

[0]: https://github.com/dvkt/phetch/releases/download/v0.1.9/phetch-v0.1.9-linux-x86_64.tar.gz
[1]: https://github.com/dvkt/phetch/releases/download/v0.1.9/phetch-v0.1.9-linux-armv7.tar.gz
[2]: https://github.com/dvkt/phetch/releases/download/v0.1.9/phetch-v0.1.9-macos.zip
