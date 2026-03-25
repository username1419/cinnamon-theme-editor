use std::str::FromStr;

use dioxus::{html::meta::property, prelude::debug};

use crate::app::io::parser::{property::Property, property_value::Value};

use super::declaration::Declaration;

#[derive(Clone, Debug, Default, PartialEq)]
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
    pub fn set_style_attribute(&mut self, attribute: Property, values: Vec<Value>) {
        if let Some(declaration) = self
            .declarations
            .iter_mut()
            .find(|d| attribute.eq(&d.property))
        {
            declaration.value = values;
        } else {
            let declarations = &mut self.declarations;
            declarations.push(Declaration::new(attribute, values));
        }
    }

    /// this gets the original data, and WILL NOT reflect changes if this struct is modified using
    /// DeclarationBlock::set_style_attribute()
    pub fn get_raw(&self) -> &String {
        &self.raw
    }

    pub fn append(&mut self, declaration_block: DeclarationBlock) {
        for declaration in declaration_block.declarations {
            self.set_style_attribute(declaration.property, declaration.value);
        }
    }

    pub fn find_attribute(&self, name: &str) -> Option<&Declaration> {
        self.declarations
            .iter()
            .find(|d| name.eq(d.property.get_raw()))
    }

    pub fn findmut_attribute(&mut self, name: String) -> Option<&mut Declaration> {
        self.declarations
            .iter_mut()
            .find(|d| name.eq(d.property.get_raw()))
    }
}

impl ToString for DeclarationBlock {
    fn to_string(&self) -> String {
        self.declarations
            .iter()
            .map(|d| format!("{};", d.to_string()))
            .collect()
    }
}
