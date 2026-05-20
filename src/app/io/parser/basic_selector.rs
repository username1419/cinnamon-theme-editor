use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum BasicSelectorType {
    Type,
    Class,
    Id,
    Universal,
}

impl BasicSelectorType {
    pub fn get_type_of(selector: &String) -> BasicSelectorType {
        let mut binding = selector.chars().peekable();
        let ch = binding.peek().unwrap();
        match ch {
            '#' => BasicSelectorType::Id,
            '.' => BasicSelectorType::Class,
            // basic selectors have weird valid beginning chars huh
            c => {
                if '_'.eq(c) || c.is_ascii_alphabetic() || c.to_digit(10).unwrap_or_default() > 80 {
                    BasicSelectorType::Type
                } else if '*'.eq(c) {
                    BasicSelectorType::Universal
                } else {
                    panic!("Invalid basic selector \"{}\"", selector)
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum PseudoElement {
    Before,
    After,
    Other(String),
}

impl PseudoElement {
    /// Tries to match a string to a PseudoElement enum.
    ///
    /// # Examples
    /// ```
    /// let part_fail = "::before";
    /// assert_eq(None, PseudoElement::from_str(part_fail));
    ///
    /// let part_pass = "before";
    /// assert_eq(Some(PseudoElement::Before), PseudoElement::from_str(part_pass));
    ///
    /// let part_unrecognized = "first-line";
    /// assert_eq(Some(PseudoElement::Other("first-line".to_string())), PseudoElement::from_str(part_unrecognized));
    /// ```
    fn from_str(pseudo_element: &str) -> Option<PseudoElement> {
        if pseudo_element.starts_with(|c: char| !c.is_alphabetic()) {
            return None;
        }
        match pseudo_element {
            "before" => Some(Self::Before),
            "after" => Some(Self::After),
            _ => Some(Self::Other(pseudo_element.to_string())),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
/// Represents a Pseudo-class inside a selector. Eg. in `StLabel:ltr`, `ltr` is the pseudo-class.
pub enum PseudoClass {
    /// Matches when cursor hovers over the element
    Hover,
    Active,
    /// Matches when element is enabled
    Enabled,
    /// Matches when element is disabled
    Disabled,
    /// Matches when element is focused, eg. with <Tab> key
    Focus,
    /// Matches when a radio, checkbox, or option element is checked
    Checked,
    /// found in `.applet-box`
    Highlight,
    Ltr,
    Rtl,
    /// Matches an element when the user drag and drop them. Only works in panel edit mode
    Dnd,
    /// i got no clue what this does
    Entered,
    /// Calendar exclusive
    AllDay,
    /// Calendar exclusive
    Soon,
    /// Calendar exclusive
    Imminent,
    /// Calendar exclusive
    Alternate,
    /// Calendar exclusive
    Insensitive,
    /// Calendar exclusive
    Current,
    /// Button pseudoclass for something idk
    DestructiveAction,
    /// found in `.window-list-item-box`
    GroupFocus,
    /// found in `.window-list-item-box`
    Running,
    /// found in `.item-box`
    Outlined,
    /// found in `.item-box`
    Selected,
    /// found in `StBoxLayout`
    FirstChild,
    /// found in `StBoxLayout`
    Small,
    /// found in `.workspace-button`
    Shaded,
    /// found in `.tile-hud`
    Top,
    /// found in `.tile-hud`
    Bottom,
    /// found in `.tile-hud`
    Left,
    /// found in `.tile-hud`
    Right,
    /// found in `.tile-hud`
    TopLeft,
    /// found in `.tile-hud`
    TopRight,
    /// found in `.tile-hud`
    BottomLeft,
    /// found in `.tile-hud`
    BottomRight,

    Other(String),
}

impl PseudoClass {
    /// Tries to match a string to a PseudoClass enum.
    ///
    /// # Examples
    /// ```
    /// let part_fail = "::hover";
    /// assert_eq(None, PseudoClass::from_str(part_fail));
    ///
    /// let part_pass = "hover";
    /// assert_eq(Some(PseudoClass::Hover), PseudoClass::from_str(part_pass));
    ///
    /// let part_unrecognized = "target";
    /// assert_eq(Some(PseudoClass::Other("target".to_string())), PseudoClass::from_str(part_unrecognized));
    /// ```
    fn from_str(pseudo_class: &str) -> Option<PseudoClass> {
        if pseudo_class.starts_with(|c: char| !c.is_alphabetic()) {
            return None;
        }
        match pseudo_class {
            "hover" => Some(Self::Hover),
            "active" => Some(Self::Active),
            "enabled" => Some(Self::Enabled),
            "disabled" => Some(Self::Disabled),
            "focus" => Some(Self::Focus),
            "checked" => Some(Self::Checked),
            "highlight" => Some(Self::Highlight),
            "ltr" => Some(Self::Ltr),
            "rtl" => Some(Self::Rtl),
            "dnd" => Some(Self::Dnd),
            "entered" => Some(Self::Entered),
            "allday" => Some(Self::AllDay),
            "soon" => Some(Self::Soon),
            "imminent" => Some(Self::Imminent),
            "alternate" => Some(Self::Alternate),
            "insensitive" => Some(Self::Insensitive),
            "current" => Some(Self::Current),
            "destructive-action" => Some(Self::DestructiveAction),
            _ => Some(Self::Other(pseudo_class.to_string())),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct BasicSelector {
    /// The selector's raw contents
    raw: String,
    /// The type of basic selector. Eg.: Class, Id, Type. Not to be confused with
    /// Selector::selector_type
    selector_type: BasicSelectorType,
    /// The selector's default selector type, if it has been converted by
    /// Selector::to_webview_safe(). Otherwise None.
    default_selector_type: Option<BasicSelectorType>,
    /// Pseudo-class of the selector
    pseudo_class: Vec<PseudoClass>,
    /// Pseudo-element of the selector
    pseudo_element: Option<PseudoElement>,
}

impl BasicSelector {
    pub fn new(
        raw: String,
        selector_type: BasicSelectorType,
        default_selector_type: Option<BasicSelectorType>,
        pseudo_class: Vec<PseudoClass>,
        pseudo_element: Option<PseudoElement>,
    ) -> Self {
        Self {
            raw,
            pseudo_class,
            pseudo_element,
            selector_type,
            default_selector_type,
        }
    }

    pub fn from_raw(basic_selector: &str) -> Self {
        let mut basic_selector = basic_selector;
        if let Some(c) = basic_selector.chars().peekable().peek()
            && c.is_whitespace()
        {
            basic_selector = basic_selector.trim();
        }
        let mut pseudo_class = Vec::new();
        let mut pseudo_element = None;
        let mut tracking_token = String::new();
        let mut chars = basic_selector.chars().peekable();
        let raw = basic_selector.to_string();
        let selector_type = BasicSelectorType::get_type_of(&raw);

        while let Some(ch) = chars.peek() {
            // TODO: seriously this is a horrible way to deal with css errors
            if ':'.ne(ch) {
                tracking_token.push(chars.next().unwrap());
                if chars.peek().is_some() {
                    continue;
                }
            }

            chars.next();
            tracking_token = chars
                .clone()
                .take_while(|c| c.is_ascii_alphabetic() || '-'.eq(c))
                .collect();

            let ch_ne = chars.peek();

            if ch_ne.is_some() && !tracking_token.is_empty() {
                pseudo_class.push(
                    PseudoClass::from_str(tracking_token.as_str()).unwrap_or_else(|| {
                        panic!(
                            "Failed to unwrap token \"{tracking_token}\"\nRaw: \"{basic_selector}\""
                        )
                    }),
                );
                tracking_token.clear();
            } else if ch_ne.is_some() {
                chars.next();
                tracking_token = chars
                    .clone()
                    .take_while(|c| c.is_ascii_alphabetic() || '-'.eq(c))
                    .collect();
                pseudo_element = PseudoElement::from_str(tracking_token.as_str());
                tracking_token.clear();
            }

            for _ in (0..tracking_token.len()).collect::<std::vec::Vec<usize>>() {
                chars.next();
            }
        }

        Self {
            raw,
            pseudo_class,
            pseudo_element,
            selector_type,
            default_selector_type: None,
        }
    }

    pub fn get_raw(&self) -> &String {
        &self.raw
    }

    pub fn get_default_selector_type(&self) -> Option<&BasicSelectorType> {
        self.default_selector_type.as_ref()
    }

    pub fn set_default_selector_type(&mut self, default_selector_type: Option<BasicSelectorType>) {
        self.default_selector_type = default_selector_type;
    }

    pub fn get_selector_type(&self) -> &BasicSelectorType {
        &self.selector_type
    }
}

impl Display for BasicSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.raw)
    }
}
