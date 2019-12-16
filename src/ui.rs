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

    pub fn print(&self) {
        print!("{}", self.render());
    }

    pub fn render(&self) -> String {
        String::new()
    }
}
