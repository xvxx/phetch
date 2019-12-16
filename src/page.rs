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
