## v1.0.2-dev

This release fixes a few small but irritating bugs:

- Downloads can now be cancelled while in-progress with no funny
  business.
- Resizing your terminal now resizes phetch automatically.

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
