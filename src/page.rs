use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use types::Type;

#[derive(Debug)]
pub struct Line {
    pos: usize, // which line in the page
    name: String,
    host: String,
    port: String,
    selector: String,
    typ: Type,
}

#[derive(Debug)]
pub struct Page {
    typ: Type,        // entry type
    raw: String,      // raw gopher response
    url: String,      // gopher url
    lines: Vec<Line>, // lines
    line: usize,      // selected line
    input: String,    // user's inputted value
    offset: usize,    // scrolling position
}

impl Page {
    pub fn from(url: String, gopher_response: String) -> Page {
        Self::parse_menu(url, gopher_response)
    }

    // Loads a Page given a URL.
    pub fn load(host: &str, port: &str, selector: &str) -> io::Result<Page> {
        let url = format!("{}:{}{}", host, port, selector);
        match Self::fetch(host, port, selector) {
            Ok(res) => Ok(Page::from(url, res)),
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

    // Parses the lines in a raw Gopher menu response.
    fn parse_menu(url: String, raw: String) -> Page {
        let mut lines = vec![];
        let mut line = (0, 0, Type::Menu); // (name start pos, name end, type)
        let mut start = true; // are we at beginning of a line?
        let mut count = 0; // which line # are we
        let mut skip_line = false;

        for (i, c) in raw.char_indices() {
            if start {
                line.0 = i + 1;
                match c {
                    '0' => {
                        line.2 = Type::Text;
                    }
                    '1' => {
                        line.2 = Type::Menu;
                    }
                    'h' => {
                        line.2 = Type::HTML;
                    }
                    'i' => {
                        line.2 = Type::Info;
                    }
                    '\n' => continue,
                    _ => {
                        eprintln!("unknown line type: {}", c);
                        skip_line = true;
                    }
                }
                start = false;
            } else if c == '\n' {
                start = true;
                if skip_line {
                    skip_line = false;
                    continue;
                }
                if i > line.0 {
                    line.1 = i;
                    let mut parts = [""; 4];
                    for (j, s) in raw[line.0..line.1].split('\t').enumerate() {
                        if j < parts.len() {
                            parts[j] = s;
                        }
                    }
                    lines.push(Line {
                        name: parts[0].to_string(),
                        selector: parts[1].to_string(),
                        host: parts[2].to_string(),
                        port: parts[3].trim_end_matches('\r').to_string(),
                        typ: line.2,
                        pos: count,
                    });
                    count += 1;
                }
            }
        }

        Page {
            raw,
            url,
            lines,
            line: 0,
            typ: Type::Menu,
            input: String::new(),
            offset: 0,
        }
    }
}
