use crate::gopher::{self, Type};
use crate::ui::{Action, Key, View, MAX_COLS, SCROLL_LINES};
use std::fmt;
use termion::{clear, cursor};

pub struct Menu {
    pub url: String,          // gopher url
    pub lines: Vec<Line>,     // lines
    pub links: Vec<usize>,    // links (index of line in lines vec)
    pub longest: usize,       // size of the longest line
    pub raw: String,          // raw response
    pub input: String,        // user's inputted value
    pub link: usize,          // selected link
    pub scroll: usize,        // scrolling offset
    pub searching: bool,      // search mode?
    pub size: (usize, usize), // cols, rows
    pub wide: bool,           // in wide mode?
}

pub struct Line {
    pub name: String,
    pub url: String,
    pub typ: Type,
    pub link: usize, // link #, if any
}

// direction of a given link relative to the visible screen
#[derive(PartialEq)]
enum LinkPos {
    Above,
    Below,
    Visible,
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

impl View for Menu {
    fn raw(&self) -> String {
        self.raw.to_string()
    }

    fn render(&self) -> String {
        self.render_lines()
    }

    fn respond(&mut self, key: Key) -> Action {
        self.process_key(key)
    }

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    fn url(&self) -> String {
        self.url.to_string()
    }
}

impl Menu {
    pub fn from(url: String, response: String) -> Menu {
        Self::parse(url, response)
    }

    fn cols(&self) -> usize {
        self.size.0
    }

    fn rows(&self) -> usize {
        self.size.1
    }

    /// Calculated size of left margin.
    fn indent(&self) -> usize {
        let cols = self.cols();
        let longest = if self.longest > MAX_COLS {
            MAX_COLS
        } else {
            self.longest
        };
        if longest > cols {
            0
        } else {
            let left = (cols - longest) / 2;
            if left > 6 {
                left - 6
            } else {
                0
            }
        }
    }

    fn link(&self, i: usize) -> Option<&Line> {
        if let Some(line) = self.links.get(i) {
            self.lines.get(*line)
        } else {
            None
        }
    }

    /// Is the given link visible on screen?
    fn is_visible(&self, link: usize) -> bool {
        self.link_visibility(link) == Some(LinkPos::Visible)
    }

    /// Where is the given link relative to the screen?
    fn link_visibility(&self, i: usize) -> Option<LinkPos> {
        if let Some(&pos) = self.links.get(i) {
            Some(if pos < self.scroll {
                LinkPos::Above
            } else if pos >= self.scroll + self.rows() - 1 {
                LinkPos::Below
            } else {
                LinkPos::Visible
            })
        } else {
            None
        }
    }

    fn render_lines(&self) -> String {
        let mut out = String::new();
        let (cols, rows) = self.size;

        macro_rules! push {
            ($c:expr, $e:expr) => {{
                out.push_str("\x1b[");
                out.push_str($c);
                out.push_str("m");
                out.push_str(&$e);
                out.push_str("\x1b[0m");
            }};
        }

        let iter = self.lines.iter().skip(self.scroll).take(rows - 1);
        let indent = self.indent();
        let left_margin = " ".repeat(indent);

        let mut line_count = 0;
        for line in iter {
            line_count += 1;
            let mut line_size = 0;
            if !self.wide {
                out.push_str(&left_margin);
                line_size += indent;
            }
            if line.typ == Type::Info {
                out.push_str("      ");
                line_size += 6;
            } else {
                if line.link == self.link {
                    out.push_str("\x1b[97;1m*\x1b[0m")
                } else {
                    out.push(' ');
                }
                line_size += 1;
                out.push(' ');
                line_size += 1;
                out.push_str("\x1b[95m");
                if line.link < 9 {
                    out.push(' ');
                    line_size += 1;
                }
                let num = (line.link + 1).to_string();
                out.push_str(&num);
                line_size += num.len();
                out.push_str(".\x1b[0m ");
                line_size += 2;
            }

            // truncate long lines, instead of wrapping
            let name = if line.name.len() > MAX_COLS {
                line.name.chars().take(MAX_COLS).collect::<String>()
            } else {
                line.name.to_string()
            };
            match line.typ {
                Type::Text => push!("96", name),
                Type::Menu => push!("94", name),
                Type::Info => push!("93", name),
                Type::HTML => push!("92", name),
                Type::Error => push!("91", name),
                Type::Telnet => push!("4;97;90", name),
                typ if typ.is_download() => push!("4;97", name),
                typ if !typ.is_supported() => push!("107;91", name),
                _ => push!("0", name),
            }

            // clear rest of line
            line_size += name.len();
            out.push_str(&" ".repeat(cols - line_size)); // fill line

            out.push_str("\r\n");
        }

        if self.searching {
            out.push_str(&self.render_input());
        }

        // clear remainder of screen
        let blank_line = " ".repeat(cols);
        for _ in 0..rows - line_count - 1 {
            out.push_str(&blank_line);
            out.push_str(&"\r\n");
        }

        out
    }

    /// Clear and re-draw the cursor.
    fn reset_cursor(&mut self, old_link: usize) -> Action {
        if self.links.is_empty() {
            return Action::None;
        }
        let mut out = String::new();
        if let Some(clear) = self.clear_cursor(old_link) {
            out.push_str(clear.as_ref());
        }
        if let Some(cursor) = self.draw_cursor() {
            out.push_str(cursor.as_ref());
        }
        Action::Draw(out)
    }

    /// Clear the cursor, if it's on screen.
    fn clear_cursor(&self, link: usize) -> Option<String> {
        if self.links.is_empty() || !self.is_visible(link) {
            return None;
        }
        let &pos = self.links.get(link)?;
        Some(format!(
            "{} ",
            cursor::Goto((self.indent() + 1) as u16, (pos + 1) as u16)
        ))
    }

    /// Print this string to draw the cursor on screen.
    /// Returns None if no is link selected.
    fn draw_cursor(&self) -> Option<String> {
        if self.links.is_empty() {
            return None;
        }
        let &pos = self.links.get(self.link)?;
        Some(format!(
            "{}\x1b[97;1m*\x1b[0m",
            cursor::Goto((self.indent() + 1) as u16, (pos + 1) as u16)
        ))
    }

    /// User input field.
    fn render_input(&self) -> String {
        format!(
            "{}Find:\x1b[0m {}{}{}",
            cursor::Goto(1, self.rows() as u16),
            self.input,
            cursor::Show,
            clear::UntilNewline,
        )
    }

    fn redraw_input(&self) -> Action {
        if self.searching {
            Action::Draw(self.render_input())
        } else {
            Action::Draw(format!(
                "{}{}{}",
                cursor::Goto(1, self.rows() as u16),
                clear::CurrentLine,
                cursor::Hide
            ))
        }
    }

    /// Scroll down by SCROLL_LINES, if possible.
    fn action_page_down(&mut self) -> Action {
        // If there are fewer menu items than screen lines, just
        // select the final link and do nothing else.
        if self.lines.len() < self.rows() {
            if !self.links.is_empty() {
                self.link = self.links.len() - 1;
                return Action::Redraw;
            }
            return Action::None;
        }

        // If we've already scrolled too far, select the final link
        // and do nothing.
        if self.scroll >= self.final_scroll() {
            self.scroll = self.final_scroll();
            self.link = self.links.len() - 1;
            return Action::Redraw;
        }

        // Scroll...
        self.scroll += SCROLL_LINES;

        // ...but don't go past the final line.
        if self.scroll > self.final_scroll() {
            self.scroll = self.final_scroll();
        }

        // If the selected link isn't visible...
        if Some(LinkPos::Above) == self.link_visibility(self.link) {
            // ...find the next one that is.
            if let Some(&next_link_pos) = self
                .links
                .iter()
                .skip(self.link + 1)
                .find(|&&i| i >= self.scroll)
            {
                if let Some(next_link_line) = self.lines.get(next_link_pos) {
                    self.link = next_link_line.link;
                }
            }
        }

        Action::Redraw
    }

    fn action_page_up(&mut self) -> Action {
        if self.scroll > 0 {
            if self.scroll > SCROLL_LINES {
                self.scroll -= SCROLL_LINES;
            } else {
                self.scroll = 0;
            }
            if self.link == 0 {
                return Action::Redraw;
            }
            if let Some(dir) = self.link_visibility(self.link) {
                match dir {
                    LinkPos::Below => {
                        let scroll = self.scroll;
                        if let Some(&pos) = self
                            .links
                            .iter()
                            .take(self.link)
                            .rev()
                            .find(|&&i| i < (self.rows() + scroll - 1))
                        {
                            self.link = self.lines.get(pos).unwrap().link;
                        }
                    }
                    LinkPos::Above => {}
                    LinkPos::Visible => {}
                }
            }
            Action::Redraw
        } else if self.link > 0 {
            self.link = 0;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_up(&mut self) -> Action {
        // no links, just scroll up
        if self.link == 0 {
            return if self.scroll > 0 {
                self.scroll -= 1;
                Action::Redraw
            } else if !self.links.is_empty() {
                self.link = self.links.len() - 1;
                self.scroll_to(self.link);
                Action::Redraw
            } else {
                Action::None
            };
        }

        // if text is entered, find previous match
        // TODO fix number input like this
        if self.searching && !self.input.is_empty() {
            if let Some(pos) = self.rlink_matching(self.link, &self.input) {
                return self.action_select_link(pos);
            } else {
                return Action::None;
            }
        }

        let new_link = self.link - 1;
        if let Some(dir) = self.link_visibility(new_link) {
            match dir {
                LinkPos::Above => {
                    // scroll up by 1
                    if self.scroll > 0 {
                        self.scroll -= 1;
                    }
                    // select it if it's visible now
                    if self.is_visible(new_link) {
                        self.link = new_link;
                    }
                }
                LinkPos::Below => {
                    // jump to link....
                    if let Some(&pos) = self.links.get(new_link) {
                        self.scroll = pos;
                        self.link = new_link;
                    }
                }
                LinkPos::Visible => {
                    // select next link up
                    let old_link = self.link;
                    self.link = new_link;
                    // scroll if we are within 5 lines of the top
                    if let Some(&pos) = self.links.get(self.link) {
                        if self.scroll > 0 && pos < self.scroll + 5 {
                            self.scroll -= 1;
                        } else {
                            // otherwise redraw just the cursor
                            return self.reset_cursor(old_link);
                        }
                    }
                }
            }
            Action::Redraw
        } else {
            Action::None
        }
    }

    /// Final `self.scroll` value.
    fn final_scroll(&self) -> usize {
        let padding = (self.rows() as f64 * 0.9) as usize;
        if self.lines.len() > padding {
            self.lines.len() - padding
        } else {
            0
        }
    }

    // search through links to find a match based on the pattern,
    // starting at link position `start`. returns the link position.
    fn link_matching(&self, start: usize, pattern: &str) -> Option<usize> {
        self.link_match_with_iter(pattern, &mut self.links.iter().skip(start))
    }

    // search backwards
    fn rlink_matching(&self, start: usize, pattern: &str) -> Option<usize> {
        self.link_match_with_iter(pattern, &mut self.links.iter().take(start).rev())
    }

    fn link_match_with_iter<'a, T>(&self, pattern: &str, it: &mut T) -> Option<usize>
    where
        T: std::iter::Iterator<Item = &'a usize>,
    {
        let pattern = pattern.to_ascii_lowercase();
        for &pos in it {
            let line = self.lines.get(pos)?;
            if line.name.to_ascii_lowercase().contains(&pattern) {
                return Some(line.link);
            }
        }
        None
    }

    fn action_down(&mut self) -> Action {
        let new_link = self.link + 1;

        // no links or final link selected already
        if self.links.is_empty() || new_link >= self.links.len() {
            // if there are more rows, scroll down
            if self.lines.len() >= self.rows() && self.scroll < self.final_scroll() {
                self.scroll += 1;
                return Action::Redraw;
            } else {
                // wrap around
                self.link = 0;
                self.scroll = 0;
                return Action::Redraw;
            }
        }

        // if text is entered, find next match
        if !self.input.is_empty() {
            if let Some(pos) = self.link_matching(self.link + 1, &self.input) {
                return self.action_select_link(pos);
            } else {
                return Action::None;
            }
        }

        if self.link < self.links.len() {
            if let Some(dir) = self.link_visibility(new_link) {
                match dir {
                    LinkPos::Above => {
                        // jump to link....
                        if let Some(&pos) = self.links.get(new_link) {
                            self.scroll = pos;
                            self.link = new_link;
                        }
                    }
                    LinkPos::Below => {
                        // scroll down by 1
                        self.scroll += 1;
                        // select it if it's visible now
                        if self.is_visible(new_link) {
                            self.link = new_link;
                        }
                    }
                    LinkPos::Visible => {
                        // link is visible, so select it
                        if let Some(&pos) = self.links.get(self.link) {
                            let old_link = self.link;
                            self.link = new_link;

                            // scroll if we are within 5 lines of the end
                            if self.lines.len() >= self.rows() // dont scroll if content too small
                                && pos >= self.scroll + self.rows() - 6
                            {
                                self.scroll += 1;
                            } else {
                                // otherwise try to just re-draw the cursor
                                return self.reset_cursor(old_link);
                            }
                        }
                    }
                }
                Action::Redraw
            } else {
                Action::None
            }
        } else {
            Action::None
        }
    }

    fn action_select_link(&mut self, link: usize) -> Action {
        if let Some(&pos) = self.links.get(link) {
            if !self.is_visible(link) {
                if pos > 5 {
                    self.scroll = pos - 5;
                } else {
                    self.scroll = 0;
                }
            }
            self.link = link;
            Action::Redraw
        } else {
            Action::None
        }
    }

    fn action_follow_link(&mut self, link: usize) -> Action {
        self.action_select_link(link);
        self.action_open()
    }

    fn scroll_to(&mut self, link: usize) -> Action {
        if !self.is_visible(link) {
            if let Some(&pos) = self.links.get(link) {
                if pos > 5 {
                    self.scroll = pos - 5;
                } else {
                    self.scroll = 0;
                }
                if self.scroll > self.final_scroll() {
                    self.scroll = self.final_scroll();
                }
                return Action::Redraw;
            }
        }
        Action::None
    }

    fn action_open(&mut self) -> Action {
        // if the selected link isn't visible, jump to it:
        if !self.is_visible(self.link) {
            return self.scroll_to(self.link);
        }

        self.searching = false;
        self.input.clear();

        if let Some(line) = self.link(self.link) {
            let url = line.url.to_string();
            let (typ, _, _, _) = gopher::parse_url(&url);
            match typ {
                Type::Search => {
                    let prompt = format!("{}> ", line.name);
                    Action::Prompt(
                        prompt.clone(),
                        Box::new(move |query| {
                            Action::Open(
                                format!("{}{}", prompt, query),
                                format!("{}?{}", url, query),
                            )
                        }),
                    )
                }
                Type::Error => Action::Error(line.name.to_string()),
                Type::Telnet => Action::Error("Telnet support coming soon".into()),
                t if !t.is_supported() => Action::Error(format!("{:?} not supported", t)),
                _ => Action::Open(line.name.to_string(), url),
            }
        } else {
            Action::None
        }
    }

    // self.searching == true
    fn process_search_mode_char(&mut self, c: char) -> Action {
        if c == '\n' {
            if self.link_matching(0, &self.input).is_some() {
                return self.action_open();
            } else {
                let input = self.input.clone();
                self.searching = false;
                self.input.clear();
                return Action::Error(format!("No links match: {}", input));
            }
        }

        self.input.push(c);
        if let Some(pos) = self.link_matching(0, &self.input) {
            self.action_select_link(pos)
        } else {
            self.redraw_input()
        }
    }

    fn process_key(&mut self, key: Key) -> Action {
        if self.searching {
            if let Key::Char(c) = key {
                return self.process_search_mode_char(c);
            }
        }

        match key {
            Key::Char('\n') => self.action_open(),
            Key::Up | Key::Ctrl('p') | Key::Char('p') | Key::Ctrl('k') | Key::Char('k') => {
                self.action_up()
            }
            Key::Down | Key::Ctrl('n') | Key::Char('n') | Key::Ctrl('j') | Key::Char('j') => {
                self.action_down()
            }
            Key::PageUp | Key::Ctrl('-') | Key::Char('-') => self.action_page_up(),
            Key::PageDown | Key::Ctrl(' ') | Key::Char(' ') => self.action_page_down(),
            Key::Home => {
                self.scroll = 0;
                self.link = 0;
                Action::Redraw
            }
            Key::End => {
                self.scroll = self.final_scroll();
                if !self.links.is_empty() {
                    self.link = self.links.len() - 1;
                }
                Action::Redraw
            }
            Key::Char('f') | Key::Ctrl('f') | Key::Char('/') | Key::Char('i') | Key::Ctrl('i') => {
                self.searching = true;
                Action::Redraw
            }
            Key::Char('w') | Key::Ctrl('w') => {
                self.wide = !self.wide;
                Action::Redraw
            }
            Key::Backspace | Key::Delete => {
                if self.searching {
                    self.input.pop();
                    self.redraw_input()
                } else {
                    Action::Keypress(key)
                }
            }
            Key::Esc | Key::Ctrl('c') => {
                if self.searching {
                    if self.input.is_empty() {
                        self.searching = false;
                    } else {
                        self.input.clear();
                    }
                    self.redraw_input()
                } else {
                    Action::Keypress(key)
                }
            }
            Key::Char(c) => {
                if !c.is_digit(10) {
                    return Action::Keypress(key);
                }

                self.input.push(c);
                // jump to number
                let s = self
                    .input
                    .chars()
                    .take(self.input.len())
                    .collect::<String>();
                if let Ok(num) = s.parse::<usize>() {
                    if num > 0 && num <= self.links.len() {
                        if self.links.len() < (num * 10) {
                            return self.action_follow_link(num - 1);
                        } else {
                            return self.action_select_link(num - 1);
                        }
                    }
                }

                Action::None
            }
            _ => Action::Keypress(key),
        }
    }

    // parse gopher response into a Menu object
    pub fn parse(url: String, raw: String) -> Menu {
        let mut lines = vec![];
        let mut links = vec![];
        let mut longest = 0;
        for line in raw.split_terminator('\n') {
            if let Some(c) = line.chars().nth(0) {
                let typ = match Type::from(c) {
                    Some(t) => t,
                    None => continue,
                };

                // assemble line info
                let parts: Vec<&str> = line.split_terminator('\t').collect();

                let mut name = String::from("");
                if !parts[0].is_empty() {
                    name.push_str(&parts[0][1..]);
                }
                if name.len() > longest {
                    longest = name.len();
                }
                // check for URL:<url> syntax
                if parts.len() > 1 && parts[1].starts_with("URL:") {
                    lines.push(Line {
                        name,
                        url: parts[1].trim_start_matches("URL:").to_string(),
                        typ,
                        link: links.len(),
                    });
                    if typ != Type::Info {
                        links.push(lines.len() - 1);
                    }
                    continue;
                }

                // assemble regular, gopher-style URL
                let mut url = String::from("gopher://");
                // host
                if parts.len() > 2 {
                    url.push_str(parts[2]);
                }
                // port
                if parts.len() > 3 {
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
                    // add trailing / if the selector is blank
                    if parts.is_empty() || parts.len() > 1 && parts[1].is_empty() {
                        url.push('/');
                    }
                }
                // selector
                if parts.len() > 1 {
                    let mut sel = parts[1].to_string();
                    if !sel.is_empty() {
                        if !sel.starts_with('/') {
                            sel.insert(0, '/');
                        }
                        url.push_str(&sel);
                    }
                }
                lines.push(Line {
                    name,
                    url,
                    typ,
                    link: links.len(),
                });
                if typ != Type::Info {
                    links.push(lines.len() - 1);
                }
            }
        }

        Menu {
            url,
            lines,
            links,
            longest,
            raw,
            input: String::new(),
            link: 0,
            scroll: 0,
            searching: false,
            size: (0, 0),
            wide: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse {
        ($s:literal) => {
            Menu::parse("test".to_string(), $s.to_string());
        };
    }

    #[test]
    fn test_simple_menu() {
        let menu = parse!(
            "
i---------------------------------------------------------
1SDF PHLOGOSPHERE (297 phlogs)	/phlogs/	gopher.club	70
1SDF GOPHERSPACE (1303 ACTIVE users)	/maps/	sdf.org	70
i---------------------------------------------------------
"
        );
        assert_eq!(menu.lines.len(), 4);
        assert_eq!(menu.links.len(), 2);
        assert_eq!(menu.lines[1].url, "gopher://gopher.club/1/phlogs/");
        assert_eq!(menu.lines[2].url, "gopher://sdf.org/1/maps/");
    }

    #[test]
    fn test_no_path() {
        let menu = parse!("1Circumlunar Space		circumlunar.space	70");
        assert_eq!(menu.links.len(), 1);
        assert_eq!(menu.lines[0].url, "gopher://circumlunar.space/1/");
    }

    #[test]
    fn test_find_links() {
        let mut menu = parse!(
            "
i________________________________G_O_P_H_E_R_______________________________	Err	bitreich.org	70
iHelp us building a nice sorted directory of the gopherspace:	Err	bitreich.org	70
1THE GOPHER LAWN – THE gopher directory	/lawn	bitreich.org	70
i	Err	bitreich.org	70
1Gopher Tutorials Project	/tutorials	bitreich.org	70
i	Err	bitreich.org	70
iRun more gopherholes on tor!	Err	bitreich.org	70
1The Gopher Onion Initiative	/onion	bitreich.org	70
i	Err	bitreich.org	70
1You are missing a gopher client? Use our kiosk mode.	/kiosk	bitreich.org	70
hssh kiosk@bitreich.org	URL:ssh://kiosk@bitreich.org	bitreich.org	70
i	Err	bitreich.org	70
iFurther gopherspace links:	Err	bitreich.org	70
1The Gopher Project	/	gopherproject.org	70
7Search the global gopherspace at Veronica II	/v2/vs	gopher.floodgap.com	70
i	Err	bitreich.org	70
iBest viewed using:	Err	bitreich.org	70
1sacc	/scm/sacc	bitreich.org	70
1clic	/scm/clic	bitreich.org	70
i	Err	bitreich.org	70"
        );
        menu.term_size(80, 40);

        assert_eq!(menu.links.len(), 9);
        assert_eq!(menu.link(0).unwrap().url, "gopher://bitreich.org/1/lawn");
        assert_eq!(
            menu.link(1).unwrap().url,
            "gopher://bitreich.org/1/tutorials"
        );
        assert_eq!(menu.link(2).unwrap().url, "gopher://bitreich.org/1/onion");
        assert_eq!(menu.link(3).unwrap().url, "gopher://bitreich.org/1/kiosk");
        assert_eq!(menu.link, 0);

        let ssh = menu.link(4).unwrap();
        assert_eq!(ssh.url, "ssh://kiosk@bitreich.org");
        assert_eq!(ssh.typ, Type::HTML);

        menu.action_down();
        assert_eq!(menu.link, 1);
        assert_eq!(menu.link(menu.link).unwrap().link, 1);

        menu.action_down();
        assert_eq!(menu.link, 2);
        assert_eq!(menu.link(menu.link).unwrap().link, 2);

        menu.action_page_down();
        assert_eq!(menu.link, 8);
        assert_eq!(menu.link(menu.link).unwrap().link, 8);

        menu.action_up();
        assert_eq!(menu.link, 7);
        assert_eq!(menu.link(menu.link).unwrap().link, 7);

        assert_eq!(menu.scroll, 0);
        menu.action_page_up();
        assert_eq!(menu.link, 0);
        assert_eq!(menu.link(menu.link).unwrap().link, 0);
    }
}
