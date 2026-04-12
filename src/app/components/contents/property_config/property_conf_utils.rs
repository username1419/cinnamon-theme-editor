use dioxus::prelude::*;

use crate::{
    app::io::parser::{property_value::Value, selector::Selector},
    config::AppConfiguration,
};

pub fn find_element_attribute(attribute: &str) -> Vec<Value> {
    let config = consume_context::<AppConfiguration>();
    let element_style = config.element_style;
    let editing_style = config.editing_stylesheet;
    let default_style = config.default_style;

    if let Some(declaration) = element_style.peek().find_attribute(attribute) {
        return declaration.value.clone();
    }
    let style = &*default_style.peek();
    let category_style = style.get(&*config.inspector_type.peek()).unwrap();
    let selected_elements = &*config.selected_elements.peek();
    let selected = selected_elements.iter().next();
    if selected_elements.len() > 1 {
        // TODO: yea you heard 'im
        warn!("hey this isnt implemented yet");
        return Vec::new();
    } else {
        if let Some(element) = selected {
            let element_name = element
                .get_last()
                .expect("Selector is not supposed to be empty.")
                .0;
            debug!("searching for selector {}", element_name.get_raw());
            let style =
                category_style.get_declaration(&Selector::from_raw(&element_name.get_raw()));
            if let Some(block) = style {
                debug!("found element style {:?}", block);
                if let Some(declaration) = block.find_attribute(attribute) {
                    debug!(
                        "attribute value found for {}: {:?}",
                        attribute, declaration.value
                    );
                    return declaration.value.clone();
                }
            }
        } else {
            warn!("there are no currently selected elements");
            return Vec::default();
        }
    }

    if let Some(selected) = selected {
        debug!("searching for selector {}", selected);
        if let Some(block) = editing_style.peek().get_declaration(selected) {
            if let Some(declaration) = block.find_attribute(attribute) {
                debug!(
                    "attribute value found for {}: {:?}",
                    attribute, declaration.value
                );
                return declaration.value.clone();
            }
        }
    }

    Vec::new()
}
