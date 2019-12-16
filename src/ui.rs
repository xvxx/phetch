use page::Page;

#[derive(Debug)]
pub struct UI {
    pages: Vec<Page>,
    page: usize,
}

impl UI {
    pub fn new() -> UI {
        UI {
            pages: vec![],
            page: 0,
        }
    }

    pub fn run(&self) {
        loop {
            self.print();
            self.respond_to_user();
        }
    }

    pub fn print(&self) {
        // print!("{}", self.render());
        print!("{:#?}", self);
    }

    pub fn render(&self) -> String {
        String::new()
    }

    pub fn load(&mut self, host: &str, port: &str, selector: &str) {
        match Page::load(host, port, selector) {
            Ok(page) => self.add_page(page),
            Err(e) => {
                eprintln!(
                    "\x1B[91merror loading \x1b[93m{}:{}{}: \x1B[0m{}",
                    host, port, selector, e
                );
                std::process::exit(1);
            }
        }
    }

    fn add_page(&mut self, page: Page) {
        self.pages.push(page);
        if self.pages.len() > 1 {
            self.page += 1;
        }
    }

    fn respond_to_user(&self) {}
}
