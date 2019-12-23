use bookmarks;
use history;

pub fn lookup(name: &str) -> Option<String> {
    Some(match name {
        "" | "/" | "help" => format!("{}{}", HEADER, HELP),
        "home" => format!("{}{}", HEADER, HOME),
        "history" => history::as_raw_menu(),
        "bookmarks" => bookmarks::as_raw_menu(),
        "keys" => format!("{}{}", HEADER, KEYS),
        "nav" => format!("{}{}", HEADER, NAV),
        "types" => format!("{}{}", HEADER, TYPES),
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
1phetch help        \x1b[90mctrl-h	/	help
1show history       \x1b[90mctrl-a	/history	help
1show bookmarks     \x1b[90mctrl-b	/bookmarks	help
i\x1b[0m
";

pub const HELP: &str = "
i      ** help topics **
i
1keyboard shortcuts	/keys	help
1menu navigation	/nav	help
1gopher types	/types	help
i 
i            ~ * ~
i 
1start screen	/home	help
1history	/history	help
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
1up & down arrows	/nav	help
i
iuse the up and down arrows or the
ictrl-p/ctrl-n combos to select menu 
iitems. phetch will scroll for you,
ior you can use page up & page down
i(or - and spacebar) to jump by many 
ilines quickly.
i
1number keys	/nav	help
i
iif there are few enough menu items,
ipressing a number key will open the
iitem immediately. otherwise, it'll
ibe selected. use enter to open it.
i
1incremental search	/nav	help
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
3errors	/types	help
7search servers	/	forthworks.com	7001
8telnet links	/types	help
hexternal URLs	URL:https://en.wikipedia.org/wiki/Phetch/	help
i
iand these download types:
i
4binhex	/types	help
5dosfiles	/types	help
6uuencoded files	/types	help
9binaries	/types	help
gGIFs	/types	help
Iimages downloads	/types	help
ssound files	/types	help
ddocuments	/types	help
i
iphetch does not support: 
i
2CSO Entries 	/types	help
+Mirrors	/types	help
TTelnet3270	/types	help
i
";
