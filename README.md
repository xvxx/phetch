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
- [x] fetch gopher URL
- [x] parse gopher URL
- [x] fetch site by hostname: $ phetch sdf.org
- [x] fetch site by URL:      $ phetch gopher://hngopher.com/1/live/p1/
- [ ] TLS
- [x] phetch -h
- [x] phetch -v
### UI
- [x] render MENU
- [x] render TEXT
- [x] size content to terminal
- [ ] detect terminal resize 
- [ ] status() helper
- [ ] show all errors in status()
### MENU Links
- [x] up/down navigate links
- [ ] up/down scroll when next link out of view
- [ ] page up/page down show next page, highlight first link
- [x] `ENTER` visits highlighted link
- [ ] open HTML link in browser
### TEXT Scrolling
- [x] up/down scroll by 1 line
- [x] page up/page down show next page
- [ ] stop DOWN at last page
- [ ] stop PGNDOWN at last page
### Keyboard Shortcuts
- [x] backspace/left arrow goes back in history
- [x] right arrow goes forward in history, if any
- [ ] `?` to show all keyboard shortcuts
- [ ] `c` copies current URL to clipboard https://git.io/Je7YL
- [ ] input field that... takes input
- [ ] search functiponalir
### Download binaries
- [ ] download to ~/Downloads
- [ ] ? download to pwd
- [ ] ? download to custom location
### Persistent History
- [ ] save history to file
- [ ] load history from file
- [ ] load most recent URL when opening without args
### Bonus
- [ ] play sound file in background
- [ ] center content
- [ ] render markdown-lite
- [ ] display HTML-lite
- [ ] pipe input to render as gopher
      $ phetch gopher.antirez.com:70 | gg
- [ ] syntax highlight code
      $ phetch code.some-gopher-site.io/gw/main.go
### Overview
- [x] keyboard shortcuts:
        ← back in history
        → forward in history
        ↑ scroll up
        ↓ scroll down
        ⏎ open link
        - page up
    space page down
        t scroll to top
        b scroll to bottom
        g same as t
        G same as b
- [ ] item types:
    - [ ] 0 text file
    - [ ] 1 submenu
    - [ ] 2 ccso nameserver
    - [ ] 3 error
    - [ ] 4 binhex encoded file
    - [ ] 5 DOS file
    - [ ] 6 uuencoded file
    - [ ] 7 gopher full-text search
    - [ ] 8 telnet
    - [ ] 9 binary file
    - [ ] + mirror or alternate server
    - [ ] g GIF
    - [ ] i Image
    - [ ] T telnet 3270
    - [ ] h HTML file
    - [ ] i Informational message
    - [ ] s Sound file (WAV, mp3, etc)
    - [ ] d Document
