# phetch

dirt simple terminal gopher client.

## features

- small (<1MB) executable for linux and macos
- technicolor design
- no nonsense keyboard navigation

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
- [ ] fetch gopher URL
- [ ] parse gopher URL
- [ ] fetch site by hostname: $ phetch sdf.org
- [ ] fetch site by URL:      $ phetch gopher://hngopher.com/1/live/p1/
- [ ] phetch -h
- [ ] phetch -v
### UI
- [ ] render MENU
- [ ] render DOC
- [ ] size content to terminal
- [ ] detect terminal resize 
- [ ] status() helper
- [ ] show all errors in status()
### MENU Links
- [ ] up/down navigate links
- [ ] up/down scroll when next link out of view
- [ ] page up/page down show next page, highlight first link
- [ ] `ENTER` visits highlighted link
- [ ] open HTML link in browser
### DOC Scrolling
- [ ] up/down scroll by 1 line
- [ ] page up/page down show next page
### Keyboard Shortcuts
- [ ] backspace/left arrow goes back in history
- [ ] right arrow goes forward in history, if any
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
- [ ] keyboard shortcuts:
        ← back in history
        → forward in history
        ↑ scroll up
        ↓ scroll down
        ⏎ open link
        - page up
    space page down
        t scroll to top
        b scroll to bottom
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
