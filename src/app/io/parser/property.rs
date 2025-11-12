#[derive(Clone, Debug)]
pub struct Property {
    raw: String,
    category: PropertyCategory,
}

#[derive(Clone, Debug)]
pub enum PropertyCategory {
    Background,
    Foreground,
    Padding,
    Margin,
    Border,
    Other,
}

impl PropertyCategory {
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
    pub fn new(raw: String, category: PropertyCategory) -> Self {
        Self { raw, category }
    }

    pub fn from_raw(raw: &str) -> Self {
        Self {
            raw: raw.trim().to_string(),
            category: PropertyCategory::try_match(raw),
        }
    }
}
