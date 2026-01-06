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
