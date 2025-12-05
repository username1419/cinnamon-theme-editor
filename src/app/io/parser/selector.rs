use super::basic_selector::BasicSelector;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SelectorType {
    Complex,
    Compound,
    Simple,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SelectorCategory {
    Panel,
    Menu,
    Window,
    Calendar,
    Dialog,
    Entry,
    Sound,
    GroupWindow,
    Other,
}

impl SelectorCategory {
    // i hate this
    // but we already have like 6 deps so we cant just use strum
    pub const VALUES: [SelectorCategory; 9] = [
        Self::Panel,
        Self::Menu,
        Self::Window,
        Self::Calendar,
        Self::Dialog,
        Self::Entry,
        Self::Sound,
        Self::GroupWindow,
        Self::Other,
    ];
}

impl Default for SelectorCategory {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Combinator {
    NextSibling,
    Child,
    SubsequentSibling,
    Descendant,
    None,
}

impl Combinator {
    pub fn try_match(combinator: &str) -> Self {
        if combinator.is_empty() {
            return Self::None;
        }
        match combinator.trim() {
            "" => Self::Descendant,
            ">" => Self::Child,
            "~" => Self::SubsequentSibling,
            "+" => Self::NextSibling,
            _ => panic!("Unknown combinator {}", combinator),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Selector {
    /// The selector's raw contents
    raw: String,
    /// The selector type of the instance
    selector_type: SelectorType,
    /// The category of the selector
    selector_category: SelectorCategory,
    /// A list of basic selectors and their next combinator. List combinators are not supported.
    selectors: Vec<(BasicSelector, Combinator)>,
}

impl Selector {
    pub fn from_raw(raw: &str) -> Self {
        let selector_type = Self::get_type(raw);
        let selector_category = Self::get_category(raw);
        let mut selectors = Vec::new();
        let mut chars = raw.chars().peekable();
        let raw = raw.to_string().clone();
        let mut tracking_selector = String::new();

        while let Some(ch) = chars.peek() {
            if !(">~+".contains(*ch) || ch.is_whitespace()) {
                tracking_selector.push(chars.next().unwrap());
                if chars.peek().is_some() {
                    continue;
                }
            }

            let combinator = chars
                .clone()
                .take_while(|c: &char| ">~+".contains(*c) || c.is_whitespace())
                .collect::<String>();
            for _ in [0..combinator.len()] {
                chars.next();
            }
            selectors.push((
                BasicSelector::from_raw(tracking_selector.as_str()),
                Combinator::try_match(combinator.as_str()),
            ));
            tracking_selector.clear();
        }

        Self {
            raw,
            selector_type,
            selector_category,
            selectors,
        }
    }

    /// Returns the type of selector given a basic selector. Panics if the input is not trimmed.
    ///
    /// # Examples
    /// ```
    /// let compound_selector = String.from("p#id.class");
    /// assert_eq!(SelectorType::Compound, Selector::get_type(selector));
    ///
    /// let simple_selector = String.from(".id");
    /// assert_eq!(SelectorType::Simple, Selector::get_type(simple_selector));
    ///
    /// let complex_selector = String::from("p a#id");
    /// assert_eq!(SelectorType::Complex, Selector::get_type(complex_selector));
    ///
    /// let wildcard = String::from("*");
    /// assert_eq!(SelectorType::Wildcard, Selector::get_type(wildcard));
    /// ```
    pub fn get_type(selector: &str) -> SelectorType {
        if selector.starts_with(' ') || selector.ends_with(' ') {
            panic!();
        }

        let mut match_char_count: u16 = 0; // please dont have more complex selectors than
        // this
        let mut whitespace_before = false;
        let mut is_complex = false;
        selector.chars().for_each(|c| match c {
            ' ' => whitespace_before = true,
            c => {
                match c {
                    '.' => match_char_count += 1,
                    '#' => match_char_count += 1,
                    _ => (),
                }

                if whitespace_before {
                    is_complex = true
                }
            }
        });

        let individual_selector_count = match_char_count
            + if selector.starts_with(|c| c != '.' && c != '#') {
                1
            } else {
                0
            };

        if is_complex {
            SelectorType::Complex
        } else if individual_selector_count > 1 {
            SelectorType::Compound
        } else {
            SelectorType::Simple
        }
    }

    /// Tries to assign a category to the selector based on its name. If the assignment fails, the
    /// category is assigned to `SelectorCategory::Other`.
    ///
    /// The function searches for containing substrings matching each of the categories using regex
    /// and selects the last match while prioritizing id(#) matches.
    ///
    /// The results of this categorization can be changed sporadically and should only be used to
    /// separate different selectors for users' ease of use.
    ///
    /// # Examples
    /// ```
    /// let selector = ".popup-menu-item";
    /// println!("{:?}", Selector::get_category(selector)); // SelectorCategory::Menu
    /// ```
    pub fn get_category(selector: &str) -> SelectorCategory {
        let mut category = SelectorCategory::Other;
        let mut priority_seen = false;

        const PATTERNS: &[(&str, SelectorCategory)] = &[
            ("panel", SelectorCategory::Panel),
            ("menu", SelectorCategory::Menu),
            ("window", SelectorCategory::Window),
            ("calendar", SelectorCategory::Calendar),
            ("dialog", SelectorCategory::Dialog),
            ("entry", SelectorCategory::Entry),
            ("sound", SelectorCategory::Sound),
            ("grouped-window", SelectorCategory::GroupWindow),
        ];

        for &(pattern, cat) in PATTERNS {
            if let Some(pos) = selector.find(pattern) {
                let is_id = pos > 0 && &selector[pos - 1..pos] == "#";
                if is_id || !priority_seen {
                    category = cat;
                    if is_id {
                        priority_seen = true;
                    }
                }
            }
        }

        category
    }
}
