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

    pub fn load(&mut self, host: &str, port: &str, selector: &str) {
        if let Some(page) = Page::load(host, port, selector) {
            self.pages.push(page);
        } else {
            eprintln!("error loading {}:{}{}", host, port, selector);
            std::process::exit(1);
        }
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }

    pub fn render(&self) -> String {
        String::new()
    }

    pub fn run(&self) {
        loop {
            self.print();
            self.respond_to_user();
        }
    }

    fn respond_to_user(&self) {}
}
