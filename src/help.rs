pub fn lookup(name: &str) -> Option<&str> {
    match name {
        "" | "/" | "help" => Some(HELP),
        "types" => Some(TYPES),
        "nav" => Some(NAV),
        "keys" => Some(KEYS),
        _ => None,
    }
}

pub const HELP: &str = "
i
i
i      /         /         /   
i ___ (___  ___ (___  ___ (___ 
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  / 
i|   
i
i   ** keyboard shortcuts **
i
ileft       back in history
iright      forward in history
iup         select prev link 
idown       select next link 
ipage up    scroll page up
ipage down  scroll page down
i
ictrl-g     go to gopher url
ictrl-u     show current gopher url
ictrl-y     copy url to clipboard
ictrl-r     view raw version of page
ictrl-w     toggle wide mode 
i 
i            ~ * ~
i
iPress the # of a link to visit
ior select it. Use ENTER to open
ithe selected link.
i
iTo select a link by name, just 
istart typing.
i 
i            ~ * ~
i
1menu navigation	/nav	help
1gopher types	/types	help
1all keys	/keys	help
hvisit homepage	URL:https://github.com/dvkt/phetch
i
";

pub const NAV: &str = "
i
i
i      /         /         /   
i ___ (___  ___ (___  ___ (___ 
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  / 
i|   
i
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
ibut you can use page up & page down
ito jump by many lines quickly.
i
1number keys	/nav	help
i
iif there are few enough menu items,
ipressing a number key will open the
iitem immediately. otherwise, it'll
ibe selected.
i
1incremental search	/nav	help
i
ijust start typing. phetch will look
ifor the first case insensitive match
iand try to select it.
i
";

pub const TYPES: &str = "
i
i
i      /         /         /   
i ___ (___  ___ (___  ___ (___ 
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  / 
i|   
i
i     ** gopher types **
i
iphetch supports these links:
i
0text files
1menu items
3errors
hexternal URLs
7search servers
8telnet launching
i
iand these download types:
i
4binhex
5dosfiles
6uuencoded files
9binaries
gGIFs
Iimages downloads
ssound files
ddocuments
i
iphetch does not support: 
i
2CSO Entries 
+Mirrors
TTelnet3270
i
";
