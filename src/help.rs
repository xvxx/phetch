use history;

pub fn lookup(name: &str) -> Option<String> {
    Some(match name {
        "" | "/" | "help" => HELP.into(),
        "types" => TYPES.into(),
        "nav" => NAV.into(),
        "home" => HOME.into(),
        "history" => history::load_as_raw_menu().unwrap_or_else(|| String::new()),
        _ => return None,
    })
}

pub const HOME: &str = "
i                                      	/spacer
i
i      /         /         /   
i ___ (___  ___ (___  ___ (___ 
i|   )|   )|___)|    |    |   )
i|__/ |  / |__  |__  |__  |  / 
i|   
i
i~ the quick lil gopher client ~
i
7search gopher	/v2/vs	gopher.floodgap.com
1welcome to gopherspace	/gopher	gopher.floodgap.com
1the gopher project	/	gopherproject.org
1gopher lawn	/lawn	bitreich.org
i 
i            ~ * ~
i
1phetch help (ctrl+h)	/	help
hphetch homepage	URL:https://github.com/dvkt/phetch
i
";

pub const HELP: &str = "
i                                      	/spacer
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
ictrl-u     show gopher url
ictrl-y     copy url to clipboard
ictrl-r     view raw source
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
hvisit homepage	URL:https://github.com/dvkt/phetch
i
";

pub const NAV: &str = "
i                                      	/spacer
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
i                                      	/spacer
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
