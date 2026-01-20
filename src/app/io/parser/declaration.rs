use super::{property::Property, property_value::Value};

#[derive(Clone, Debug)]
pub struct Declaration {
    pub raw: String,
    pub property: Property,
    pub value: Vec<Value>,
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
        let raw = format!("{:?}:{:?}", property, value);
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
