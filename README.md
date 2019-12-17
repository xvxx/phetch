# phetch

dirt simple terminal gopher client.

## features

- small (<1MB) executable for linux and macos
- technicolor design
- no nonsense keyboard navigation

## usage

    phetch <gopher-url>        # Show GopherHole at URL
    phetch -raw <gopher-url>   # Print raw Gopher response.
    phetch -help               # Show this screen.
    phetch -version            # Show phetch version.

## development

    cargo run -- <gopher-url>

## resources

- [rfc 1346](https://tools.ietf.org/html/rfc1436)
- http://ascii-table.com/ansi-escape-sequences.php

## gopher sites

- gopher.black
- sdf.org
- gopher.quux.org
- hngopher.com
- bitreich.org

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
- [ ] input field that... takes input
- [ ] search functionality
- [ ] download to ~/Downloads
    - https://github.com/dvkt/gg/blob/master/gg.go#L442
- [ ] save history to file
- [ ] load history from file
- [ ] load most recent URL when opening without args
### Bonus
- [ ] play sound file in background
- [ ] render markdown-lite
- [ ] display HTML-lite
- [ ] stop DOWN at last page
- [ ] stop PGNDOWN at last page
- [ ] ? download to pwd
- [ ] ? download to custom location
- [ ] center content
- [ ] pipe input to render as gopher
      $ phetch gopher.antirez.com:70 | gg
- [ ] syntax highlight code
      $ phetch code.some-gopher-site.io/gw/main.go
- [ ] fuzzy find search links
    - https://github.com/stewart/rff
    - https://github.com/Schlechtwetterfront/fuzzy-rs
- [ ] detect SIGWINCH
    - https://github.com/BurntSushi/chan-signal
