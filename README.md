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

    cargo run
    
## resources

- [rfc 1346](https://tools.ietf.org/html/rfc1436)

## gopher sites

- gopher.black
- sdf.org
- gopher.quux.org
- hngopher.com 
- bitreich.org

## TODO

### Basics
- [ ] TLS
- [ ] status() helper
- [ ] show all errors in status()
- [ ] MENU: up/down scroll when next link out of view
- [ ] MENU: page up/page down show next page, highlight first link
- [ ] MENU: open HTML link in browser
- [ ] `?` to show all keyboard shortcuts
- [ ] `c` copies current URL to clipboard https://git.io/Je7YL
- [ ] input field that... takes input
- [ ] search functionality
- [ ] download to ~/Downloads
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