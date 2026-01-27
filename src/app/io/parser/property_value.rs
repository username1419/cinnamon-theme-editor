#[derive(Clone, Debug)]
pub struct Value {
    raw: String,
    value: String,
    unit: ValueUnit,
}

#[derive(Clone, Debug)]
pub enum ValueUnit {
    None,
    Px,
    Em,
    Rem,
    Percent,
    Other,
}

impl ValueUnit {
    pub fn match_end(raw: &str) -> Self {
        let mut category = ValueUnit::Other;

        const PATTERNS: [(&str, ValueUnit); 4] = [
            ("px", ValueUnit::Px),
            ("em", ValueUnit::Em),
            ("rem", ValueUnit::Rem),
            ("%", ValueUnit::Percent),
        ];

        for (pattern, cat) in PATTERNS {
            if raw.ends_with(pattern) {
                category = cat.clone();
            }
        }

        category
    }
}

impl Value {
    /// # Exmaples
    /// ```
    /// let values = "13px 9px 13px 9px";
    /// let parsed = Value::from_raw(values);
    ///
    /// assert_eq!(parsed.len(), 4);
    /// ```
    pub fn from_raw(raw: &str) -> Vec<Self> {
        let mut collection = Vec::new();
        let mut chars = raw.chars();
        let mut value = String::new();

        while let Some(ch) = chars.next() {
            if ch.is_whitespace() {
                if !value.is_empty() {
                    collection.push(Self::from_raw_single(value.as_str()));
                    value.clear();
                }
                continue;
            }

            value.push(ch);
            if matches!(ch, '(') {
                while let Some(ch_inner) = chars.next() {
                    value.push(ch_inner);
                    if matches!(ch_inner, ')') {
                        let ne = chars.next();
                        assert!(ne.is_none_or(|c| c.is_whitespace()));
                        collection.push(Self::from_raw_single(value.as_str()));
                        value.clear();
                    }
                }
            }
        }

        if !value.is_empty() {
            collection.push(Self::from_raw_single(value.as_str()));
        }

        collection
    }

    /// # Exmaples
    /// ```
    /// let value = Value::from_raw_single("13px");
    /// let check = Value {
    ///     raw: "13px".to_string(),
    ///     value: "13".to_string(),
    ///     unit: ValueUnit::Px,
    /// };
    ///
    /// assert_eq!(value, check);
    /// ```
    pub fn from_raw_single(raw: &str) -> Self {
        let raw = raw.to_string();
        let is_num = raw.starts_with(|c: char| c.is_numeric());
        let value = raw
            .chars()
            .take_while(|c| {
                if is_num {
                    c.is_numeric() || matches!(c, '.')
                } else {
                    true
                }
            })
            .collect();
        let unit = if is_num {
            ValueUnit::match_end(raw.as_str())
        } else {
            ValueUnit::None
        };

        Self { raw, value, unit }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}
