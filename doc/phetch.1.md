PHETCH(1)

# NAME

phetch - quick lil gopher client

# SYNOPSIS

*phetch* [_OPTIONS_] [_URL_]

# DESCRIPTION

*phetch* is a terminal client designed to help you quickly navigate
the gophersphere. It features non-nonsense keyboard navigation,
support for most Gopher features, easy-to-use TLS and Tor support, as
well as bookmarking and history features.

Usually *phetch* is started with a Gopher URL:

	phetch gopher://some-gopher-url.com

If no URL is given, however, *phetch* will launch and open its default
"start page". This can be configured to be any URL. (See *CONFIG*.)

# OPTIONS

*-l*, *--local*
	Connect to the local Gopher server at URL _127.0.0.1:7070_.

*-p* _URL_, *--print* _URL_
	Print a rendered Gopher server response of _URL_ and exit.

*-r* _URL_, *--raw* _URL_
	Print the raw Gopher server response of _URL_ and exit.

*-s*, *--tls*
	Attempt to fetch all pages securely over TLS.

*-o*, *--tor*
	Make all connections using a local Tor proxy.
	Tor is the Onion Router.
	Set the TOR_PROXY env variable to use an address other than the
	Tor default of 127.0.0.1:9050.

*-h*, *--help*
	Print a help summary and exit.

*-v*, *--version*
	Print version information and exit.

# NOTES

When given a _URL_, *phetch* will show the requested Gopher page and
enter interactive mode.

Without a _URL_, *phetch* will show a builtin dashboard with easy
access to online help, bookmarks and history, and enter interactive
mode.

# NAVIGATION

## KEYBOARD SHORTCUTS

All single letter commands also work with the *Ctrl* key: e.g., *h*
and *Ctrl-h* are synonyms.

*h*
	Go to builtin help page.
*q*
	Quit *phetch*.

*left arrow*
	Go back in history.
*right arrow*
	Go forward in history.
*up arrow*, *p*, *k*
	Select previous link.
*down arrow*, *n*, *j*
	Select next link.
*PgUp*, *-*
	Scroll up by many lines.
*PgDn*, *SPACE*
	Scroll down by many lines.

*Number key*
	Open/select link.
*Enter*
	Open current link.
*Esc*, *Ctrl-c*
	Cancel

*f*, */*
	Find link in page.

*g*
	Go to Gopher URL.
*u*
	Edit URL.
*y*
	Copy URL.

*b*
	Show bookmarks.
*s*
	Save bookmark.
*a*
	Show history. (Mnemonic: *All* pages/history)

*r*
	View raw source.
*w*
	Toggle wide mode.

## MENU NAVIGATION

Up and down arrows
	Use the up and down arrows, *j* and *k* keys, or *n* and *p*
	keys to select links. *phetch* will scroll for you, or you can
	use page up and page down (or *-* and spacebar) to scroll by
	many lines at once.

Number keys
	If there are few enough menu items, pressing a number key will
	open a link. Otherwise, the first matching number will be
	selected. Use *Enter* to open the selected link.

Incremental search
	Press *f* or */* to activate search mode, then just start
	typing. *phetch* will look for the first case-insensitive match
	and try to select it. Use arrow keys or *Ctrl-p*/*Ctrl-n* to cycle
	through matches.

# BOOKMARKS

There are two ways to save the URL of the current page:

*y*
	Copy URL.
*s*
	Save bookmark.

Bookmarks will be saved to the file _~/.config/phetch/bookmarks.gph_ if
the directory _~/.config/phetch/_ exists.

*b*
	View saved bookmarks.

The clipboard function uses *pbcopy* on MacOS, and *xsel* _-sel clip_
on Linux.

# HISTORY

If you create a _history.gph_ file in _~/.config/phetch/_, each Gopher
URL you open will be stored there.

New URLs are appended to the bottom, but loaded in reverse order, so
you'll see all the most recently visited pages first when you press 
the *a* key.

Feel free to edit your history file directly, or share it with your
friends!

# CONFIG

If you create a _phetch.conf_ file in _~/.config/phetch/_, it will be
automatically loaded when *phetch* starts. The config file supports
most command line options, for your convenience. For example,
*phetch* will always launch in TLS mode if `tls yes` appears in the
config file -- no need to pass `--tls` or `-t` on startup.

Here is an example config with all options:

```
# Page to load when launched with no URL argument.
start gopher://phetch/1/home

# Always use TLS mode. (--tls)
tls no

# Connect using local TOR proxy. (--tor)
tor no

# Always start in wide mode. (--wide)
wide no

# Use emoji indicators for TLS & Tor. (--emoji)
emoji no
```

# ABOUT

*phetch* is maintained by chris west, and released under the MIT license.

phetch's Gopher hole:
	_gopher://phkt.io/1/phetch_
phetch's webpage:
	_https://github.com/xvxx/phetch_
