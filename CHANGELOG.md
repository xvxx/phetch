## v1.2.0 (dev)

phetch is all about fun colors, but your options are limited. You
can turn off colors with the `NO_COLOR` env variable or you can
leave them on. That's it.

Well, not anymore. As of `v1.2`, phetch now supports themes.

### Themes

Themes are simple files with the same format as `phetch.conf`:

    $ cat ~/.config/phetch/default.theme
    # Color Scheme
    ## UI
    ui.cursor white bold
    ui.number magenta
    ui.menu yellow
    ui.text white

    ## Items
    item.text cyan
    item.menu blue
    item.error red
    item.search white
    item.telnet grey
    item.external green
    item.download white underline
    item.media green underline
    item.unsupported whitebg red

Create your theme file and launch phetch with `-t FILE`, or set
the `theme FILE` option in your `~/.config/phetch/phetch.conf`

You can see available colors and learn more about themes by opening
phetch's help - press `h` then `7` to get there quickly.

### Config Options

This release also adds a few new config options, for your convenience:

- `scroll` controls how many lines to jump by when paging up/down.
  If set to 0 (the new default), you'll jump by an entire screen.
- `autoplay` controls whether you'll be prompted to play media files
  or not. By default it's false, but one might find it handy to set
  to `true` if hosting, say, a Gopher-powered music server.

### Keyboard Shortcuts

Last but not least, you can now reload the current URL by pressing `R`.
Handy when developing your own Gopherhole!

## v1.1.0

Three new features in this release, plus an unknown number of new
bugs:

1. When the `NO_COLOR` env variable is set, phetch won't use colors
   when rendering menus. See https://no-color.org/ for more information.

2. CP437 encoding support! You can toggle it on or off using `ctrl-e`
   (for encoding) when viewing a Gopher text document, or using the
   `--encoding` command line flag. See
   https://en.wikipedia.org/wiki/Code_page_437.

   Huge thanks to Kjell for suggesting this feature and providing some
   great test data!

   _NOTE: This only works for text documents since there's no `TAB`
   character in CP437._

3. phetch now supports a primitive form of wrapping long lines when
   rendering Gopher text documents. It won't reflow the text, but it
   will make some phlogs and other documents slightly more readable.
   Enable it with `--wrap NUM` or by adding `wrap NUM` to your
   `phetch.conf`. You can disable it with `wrap 0`.

---

You may have run into long lines that don't break at the margins,
making the page hard to scroll and read:

![not wrapped](https://user-images.githubusercontent.com/41523880/97058194-f73d9d80-1541-11eb-8fc8-910489fafcc3.png)

Now, by either passing `--wrap NUM` or adding `wrap NUM` to your
`phetch.conf` file, phetch will attempt to wrap long lines at the
nearest punctation or space:

![wrapped](https://user-images.githubusercontent.com/41523880/97058201-fa388e00-1541-11eb-84ef-c539304870a6.png)

This is really useful if you want to browse, say, a directory of
Markdown files over Gopher. Modern Markdown is often written with the
assumption that the client will do the wrapping, so it can end up
looking pretty messy in an ananchronistic client like phetch. Reading
those files is now a bit easier:

| not wrapped                                                                                                          | wrapped                                                                                                          |
| -------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| ![not wrapped](https://user-images.githubusercontent.com/41523880/97057857-1556ce00-1541-11eb-9cc1-6c6d438529ea.png) | ![wrapped](https://user-images.githubusercontent.com/41523880/97057869-1ee03600-1541-11eb-8e7b-ae47ff9ec871.png) |

This also works nicely on native Gopher content: phlog entries
sometimes have long URLs in their footnotes, and that could screw up
phetch's margin calculations.

Note that this doesn't do any _reflow_ of text, so documents with long
lines will still look a bit wonky, as you can see above. Some lines
will be too short. But it's a lot more usable, so we'll take it!

PS: You can use smaller values to get weird with it:

![weird](https://user-images.githubusercontent.com/41523880/97057878-269fda80-1541-11eb-9435-89f97cce8825.png)

Enjoy!

## v1.0.7

This release fixes https://github.com/xvxx/phetch/issues/19

phetch was aborting whenever it encountered a connection error
instead of trying the alternate socket addrs it was given.

Special thanks to @Ramiferous and @voidpin and **rvp**!

## v1.0.6

- More "reload" bugfixes.

## v1.0.5

Fix a crash introduced in 1.0.4.

## v1.0.4

- The `ctrl-u` and `ctrl-g` keyboard shortcuts can now be used
  to reload the current page.

## v1.0.3

This release adds support for the `;` and `s` Gopher item types,
as well as the ability to play them in a media player - meaning
you can now run Gopher-powered media servers! As seen here:

https://twitter.com/grufwub/status/1264296292764856320

`mpv` is used by default, but you can specify a custom player
or disable the feature using the `-m` and `-M` flags. Info has
been added to `--help` and the phetch manual.

Special thanks to @grufwub for the feature request and getting
the code rolling!

Enjoy!

## v1.0.2

This release fixes a few small but irritating bugs:

- ANSI color codes now render properly. Full technicolor support.
  Try it out: `phetch gopher://tilde.black/1/users/genin/`
- Resizing your terminal now resizes phetch automatically.
- Downloads can now be cancelled while in-progress with no funny
  business.
- Debug information is now properly displayed when phetch crashes.

## v1.0.1

This is a small bugfix release. Thanks to @TheEnbyperor and @grufwub!

- phetch no longer panics on multibyte characters when trying to
  truncate Gopher content.

## v1.0.0

`phetch` is now **v1.0.0**! Major thanks to @kseistrup for design,
testing, and documentation, @iglosiggio for supporting [GILD][gild],
@lartu for inspiration, and @antirez for re-introducing me to Gopher
one year ago with his blog post, [Gopher: a present for
Redis](http://antirez.com/news/127).

---

![phetch screen][phetch screen]

---

`phetch` is a terminal Gopher client designed to help you quickly
navigate the gophersphere. With a snappy, text-based UI, Gopher types
distinguished by color, and built-in support for secure Gopher and Tor
routing, `phetch` is perfect for catching up on the latest from
sdf.org or kicking back and enjoying some Zaibatsu.

Download a binary release below for Linux, Raspberry Pi, or macOS, or
see the [Installation][install] section of the README for instructions
on how to install for Arch Linux with AUR (`yay phetch`), macOS with
homebrew (`brew install xvxx/code/phetch`), or how to build from
source.

---

I have fond memories of using telnet to connect to the local library
when I was a kid, browsing their selection of books in an
amber-colored, text-based interface. This was the mid-90s, so I was
using some version of Windows, literally dialing into the library with
Hyperterminal.

<p align="center">
<img src="https://git.io/JvusG" alt="library tui">
</p>

It was futuristic. And, I thought, lost in the past. But Gopher, a
relic of that text-based era, lives on thanks to the work of some
amazing folks, and today there are more Gopher servers than ever.

The protocol is simple, constrained, and bursting with opportunity.
And while [MTV may not have an active Gopher server anymore][mtv], you
can easily run your own, or find a generous host like SDF or a tilde.

---

![gopher menu in phetch][phetch menu]

---

`phetch` is my attempt to bring a little bit of that retro-nostalgia
back into my terminal. Sure, I can acccess Gopher just fine using
`lynx` or through a web proxy like [Floodgap][floodgap], but where's
the fun in that?

To get started just install and run `phetch`.

It's not perfect, but I've had fun using it, and I hope you do too!

[phetch screen]: https://raw.githubusercontent.com/xvxx/phetch/f1fe58d2483af1c64fa61aa46e5858b599f8e67b/img/start.png
[phetch menu]: https://raw.githubusercontent.com/xvxx/phetch/3ec5e3f4335a5fdf709b5643da8aa4d5abe70815/img/dos.png
[install]: README.md#installation
[gild]: https://github.com/xvxx/gild
[floodgap]: https://gopher.floodgap.com/gopher/
[mtv]: https://tedium.co/2017/06/22/modern-day-gopher-history/

## v0.9.1

This update improves the release system. The man page is now included
in release downloads and installed with `homebrew`.

## v0.9.0

This is the first release candidate for `phetch v1.0.0`. We will
continue fixing bugs, tweaking the release system, and pruning
the public Rust API, but no new features will be added until v1.0.0
is released.

### Added

- Changelog is now available:
  gopher://phkt.io/0/code/phetch/CHANGELOG.md
- Added some basic internals documentation.
- Added `--no-default-features` build flag to disable Tor and TLS.

### Changed

- Parsing and rendering Gophermaps got a major performance boost.
- Memory utilization has been reduced.
- Error checking has been improved throughout.
- Fixed .onion URLs when using Tor.
- phetch is now clippy compatible.
- phetch config is not loaded in tests.
- TTY checking disabled in tests.
- Fixed `--no-config` flag.
- Fixed crash when building without git.
- Fixed a few status line display bugs.
- Fixed a minor config parsing bug.

## v0.1.13

This release fixes some longstanding display bugs and introduces Tor
support to help you easily browse Gopher more anonymously.

The next release will be `v0.9.0`, the first release candidate for
`phetch v1.0`. We do not anticipate adding any more large features
before the 1.0 release.

### Added

- phetch now supports [Tor][tor]!
- phetch now supports a `~/.config/phetch/phetch.conf` config file!
- Specify your own config file with `--config FILE`. Or disable with
  `-C`/`--no-config`.
- Emoji can be used as status indicators. Put `emoji yes` in your
  config file. 🧅🔐
- `phetch --print URL` will just print a rendered version of the page.
- `phetch -p URL | cat` works now. A simplified, plaintext version of
  the page will be rendered.
- Tor and TLS can be disabled with `-O` and `-S`, opposites of their
  `-o` and `-s` flags.
- On macOS, phetch is now available through [Homebrew](brew.sh):
  > brew install xvxx/code/phetch

### Changed

- Wide mode (`ctrl-w`/`w`) is now session-wide, not per-page.
- Many rendering bugs fixed. Pages with UTF8 errors are now displayed.
- Sites that don't prefix their selectors with `/` now work.

[tor]: (https://www.torproject.org/)
