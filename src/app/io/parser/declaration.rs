use std::fmt::Display;

use super::{property::Property, property_value::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    raw: String,
    property: Property,
    value: Vec<Value>,
}

impl Declaration {
    pub fn from_raw(raw: &str) -> Self {
        let (property, value) = raw.split_once(':').expect("Invalid declaration");
        let raw = raw.trim().to_string();
        let property = Property::from_raw(property);
        let value = Value::from_raw(value);

        Self {
            raw,
            property,
            value,
        }
    }

    pub fn new(property: Property, value: Vec<Value>) -> Self {
        let raw = format!(
            "{}:{}",
            property,
            value
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        Self {
            raw,
            property,
            value,
        }
    }

    pub fn get_value(&self) -> &[Value] {
        self.value.as_slice()
    }

    pub fn get_property(&self) -> &Property {
        &self.property
    }

    pub fn add_value(&mut self, value: Value) {
        self.value.push(value);
        self.update_raw();
    }

    pub fn remove_value(&mut self, idx: usize) {
        self.value.remove(idx);
        self.update_raw();
    }

    pub fn set_value(&mut self, values: Vec<Value>) {
        self.value = values;
        self.update_raw();
    }

    fn update_raw(&mut self) {
        self.raw = self.to_string();
    }

    pub fn decompose(self) -> (String, Property, Vec<Value>) {
        (self.raw, self.property, self.value)
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}",
            self.property,
            self.value
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ))
    }
}
