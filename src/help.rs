use bookmarks;
use history;

pub fn lookup(name: &str) -> Option<String> {
    Some(match name {
        "" | "/" | "home" | "home/" => format!("{}{}", HEADER, HOME),
        "help" | "help/" => format!("{}{}", HEADER, HELP),
        "history" => history::as_raw_menu(),
        "bookmarks" => bookmarks::as_raw_menu(),
        "help/keys" => format!("{}{}", HEADER, KEYS),
        "help/nav" => format!("{}{}", HEADER, NAV),
        "help/types" => format!("{}{}", HEADER, TYPES),
        _ => return None,
    })
}

pub const HEADER: &str = "
i                                      	/spacer
i
i      /         /         /   
i ___ (___  ___ (___  ___ (___ 
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  / 
i|   
i
";

pub const HOME: &str = "
i~ the quick lil gopher client ~
i
7search gopher	/v2/vs	gopher.floodgap.com
1welcome to gopherspace	/gopher	gopher.floodgap.com
1the gopher project	/	gopherproject.org
1gopher lawn	/lawn	bitreich.org
i 
i            ~ * ~
i
1show help          \x1b[90mctrl-h	/help	phetch
1show history       \x1b[90mctrl-a	/history	phetch
1show bookmarks     \x1b[90mctrl-b	/bookmarks	phetch
i\x1b[0m
";

pub const HELP: &str = "
i      ** help topics **
i
1keyboard shortcuts	/help/keys	phetch
1menu navigation	/help/nav	phetch
1gopher types	/help/types	phetch
i 
i            ~ * ~
i 
1start screen	/home	phetch
1history	/history	phetch
hphetch webpage	URL:https://github.com/dvkt/phetch
i 
";

pub const KEYS: &str = "
i   ** keyboard shortcuts **
i
i\x1b[95mleft       \x1b[96mback in history
i\x1b[95mright      \x1b[96mforward in history
i\x1b[95mup         \x1b[96mselect prev link 
i\x1b[95mdown       \x1b[96mselect next link 
i\x1b[95mpage up    \x1b[96mscroll page up
i\x1b[95mpage down  \x1b[96mscroll page down
i
i\x1b[95mnum key    \x1b[96mopen / select link
i\x1b[95menter      \x1b[96mopen selected link
i\x1b[95mescape     \x1b[96mcancel
i
i\x1b[95mctrl-g     \x1b[96mgo to gopher url
i\x1b[95mctrl-u     \x1b[96mshow gopher url
i\x1b[95mctrl-y     \x1b[96mcopy url 
i\x1b[95mctrl-r     \x1b[96mview raw source
i\x1b[95mctrl-w     \x1b[96mtoggle wide mode
i
i\x1b[95mctrl-a     \x1b[96mshow history
i\x1b[95mctrl-b     \x1b[96mshow bookmarks
i\x1b[95mctrl-s     \x1b[96msave bookmark
i\x1b[0m
";

pub const NAV: &str = "
i    ** menu navigation **
i
ithere are three ways to navigate
imenus in phetch:
i
1up & down arrows	/help/nav	phetch
i
iuse the up and down arrows or the
ictrl-p/ctrl-n combos to select menu 
iitems. phetch will scroll for you,
ior you can use page up & page down
i(or - and spacebar) to jump by many 
ilines quickly.
i
1number keys	/help/nav	phetch
i
iif there are few enough menu items,
ipressing a number key will open the
iitem immediately. otherwise, it'll
ibe selected. use enter to open it.
i
1incremental search	/help/nav	phetch
i
ijust start typing. phetch will look
ifor the first case insensitive match
iand try to select it. use the arrow
ior ctrl-p/n keys to cycle matches.
i
";

pub const BOOKMARKS: &str = "
i       ** bookmarks **
i
";

pub const HISTORY: &str = "
i        ** history **
i
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
hexternal URLs	URL:https://en.wikipedia.org/wiki/Phetch	phetch
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
