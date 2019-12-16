use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use types::Type;

#[derive(Debug)]
pub struct Link {
    pos: usize, // which link in the page
    name: String,
    host: String,
    port: String,
    selector: String,
    typ: Type,
}

#[derive(Debug)]
pub struct Page {
    raw: String,      // raw gopher response
    url: String,      // gopher url
    links: Vec<Link>, // URL strings
    link: usize,      // selected link
    typ: Type,        // entry type
    input: String,    // user's inputted value
    offset: usize,    // scrolling position
}

impl Page {
    pub fn from(url: String, gopher_response: String) -> Page {
        Self::parse(url, gopher_response)
    }

    // Loads a Page given a URL.
    pub fn load(host: &str, port: &str, selector: &str) -> io::Result<Page> {
        match Self::fetch(host, port, selector) {
            Ok(res) => Ok(Page::from(format!("{}:{}{}", host, port, selector), res)),
            Err(e) => Err(e),
        }
    }

    // Fetches a URL and returns a raw Gopher response.
    fn fetch(host: &str, port: &str, selector: &str) -> io::Result<String> {
        let mut body = String::new();
        let stream = TcpStream::connect(format!("{}:{}", host, port))
            .and_then(|mut stream| {
                stream.write(format!("{}\r\n", selector).as_ref());
                Ok(stream)
            })
            .and_then(|mut stream| {
                stream.read_to_string(&mut body);
                Ok(())
            });

        match stream {
            Ok(_) => Ok(body),
            Err(e) => Err(e),
        }
    }

    // Parses the links in a raw Gopher response;
    fn parse(url: String, raw: String) -> Page {
        let mut links = vec![];
        let mut start = true;
        let mut is_link = false;
        let mut link = (0, 0, Type::Menu);
        let mut count = 0;

        for (i, c) in raw.char_indices() {
            if start {
                match c {
                    '0' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = Type::Text;
                    }
                    '1' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = Type::Menu;
                    }
                    'h' => {
                        is_link = true;
                        link.0 = i + 1;
                        link.2 = Type::HTML;
                    }
                    '\n' => continue,
                    _ => is_link = false,
                }
                start = false;
            } else if c == '\n' {
                start = true;
                if is_link && i > link.0 {
                    link.1 = i;
                    let mut line = [""; 4];
                    for (j, s) in raw[link.0..link.1].split('\t').enumerate() {
                        if j < line.len() {
                            line[j] = s;
                        }
                    }
                    links.push(Link {
                        name: line[0].to_string(),
                        selector: line[1].to_string(),
                        host: line[2].to_string(),
                        port: line[3].trim_end_matches('\r').to_string(),
                        typ: link.2,
                        pos: count,
                    });
                    count += 1;
                    is_link = false;
                }
            }
        }

        Page {
            raw,
            url,
            links,
            link: 0,
            typ: Type::Menu,
            input: String::new(),
            offset: 0,
        }
    }
}
