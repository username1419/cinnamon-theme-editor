use std::fmt::Display;

/// A CSS property name with a coarse category used by the property editor UI.
#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    raw: String,
    category: PropertyCategory,
}

/// Broad grouping for properties (background, border, etc.).
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyCategory {
    Background,
    Foreground,
    Padding,
    Margin,
    Border,
    Other,
}

impl PropertyCategory {
    /// Infers a category from substrings in the property name (last matching pattern wins).
    pub fn try_match(raw: &str) -> Self {
        let mut category = PropertyCategory::Other;

        const PATTERNS: [(&str, PropertyCategory); 5] = [
            ("background", PropertyCategory::Background),
            ("foreground", PropertyCategory::Foreground),
            ("padding", PropertyCategory::Padding),
            ("margin", PropertyCategory::Margin),
            ("border", PropertyCategory::Border),
        ];

        for (pattern, cat) in PATTERNS {
            if raw.contains(pattern) {
                category = cat.clone();
            }
        }

        category
    }
}

impl Property {
    /// Creates a property with an explicit category.
    pub fn new(raw: String, category: PropertyCategory) -> Self {
        Self { raw, category }
    }

    /// Parses a property name and assigns a category via [`PropertyCategory::try_match`].
    pub fn from_raw(raw: &str) -> Self {
        Self {
            raw: raw.trim().to_string(),
            category: PropertyCategory::try_match(raw),
        }
    }

    /// The property name as it appears in CSS.
    pub fn get_raw(&self) -> &String {
        &self.raw
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.raw)
    }
}
