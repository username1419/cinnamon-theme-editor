use super::{property::Property, property_value::Value};

#[derive(Clone, Debug)]
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
}

impl ToString for Declaration {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}
