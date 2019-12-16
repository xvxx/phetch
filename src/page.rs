use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use types::Type;

#[derive(Debug)]
pub struct Link {
    pos: usize, // which link in the page
    title: String,
    host: String,
    port: usize,
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
    fn new() -> Page {
        Page {
            raw: String::new(),
            url: String::new(),
            links: vec![],
            link: 0,
            typ: Type::Menu,
            input: String::new(),
            offset: 0,
        }
    }

    pub fn load(host: &str, port: &str, selector: &str) -> io::Result<Page> {
        let mut page = Self::new();
        match Self::fetch(host, port, selector) {
            Ok(res) => {
                page.raw = res;
                Ok(page)
            }
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
}
