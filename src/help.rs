//! The `help` module manages all internal Gopher pages, from the help
//! system itself to the Start and "About Phetch" pages.

use crate::{bookmarks, history};

/// Find a help file/page. If found, gives the raw Gophermap.
pub fn lookup(name: &str) -> Option<String> {
    Some(match name {
        "" | "/" | "home" | "home/" => format!("{}{}", HEADER, START),
        "history" => history::as_raw_menu(),
        "bookmarks" => bookmarks::as_raw_menu(),
        "help/config" => format!("{}{}", HEADER, CONFIG),
        "help/themes" => format!("{}{}", HEADER, THEMES),
        "help/keys" => format!("{}{}", HEADER, KEYS),
        "help/nav" => format!("{}{}", HEADER, NAV),
        "help/types" => format!("{}{}", HEADER, TYPES),
        "help/bookmarks" => format!("{}{}", HEADER, BOOKMARKS),
        "help/history" => format!("{}{}", HEADER, HISTORY),
        "help" | "help/" => format!(
            "{}{}",
            HEADER,
            HELP.replace("{platform}", crate::PLATFORM)
                .replace("{version}", crate::VERSION)
        ),
        "about" => format!(
            "{}{}",
            HEADER,
            ABOUT
                .replace("{build-date}", crate::BUILD_DATE)
                .replace("{git-ref}", crate::GIT_REF)
                .replace("{version}", crate::VERSION)
                .replace(
                    "{tls-support}",
                    if crate::TLS_SUPPORT {
                        "supported"
                    } else {
                        "not supported"
                    }
                )
                .replace(
                    "{tor-support}",
                    if crate::TOR_SUPPORT {
                        "supported"
                    } else {
                        "not supported"
                    }
                )
        ),
        _ => return None,
    })
}

const HEADER: &str = "
i
i      /         /         /
i ___ (___  ___ (___  ___ (___
i|   )|   )|___)|    |    |   )  	/spacer
i|__/ |  / |__  |__  |__  |  /
i|
i
";

const START: &str = "
i            ~ * ~
i
7search gopher	/v2/vs	gopher.floodgap.com
1gopherpedia	/	gopherpedia.com	70
1gopher lawn	/lawn	bitreich.org
1welcome to gopherspace	/gopher	gopher.floodgap.com
i
i            ~ * ~
i
1show help          (ctrl-h)	/help	phetch
1show history       (ctrl-a)	/history	phetch
1show bookmarks     (ctrl-b)	/bookmarks	phetch
i
";

const HELP: &str = "
i      ** help topics **
i
1keyboard shortcuts	/help/keys	phetch
1menu navigation	/help/nav	phetch
1gopher types	/help/types	phetch
1bookmarks	/help/bookmarks	phetch
1history	/help/history	phetch
1phetch.conf	/help/config	phetch
1themes	/help/themes	phetch
i
i            ~ * ~
i
1start screen	/home	phetch
1about phetch	/about	phetch
1check for updates	/phetch/latest?{platform}|v{version}	phkt.io
i
";

const KEYS: &str = "
i   ** keyboard shortcuts **
i
ileft       back in history
iright      next in history
iup         select prev link
idown       select next link
ipg up/down scroll by many lines
i- or space same as pg up/down
i
inum key    open/select link
ienter      open current link
iescape     cancel
ictrl-c     cancel
i
if or /     find link in page
ip or k     select prev link
in or j     select next link
i
ig          go to gopher url
iu          edit url
iy          copy url
i
ib          show bookmarks
is          save bookmark
ia          show history
i
ir          view raw source
iw          toggle wide mode
ie          toggle encoding
iq          quit phetch
ih          show help
i
iall single letter commands also
iwork with the ctrl key.
i
";

const NAV: &str = "
i    ** menu navigation **
i
ithere are three ways to
inavigate menus in phetch:
i
1up & down arrows	/help/nav	phetch
i
iuse the up and down arrows,
ij and k keys, or n and p keys
ito select links. phetch will
iscroll for you, or you can use
ipage up & page down (or - and
ispacebar) to scroll by many
ilines at once.
i
1number keys	/help/nav	phetch
i
iif there are few enough menu
iitems, pressing a number key
iwill open a link. otherwise,
ithe first matching number will
ibe selected. use enter to open
ithe selected link.
i
1incremental search	/help/nav	phetch
i
ipress f or / to activate search
imode, then just start typing.
iphetch will look for the first
icase-insensitive match and try
ito select it. use arrow keys
ior ctrl-p/n to cycle matches.
i
";

const BOOKMARKS: &str = "
i       ** bookmarks **
i
iphetch has two ways to save
ithe url of the current page:
i
iy      copy url
is      save bookmark
i
iif ~/.config/phetch/ exists,
ibookmarks will be saved to
i~/.config/phetch/bookmarks.gph
i
ipress b to view them.
i
ithe clipboard function uses:
i
i- `pbcopy` on macos
i- `xclip -sel clip` on linux
i";

const HISTORY: &str = "
i        ** history **
i
iif you create a history.gph
ifile in ~/.config/phetch/,
ieach gopher url you open will
ibe stored there.
i
inew urls are appended to the
ibottom, but loaded in reverse
iorder, so you'll see the most
irecently visited pages first
iwhen you press the a key.
i
ifeel free to edit your history
ifile directly, or share it
iwith your friends!
";

const CONFIG: &str = "
i         ** config **
i
iif you create a phetch.conf
ifile in ~/.config/phetch/ it
iwill be automatically loaded
iwhen phetch starts. the config
ifile supports most command line
ioptions, for your convenience.
i
ifor example, phetch will always
ilaunch in TLS mode if `tls yes`
iappears in the config file.
i
ihere is an example phetch.conf
iwith all possible keys:
i
i# page to load when launched
istart gopher://phetch/1/home
i
i# always use TLS mode
itls no
i
i# connect over tor proxy
itor no
i
i# start in wide mode
iwide no
i
i# show emoji status indicators
iemoji no
i
i# cp437 or utf8 encoding
iencoding utf8
i
i# wrap text at N cols. 0 = off
iwrap 0
i
i# page up/down by N lines.
i# 0 = full screen
iscroll 0
i
i# path to theme file, if any
itheme ~/.config/phetch/fun.theme
";

const THEMES: &str = "
i        ** themes **
i
iyou can change phetch's color
ischeme by supplying your own
itheme file with --theme/-t or
iby setting `theme FILE` in
iyour phetch.conf.
i
iyou can also set colors directly
iin your phetch.conf.
i
iview the current theme with:
i
i$ phetch --print-theme
i
i       ** examples **
i
itheme files are plain text files
ithat look like this:
i
iui.cursor white bold
iui.number magenta
iui.menu yellow
iui.text white
iitem.text cyan
iitem.menu blue
iitem.error red
iitem.search white
iitem.telnet grey
iitem.external green
iitem.download white underline
iitem.media green underline
iitem.unsupported whitebg red
i
i     ** valid colors **
i
ibold
iunderline
igrey
ired
igreen
iyellow
iblue
imagenta
icyan
iwhite
iblack
idarkred
idarkgreen
idarkyellow
idarkblue
idarkmagenta
idarkcyan
idarkwhite
iblackbg
iredbg
igreenbg
iyellowbg
ibluebg
imagentabg
icyanbg
iwhitebg
";

const TYPES: &str = "
i     ** gopher types **
i
iphetch supports these links:
i
0text files	/Mirrors/RFC/rfc1436.txt	fnord.one	65446
1menu items	/lawn/ascii	bitreich.org
3errors	/help/types	phetch
7search servers	/	forthworks.com	7001
8telnet links	/help/types	phetch
hexternal urls	URL:https://en.wikipedia.org/wiki/Phetch	phetch
i
ithese download types:
i
4binhex	/help/types	phetch
5dosfiles	/help/types	phetch
6uuencoded files	/help/types	phetch
9binaries	/help/types	phetch
gGIFs	/help/types	phetch
Iimages downloads	/help/types	phetch
ddocuments	/help/types	phetch
i
iand these media types:
i
ssound files	URL:https://freepd.com/music/Wakka%20Wakka.mp3	phetch
;video files	URL:https://www.youtube.com/watch?v=oHg5SJYRHA0	phetch
i
iphetch does not support:
i
2CSO Entries 	/help/types	phetch
+Mirrors	/help/types	phetch
TTelnet3270	/help/types	phetch
i
";

const ABOUT: &str = "
i     ~ version: v{version} ~
i
1phetch's gopherhole	/phetch	phkt.io
hphetch's webpage	URL:https://github.com/xvxx/phetch
0MIT license	/MIT License	gopherpedia.com
i
i            ~ * ~
i
i        special thanks
i
ikseistrup:
i    major design, testing,
i    documentation help
i
iantirez:
i    added gopher to redis
i    and opened the door
i
ilartu:
i    inspired me to add some
i    \x1b[95mcolor\x1b[0m
i
i            ~ * ~
i
itls: {tls-support}
itor: {tor-support}
igit ref: {git-ref}
ibuilt on: {build-date}
i
";
