use dioxus::prelude::{debug, dioxus_stores, error};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf, str::FromStr};

use dioxus::stores::Store;
use once_cell::sync::Lazy;
use regex::Regex;

use super::parser::{declaration_block::DeclarationBlock, selector::*};

/// Represents a CSS stylesheet.
#[derive(Debug, Store)]
pub struct StyleSheet {
    /// The stylesheet's source file path
    source: PathBuf,
    /// Imported stylesheet
    import: Option<PathBuf>,
    /// The rulesets making up the stylesheet
    rulesets: HashMap<Selector, DeclarationBlock>,
}

static COMMENT_FILTER_REGEX: Lazy<Regex> = Lazy::new(|| {
    let pattern = r"/\*[^*]*\*+([^/*][^*]*\*+)*/";
    Regex::new(pattern).expect("Failed to compile regex")
});

impl Default for StyleSheet {
    fn default() -> Self {
        StyleSheet {
            source: PathBuf::default(),
            import: None,
            rulesets: HashMap::default(),
        }
    }
}

impl StyleSheet {
    pub fn get_source(&self) -> &PathBuf {
        &self.source
    }

    pub fn get_fallback_source(&self) -> Option<&PathBuf> {
        self.import.as_ref()
    }

    pub fn get_declaration(&self, selector: Selector) -> Option<&DeclarationBlock> {
        self.rulesets.get(&selector)
    }

    pub fn get_ruleset(&self, selector: Selector) -> Option<(&Selector, &DeclarationBlock)> {
        self.rulesets.get_key_value(&selector)
    }

    // TODO: using a token parser would be faster but i cant be fucked rn
    pub fn parse(source: PathBuf, raw: String) -> Self {
        // import statement
        let mut import = None;
        let mut raw = raw;
        if raw.starts_with("@import") {
            let import_str = raw
                .chars()
                .take_while(|c| ';'.ne(c))
                .skip_while(|c| '\"'.ne(c))
                .skip(1)
                .take_while(|c| '\"'.ne(c))
                .collect::<String>();
            import = Some(
                PathBuf::from_str(import_str.as_str())
                    .expect(format!("Failed convert Path from String \"{}\"", import_str).as_str()),
            );
            raw = raw
                .chars()
                .skip_while(|c| ';'.ne(c))
                .skip(1)
                .collect::<String>();
        }

        // remove all comments
        let raw = COMMENT_FILTER_REGEX.replace_all(&raw, "");

        let mut rulesets_iter = raw.split('}');
        let mut rulesets: HashMap<Selector, DeclarationBlock> = HashMap::new();

        while let Some(ruleset) = rulesets_iter.next().map(|r| r.split('{')).as_mut() {
            // TODO: make actually good error logs
            let selector_group = ruleset.next().expect("Invalid ruleset").trim().to_string();
            if selector_group.is_empty() {
                break;
            }
            let block_binding = ruleset
                .next()
                .expect("Invalid declaration block")
                .to_string();

            let declaration_block = Self::parse_block(block_binding);

            Self::parse_selector(selector_group)
                .into_iter()
                .for_each(|selector| {
                    if let Some(d) = rulesets.get_mut(&selector) {
                        d.append(declaration_block.clone());
                        return;
                    }
                    rulesets.insert(selector, declaration_block.clone());
                });
        }

        Self {
            source,
            import,
            rulesets,
        }
    }

    fn parse_block(raw: String) -> DeclarationBlock {
        DeclarationBlock::from_raw(raw)
    }

    fn parse_selector(raw: String) -> Vec<Selector> {
        let selector_list = raw.split(',');

        selector_list
            .filter_map(|selector| {
                let selector = selector.trim();

                if !selector.is_empty() {
                    Some(Selector::from_raw(selector))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn to_webview_safe(self) -> (StyleSheet, Option<StyleSheet>) {
        let mut rulesets = HashMap::new();
        for (selector, declaration_block) in self.rulesets {
            rulesets.insert(selector.to_webview_safe(), declaration_block);
        }
        let mut import = None;
        if let Some(mut import_path) = self.import {
            import_path.push("cinnamon");
            import_path.push("cinnamon.css");
            let result = read_to_string(&import_path)
                .inspect_err(|e| error!("{}", e))
                .ok();
            debug!("Read content from {:?}: {:?}", import_path, result);
            import = result.map(|import| {
                let mut style = StyleSheet::parse(import_path, import);

                style.rulesets = style
                    .rulesets
                    .into_iter()
                    .map(|(selector, mut declaration_block)| {
                        (
                            selector.to_webview_safe(),
                            declaration_block.to_webview_safe(),
                        )
                    })
                    .collect();

                style
            });
            debug!("Converted import style: {:?}", import);
        }

        (
            StyleSheet {
                source: self.source,
                import: None,
                rulesets,
            },
            import,
        )
    }

    pub fn to_string_categories(&self) -> HashMap<SelectorCategory, String> {
        let categories = self.rulesets.iter().fold(
            HashMap::from_iter(
                SelectorCategory::VALUES
                    .clone()
                    .into_iter()
                    .zip(SelectorCategory::VALUES.iter().map(|_| String::new())),
            ),
            |mut cat, (selector, declaration_block)| {
                let val = cat.get_mut(selector.category());
                if let Some(val) = val {
                    val.push_str(&*format!(
                        "{}{{{}}}",
                        selector.to_string(),
                        declaration_block.to_string()
                    ));
                }
                cat
            },
        );

        categories
    }

    pub fn append_rule(&mut self, selector: Selector, declaration_block: DeclarationBlock) {
        let val = self.rulesets.get_mut(&selector);
        if let Some(val) = val {
            val.append(declaration_block);
        } else {
            self.rulesets.insert(selector, declaration_block);
        }
    }
}

impl ToString for StyleSheet {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for (selector, declaration_block) in self.rulesets.iter() {
            out.push_str(&*format!(
                "{}{{{}}}",
                selector.to_string(),
                declaration_block.to_string()
            ));
        }

        out
    }
}
