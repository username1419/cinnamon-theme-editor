use crate::app::io::parser::{property::Property, property_value::Value};

use super::declaration::Declaration;

#[derive(Clone, Debug)]
pub struct DeclarationBlock {
    raw: String,
    declarations: Vec<Declaration>,
}

impl DeclarationBlock {
    pub fn from_raw(raw: String) -> DeclarationBlock {
        let raw = raw.trim().to_string();
        let declarations = raw
            .split(';')
            .filter_map(|tracking| {
                let tracking = tracking.trim();
                if tracking.is_empty() {
                    return None;
                }
                Some(Declaration::from_raw(tracking))
            })
            .collect();

        DeclarationBlock { raw, declarations }
    }

    /// Sets a value to the provided style attribute. This function does not implement checking,
    /// therefore the caller must verify the validity of the provided values.
    fn set_style_attribute(&mut self, attribute: Property, values: &[Value]) {
        let mut found = false;
        for declaration in self.declarations.iter_mut() {
            if declaration.property != attribute {
                continue;
            }

            declaration.value = values.to_vec();
            found = true;
        }

        if !found {
            let declarations = &mut self.declarations;
            declarations.push(Declaration::new(attribute, values.to_vec()));
        }
    }
}

impl ToString for DeclarationBlock {
    fn to_string(&self) -> String {
        // i mean i can use self.raw but idk
        self.declarations
            .iter()
            .map(|d| format!("{};", d.to_string()))
            .collect()
    }
}
