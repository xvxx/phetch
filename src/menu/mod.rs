mod menu_view;
use gopher;
use gopher::Type;

pub type MenuView = menu_view::MenuView;

pub struct Menu {
    pub url: String,       // gopher url
    pub lines: Vec<Line>,  // lines
    pub links: Vec<usize>, // links (index of line in lines vec)
    pub longest: usize,    // size of the longest line
}

#[derive(Debug)]
pub struct Line {
    pub name: String,
    pub url: String,
    pub typ: Type,
    pub link: usize, // link #, if any
}

impl Menu {
    pub fn from(url: String, gopher_response: String) -> Menu {
        Self::parse(url, gopher_response)
    }

    pub fn parse(url: String, raw: String) -> Menu {
        let mut lines = vec![];
        let mut links = vec![];
        let mut link = 0;
        let mut longest = 0;
        for line in raw.split_terminator('\n') {
            if let Some(c) = line.chars().nth(0) {
                let typ = match gopher::type_for_char(c) {
                    Some(t) => t,
                    None => continue,
                };

                // assemble line info
                let parts: Vec<&str> = line.split_terminator('\t').collect();

                let mut name = String::from("");
                if !parts[0].is_empty() {
                    name.push_str(&parts[0][1..]);
                }
                if typ != Type::Info {
                    link += 1;
                }
                if name.len() > longest {
                    longest = name.len();
                }
                let link = if typ == Type::Info { 0 } else { link };
                if link > 0 {
                    links.push(lines.len());
                }

                // check for URL:<url> syntax
                if parts.len() > 1 {
                    if parts[1].starts_with("URL:") {
                        lines.push(Line {
                            name,
                            url: parts[1].chars().skip(4).collect::<String>(),
                            typ,
                            link,
                        });
                        continue;
                    }
                }

                // assemble regular, gopher-style URL
                let mut url = String::from("gopher://");
                if parts.len() > 2 {
                    url.push_str(parts[2]); // host
                }
                if parts.len() > 3 {
                    // port
                    let port = parts[3].trim_end_matches('\r');
                    if port != "70" {
                        url.push(':');
                        url.push_str(parts[3].trim_end_matches('\r'));
                    }
                }
                // auto-prepend gopher type to selector
                if let Some(first_char) = parts[0].chars().nth(0) {
                    url.push_str("/");
                    url.push(first_char);
                }
                if parts.len() > 1 {
                    url.push_str(parts[1]); // selector
                }
                lines.push(Line {
                    name,
                    url,
                    typ,
                    link,
                });
            }
        }

        Menu {
            url,
            lines,
            links,
            longest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_menu() {
        let menu = Menu::parse(
            "test".to_string(),
            "
i---------------------------------------------------------
1SDF PHLOGOSPHERE (297 phlogs)	/phlogs/	gopher.club	70
1SDF GOPHERSPACE (1303 ACTIVE users)	/maps/	sdf.org	70
i---------------------------------------------------------
"
            .to_string(),
        );
        assert_eq!(menu.lines.len(), 4);
        assert_eq!(menu.links.len(), 2);
        assert_eq!(menu.lines[1].url, "gopher://gopher.club/1/phlogs/");
        assert_eq!(menu.lines[2].url, "gopher://sdf.org/1/maps/");
    }
}
