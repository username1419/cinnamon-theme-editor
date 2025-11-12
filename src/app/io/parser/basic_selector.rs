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
    Highlight,
    Ltr,
    Rtl,
    /// Matches an element when the user drag and drop them. Only works in panel edit mode
    Dnd,
    ///
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
    /// Pseudo-class of the selector
    pseudo_class: Vec<PseudoClass>,
    /// Pseudo-element of the selector
    pseudo_element: Option<PseudoElement>,
}

impl BasicSelector {
    pub fn new(
        raw: String,
        pseudo_class: Vec<PseudoClass>,
        pseudo_element: Option<PseudoElement>,
    ) -> Self {
        Self {
            raw,
            pseudo_class,
            pseudo_element,
        }
    }

    pub fn from_raw(basic_selector: &str) -> Self {
        let mut base;
        let mut pseudo_class = Vec::new();
        let mut pseudo_element = None;
        let mut tracking_token = String::new();
        let mut chars = basic_selector.chars().peekable();
        let raw = basic_selector.to_string();

        while let Some(ch) = chars.peek() {
            // TODO: seriously this is a horrible way to deal with css errors
            if ':'.ne(ch) {
                tracking_token.push(chars.next().unwrap());
                if chars.peek().is_some() {
                    continue;
                }
            }

            base = tracking_token;
            chars.next();
            tracking_token = chars
                .clone()
                .take_while(|c| c.is_ascii_alphabetic() || '-'.eq(c))
                .collect();

            let ch_ne = chars.peek();

            if ch_ne.is_some() && !tracking_token.is_empty() {
                pseudo_class.push(
                    PseudoClass::from_str(tracking_token.as_str()).expect(
                        format!(
                            "Failed to unwrap token \"{tracking_token}\"\nRaw: \"{basic_selector}\""
                        )
                        .as_str(),
                    ),
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

            for _ in [0..tracking_token.len()] {
                chars.next();
            }
        }

        Self {
            raw,
            pseudo_class,
            pseudo_element,
        }
    }
}
