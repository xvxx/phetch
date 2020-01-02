use crate::{bookmarks, history};

pub fn lookup(name: &str) -> Option<String> {
    Some(match name {
        "" | "/" | "home" | "home/" => format!("{}{}", HEADER, START),
        "help" | "help/" => format!("{}{}", HEADER, HELP),
        "history" => history::as_raw_menu(),
        "bookmarks" => bookmarks::as_raw_menu(),
        "about" => format!("{}{}", HEADER, ABOUT.replace("{version}", crate::VERSION)),
        "help/keys" => format!("{}{}", HEADER, KEYS),
        "help/nav" => format!("{}{}", HEADER, NAV),
        "help/types" => format!("{}{}", HEADER, TYPES),
        "help/bookmarks" => format!("{}{}", HEADER, BOOKMARKS),
        "help/history" => format!("{}{}", HEADER, HISTORY),
        _ => return None,
    })
}

pub const HEADER: &str = "
i
i      /         /         /
i ___ (___  ___ (___  ___ (___
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  /
i|
i
";

pub const START: &str = "
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

pub const HELP: &str = "
i      ** help topics **
i
1keyboard shortcuts	/help/keys	phetch
1menu navigation	/help/nav	phetch
1gopher types	/help/types	phetch
1bookmarks	/help/bookmarks	phetch
1history	/help/history	phetch
i
i            ~ * ~
i
1start screen	/home	phetch
1about phetch	/about	phetch
hphetch webpage	URL:https://github.com/dvkt/phetch
i
";

pub const KEYS: &str = "
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
iu          show gopher url
iy          copy url
i
ib          show bookmarks
is          save bookmark
ia          show history
i
ir          view raw source
iw          toggle wide mode
iq          quit phetch
ih          show help
i
iall single letter commands also
iwork with the ctrl key.
i
";

pub const NAV: &str = "
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

pub const BOOKMARKS: &str = "
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

pub const HISTORY: &str = "
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

pub const TYPES: &str = "
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
iand these download types:
i
4binhex	/help/types	phetch
5dosfiles	/help/types	phetch
6uuencoded files	/help/types	phetch
9binaries	/help/types	phetch
gGIFs	/help/types	phetch
Iimages downloads	/help/types	phetch
ssound files	/help/types	phetch
ddocuments	/help/types	phetch
i
iphetch does not support:
i
2CSO Entries 	/help/types	phetch
+Mirrors	/help/types	phetch
TTelnet3270	/help/types	phetch
i
";

pub const ABOUT: &str = "
i     ~ version: v{version} ~
i
ispecial thanks to:
i
ikseistrup - major help with
i            design & testing
i
i  antirez - introduced me to
i            the world of gopher
i
i    lartu - inspired me to
i            add some \x1b[95mcolor
i
i            ~ * ~
i
0phetch uses the MIT license	/MIT License	gopherpedia.com
hwebpage: github.com/dvkt/phetch	URL:https://github.com/dvkt/phetch
";
