//! The Menu is a View representing a Gopher menu. It renders the
//! colorful representation, manages the cursor and selection state,
//! and responds to input like the UP and DOWN arrows or other key
//! combinations.
//!
//! The Menu doesn't draw or perform any actions on its own, instead
//! it returns an Action to the UI representing its intent.

use crate::{
    config::Config,
    gopher::{self, Type},
    terminal,
    ui::{self, Action, Key, View, MAX_COLS, SCROLL_LINES},
};
use std::fmt;

/// The Menu holds our Gopher Lines, a list of links, and maintains
/// both where the cursor is on screen and which lines need to be
/// drawn on screen. While the main UI can be used to prompt the user
/// for input, the Menu maintains its own `input` for the "quick
/// navigation" feature using number entry and the "incremental search"
/// (over menu links) feature using text entry.
pub struct Menu {
    /// Gopher URL
    pub url: String,
    /// Lines in the menu. Not all are links.
    pub lines: Vec<Line>,
    /// Indexes of links in the `lines` vector. Pauper's pointers.
    pub links: Vec<usize>,
    /// Currently selected link. Index of the `links` vec.
    pub link: usize,
    /// Size of the longest line, for wrapping purposes
    pub longest: usize,
    /// Actual Gopher response
    pub raw: String,
    /// User input on a prompt() line
    pub input: String,
    /// UI mode. Interactive (Run), Printing, Raw mode...
    pub mode: ui::Mode,
    /// Scrolling offset, in rows.
    pub scroll: usize,
    /// Incremental search mode?
    pub searching: bool,
    /// Was this menu retrieved via TLS?
    tls: bool,
    /// Retrieved via Tor?
    tor: bool,
    /// Size of the screen currently, cols and rows
    pub size: (usize, usize),
    /// Wide mode?
    wide: bool,
}

/// The Line represents a single line in a Gopher menu.
/// It must exist in the context of a Menu struct - its `link`
/// field is its index in the Menu's `links` Vec, and
/// start/end/text_end point to locations in Menu's `raw` Gopher
/// response.
pub struct Line {
    /// Gopher Item Type.
    pub typ: Type,
    /// Where this line starts in its Menu's `raw` Gopher response.
    start: usize,
    /// Where this line ends in Menu.raw.
    end: usize,
    /// Where the text/label of this line ends. Might be the same as
    /// `end`, or might be earlier.
    text_end: usize,
    /// Length of visible text, ignoring ANSI escape codes (colors).
    visible_len: usize,
    /// How many chars() to grab from text() if we want to only show
    /// `MAX_COLS` visible chars on screen, aka ignore ANSI escape
    /// codes and colors.
    truncated_len: usize,
    /// Index of this link in the Menu::links vector, if it's a
    /// `gopher::Type.is_link()`
    pub link: usize,
}

impl Line {
    /// Returns the text field of this line, given a raw Gopher response.
    /// The same Line must always be used with the same Gopher response.
    pub fn text<'a>(&self, raw: &'a str) -> &'a str {
        if self.start < self.text_end {
            &raw[self.start + 1..self.text_end]
        } else {
            ""
        }
    }

    /// Get the length of this line's text field.
    pub fn text_len(&self) -> usize {
        self.visible_len
    }

    /// Get the URL for this line, if it's a link.
    pub fn url(&self, raw: &str) -> String {
        if !self.typ.is_link() || self.text_end >= self.end {
            return String::from("");
        }

        let line = &raw[self.text_end..self.end].trim_end_matches('\r');
        let mut sel = "(null)";
        let mut host = "localhost";
        let mut port = "70";
        for (i, chunk) in line.split('\t').enumerate() {
            match i {
                0 => {}
                1 => sel = chunk,
                2 => host = chunk,
                3 => port = chunk,
                _ => break,
            }
        }

        if self.typ.is_html() {
            sel.trim_start_matches('/')
                .trim_start_matches("URL:")
                .to_string()
        } else if self.typ.is_telnet() {
            format!("telnet://{}:{}", host, port)
        } else {
            let mut path = format!("/{}{}", self.typ, sel);
            if sel.is_empty() || sel == "/" {
                path.clear();
            }
            if port == "70" {
                format!("gopher://{}{}", host, path)
            } else {
                format!("gopher://{}:{}{}", host, port, path)
            }
        }
    }
}

/// Direction of a given link relative to the visible screen.
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
    fn is_tls(&self) -> bool {
        self.tls
    }

    fn is_tor(&self) -> bool {
        self.tor
    }

    fn raw(&self) -> &str {
        self.raw.as_ref()
    }

    fn render(&mut self) -> String {
        self.render_lines()
    }

    fn respond(&mut self, key: Key) -> Action {
        self.process_key(key)
    }

    fn set_wide(&mut self, wide: bool) {
        self.wide = wide;
    }

    fn wide(&mut self) -> bool {
        self.wide
    }

    fn term_size(&mut self, cols: usize, rows: usize) {
        self.size = (cols, rows);
    }

    fn url(&self) -> &str {
        self.url.as_ref()
    }
}

impl Menu {
    /// Create a representation of a Gopher Menu from a raw Gopher
    /// response and a few options.
    pub fn from(url: &str, response: String, config: &Config, tls: bool) -> Menu {
        Menu {
            tls,
            tor: config.tor,
            wide: config.wide,
            mode: config.mode,
            ..parse(url, response)
        }
    }

    fn cols(&self) -> usize {
        self.size.0
    }

    fn rows(&self) -> usize {
        self.size.1
    }

    /// Calculated size of left margin.
    fn indent(&self) -> usize {
        if self.wide {
            return 0;
        }
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

    /// Find a link by its link index.
    fn link(&self, i: usize) -> Option<&Line> {
        let line = self.links.get(i)?;
        self.lines.get(*line)
    }

    /// Is the given link visible on screen?
    fn is_visible(&self, link: usize) -> bool {
        self.link_visibility(link) == Some(LinkPos::Visible)
    }

    /// Where is the given link relative to the screen?
    fn link_visibility(&self, i: usize) -> Option<LinkPos> {
        let &pos = self.links.get(i)?;
        Some(if pos < self.scroll {
            LinkPos::Above
        } else if pos >= self.scroll + self.rows() - 1 {
            LinkPos::Below
        } else {
            LinkPos::Visible
        })
    }

    /// The x and y position of a given link on screen.
    fn screen_coords(&self, link: usize) -> Option<(u16, u16)> {
        if !self.is_visible(link) {
            return None;
        }
        let &pos = self.links.get(link)?;
        let x = self.indent() + 1;
        let y = if self.scroll > pos {
            pos + 1
        } else {
            pos + 1 - self.scroll
        };

        Some((x as u16, y as u16))
    }

    fn render_lines(&mut self) -> String {
        let mut out = String::new();
        let limit = if self.mode == ui::Mode::Run {
            // only show as many lines as screen rows minus one
            // (status bar is always last line)
            self.rows() - 1
        } else {
            self.lines.len()
        };
        let iter = self.lines.iter().skip(self.scroll).take(limit);
        let indent = self.indent();
        let left_margin = " ".repeat(indent);

        for line in iter {
            out.push_str(&left_margin);

            if line.typ == Type::Info {
                out.push_str("      ");
            } else {
                if line.link == self.link && self.show_cursor() {
                    out.push_str(color!(Bold));
                    out.push('*');
                    out.push_str(color!(Reset));
                } else {
                    out.push(' ');
                }
                out.push(' ');
                out.push_str(color!(Magenta));
                if line.link < 9 {
                    out.push(' ');
                }
                let num = (line.link + 1).to_string();
                out.push_str(&num);
                out.push_str(". ");
                out.push_str(color!(Reset));
            }

            // truncate long lines, instead of wrapping
            let text = line
                .text(&self.raw)
                .chars()
                .take(line.truncated_len)
                .collect::<String>();

            // color the line
            if line.typ.is_download() {
                out.push_str(color!(Underline));
                out.push_str(color!(White));
            } else if !line.typ.is_supported() {
                out.push_str(color!(WhiteBG));
                out.push_str(color!(Red));
            } else {
                out.push_str(&match line.typ {
                    Type::Text => color!(Cyan),
                    Type::Menu => color!(Blue),
                    Type::Info => color!(Yellow),
                    Type::HTML => color!(Green),
                    Type::Error => color!(Red),
                    Type::Telnet => color!(Grey),
                    Type::Search => color!(White),
                    _ => color!(Red),
                });
            }
            out.push_str(&text);
            out.push_str(color!(Reset));

            // clear rest of line
            out.push_str(terminal::ClearUntilNewline.as_ref());
            out.push_str("\r\n");
        }

        // clear remainder of screen
        out.push_str(terminal::ClearAfterCursor.as_ref());

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
        let (x, y) = self.screen_coords(link)?;
        Some(format!("{} {}", terminal::Goto(x, y), terminal::HideCursor))
    }

    /// Print this string to draw the cursor on screen.
    /// Returns None if no is link selected.
    fn draw_cursor(&self) -> Option<String> {
        if self.links.is_empty() || !self.show_cursor() {
            return None;
        }
        let (x, y) = self.screen_coords(self.link)?;
        Some(format!(
            "{}\x1b[97;1m*\x1b[0m{}",
            terminal::Goto(x, y),
            terminal::HideCursor
        ))
    }

    /// Should we show the cursor? Not when printing.
    fn show_cursor(&self) -> bool {
        self.mode == ui::Mode::Run
    }

    /// User input field.
    fn render_input(&self) -> String {
        format!("Find: {}{}", self.input, terminal::ShowCursor)
    }

    fn redraw_input(&self) -> Action {
        if self.searching {
            Action::Status(self.render_input())
        } else {
            Action::Status(terminal::HideCursor.to_string())
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
            if !self.links.is_empty() {
                self.link = self.links.len() - 1;
            }
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

    /// Search through links to find a match based on the pattern,
    /// starting at link position `start`. returns the link position.
    fn link_matching(&self, start: usize, pattern: &str) -> Option<usize> {
        self.link_match_with_iter(pattern, &mut self.links.iter().skip(start))
    }

    /// Search backwards through all links.
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
            if line.text(&self.raw).to_ascii_lowercase().contains(&pattern) {
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
            } else if !self.links.is_empty() {
                // wrap around
                self.link = 0;
                self.scroll = 0;
                return Action::Redraw;
            }
        }

        // if text is entered, find next match
        if self.searching && !self.input.is_empty() {
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

    /// Select and optionally scroll to a link.
    fn action_select_link(&mut self, link: usize) -> Action {
        if let Some(&pos) = self.links.get(link) {
            let old_link = self.link;
            self.link = link;
            if self.is_visible(link) {
                if !self.input.is_empty() {
                    Action::List(vec![self.redraw_input(), self.reset_cursor(old_link)])
                } else {
                    self.reset_cursor(old_link)
                }
            } else {
                if pos > 5 {
                    self.scroll = pos - 5;
                } else {
                    self.scroll = 0;
                }
                if !self.input.is_empty() {
                    Action::List(vec![self.redraw_input(), Action::Redraw])
                } else {
                    Action::Redraw
                }
            }
        } else {
            Action::None
        }
    }

    /// Select and open link.
    fn action_follow_link(&mut self, link: usize) -> Action {
        self.action_select_link(link);
        self.action_open()
    }

    /// Scroll to a link if it's not visible.
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

    /// Open the currently selected link.
    fn action_open(&mut self) -> Action {
        // if the selected link isn't visible, jump to it:
        if !self.is_visible(self.link) {
            return self.scroll_to(self.link);
        }

        self.searching = false;
        self.input.clear();

        if let Some(line) = self.link(self.link) {
            let url = line.url(&self.raw);
            let typ = gopher::type_for_url(&url);
            match typ {
                Type::Search => {
                    let prompt = format!("{}> ", line.text(&self.raw));
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
                Type::Error => Action::Error(line.text(&self.raw).to_string()),
                t if !t.is_supported() => Action::Error(format!("{:?} not supported", t)),
                _ => Action::Open(line.text(&self.raw).to_string(), url),
            }
        } else {
            Action::None
        }
    }

    /// self.searching == true
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

    /// Respond to user input.
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
                self.input.clear();
                self.redraw_input()
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
                    .take(self.input.chars().count())
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
}

/// Parse gopher response into a Menu object.
pub fn parse(url: &str, raw: String) -> Menu {
    let mut lines = vec![];
    let mut links = vec![];
    let mut longest = 0;
    let mut start = 0;

    for line in raw.split_terminator('\n') {
        // Check for Gopher's weird "end of response" message.
        if line == ".\r" || line == "." {
            break;
        }

        if line == "" {
            start += 1;
            continue;
        }

        if let Some(mut line) = parse_line(start, &raw) {
            if line.text_len() > longest {
                longest = line.text_len();
            }
            if line.typ.is_link() {
                line.link = links.len();
                links.push(lines.len());
            }
            lines.push(line);
        }

        start += line.len() + 1;
    }

    Menu {
        url: url.into(),
        lines,
        links,
        longest,
        raw,
        input: String::new(),
        link: 0,
        mode: Default::default(),
        scroll: 0,
        searching: false,
        size: (0, 0),
        tls: false,
        tor: false,
        wide: false,
    }
}

/// Parses a single line from a Gopher menu into a `Line` struct.
pub fn parse_line(start: usize, raw: &str) -> Option<Line> {
    if raw.is_empty() || start >= raw.len() {
        return None;
    }

    let line = &raw[start..];
    let end = line.find('\n').unwrap_or_else(|| line.chars().count()) + start;
    let line = &raw[start..end]; // constrain \t search
    let text_end = if let Some(i) = line.find('\t') {
        i + start
    } else if let Some(i) = line.find('\r') {
        i + start
    } else {
        end
    };
    let typ = Type::from(line.chars().nth(0)?)?;

    // calculate the visible length of this line as well as where to
    // truncate it when abiding by `MAX_COLS`
    let mut is_color = false;
    let mut truncated_len = 0;
    let mut visible_len = 0;
    let mut iter = raw[start..text_end].char_indices().peekable();

    while let Some((i, c)) = iter.next() {
        if is_color {
            if c == 'm' {
                is_color = false;
            }
        } else {
            if c == '\x1b' {
                if let Some((_, '[')) = iter.peek() {
                    iter.next(); // skip [
                    is_color = true;
                }
            } else {
                if visible_len < MAX_COLS {
                    truncated_len = i;
                }
                visible_len += 1;
            }
        }
    }

    Some(Line {
        start,
        end,
        text_end,
        truncated_len,
        visible_len,
        typ,
        link: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse {
        ($s:literal) => {
            parse("test", $s.to_string());
        };
    }

    #[test]
    fn test_simple_menu() {
        let menu = parse!(
            "
i---------------------------------------------------------
1SDF PHLOGOSPHERE (297 phlogs)	/phlogs/	gopher.club	70
1SDF GOPHERSPACE (1303 ACTIVE users)	/maps/	sdf.org	70
1Geosphere	Geosphere	earth.rice.edu
iwacky links
i-----------	spacer
8DJ's place	a	bbs.impakt.net	6502
hgit tree	/URL:https://github.com/my/code	(null)	70
i-----------	spacer	localhost	70
i---------------------------------------------------------
"
        );
        assert_eq!(menu.lines.len(), 10);
        assert_eq!(menu.links.len(), 5);
        assert_eq!(
            menu.lines[1].url(&menu.raw),
            "gopher://gopher.club/1/phlogs/"
        );
        assert_eq!(menu.lines[2].url(&menu.raw), "gopher://sdf.org/1/maps/");
        assert_eq!(
            menu.lines[3].url(&menu.raw),
            "gopher://earth.rice.edu/1Geosphere"
        );
        assert_eq!(menu.lines[4].text(&menu.raw), "wacky links");
        assert_eq!(menu.lines[5].text(&menu.raw), "-----------");
        assert_eq!(menu.lines[6].url(&menu.raw), "telnet://bbs.impakt.net:6502");
        assert_eq!(menu.lines[7].url(&menu.raw), "https://github.com/my/code");
        assert_eq!(menu.lines[8].text(&menu.raw), "-----------");
    }

    #[test]
    fn test_no_path() {
        let menu = parse!("1Circumlunar Space		circumlunar.space	70");
        assert_eq!(menu.links.len(), 1);
        assert_eq!(menu.lines[0].url(&menu.raw), "gopher://circumlunar.space");
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
i	Err	bitreich.org	70
"
        );
        menu.term_size(80, 40);

        assert_eq!(menu.links.len(), 9);
        assert_eq!(
            menu.link(0).unwrap().url(&menu.raw),
            "gopher://bitreich.org/1/lawn"
        );
        assert_eq!(
            menu.link(1).unwrap().url(&menu.raw),
            "gopher://bitreich.org/1/tutorials"
        );
        assert_eq!(
            menu.link(2).unwrap().url(&menu.raw),
            "gopher://bitreich.org/1/onion"
        );
        assert_eq!(
            menu.link(3).unwrap().url(&menu.raw),
            "gopher://bitreich.org/1/kiosk"
        );
        assert_eq!(menu.link, 0);

        let ssh = menu.link(4).unwrap();
        assert_eq!(ssh.url(&menu.raw), "ssh://kiosk@bitreich.org");
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
