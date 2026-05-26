use dioxus::prelude::*;

use crate::{
    app::io::parser::{basic_selector::BasicSelector, property_value::Value, selector::Selector},
    config::AppConfiguration,
};

/// Finds the value of the specified attribute belonging to the currently selected element. Starts
/// by retrieving a selector from `config.selected_elements`, then searching the
pub fn find_element_attribute(attribute: &str) -> Vec<Value> {
    let config = consume_context::<AppConfiguration>();
    let selected_elements = &*config.selected_elements.peek();
    let selected = selected_elements.iter().next();
    if selected_elements.len() > 1 {
        // TODO: yea you heard 'im
        warn!("hey this isnt implemented yet");
        return Vec::new();
    } else if let Some(element) = selected {
        let element_name = element
            .get_last()
            .expect("Selector is not supposed to be empty.")
            .0;
        debug!("searching for selector {}", element_name.get_raw());
        if let Some(from_editing) = search_editing_style(element, attribute) {
            return from_editing;
        } else if let Some(from_default) = search_default_style(element_name, attribute) {
            return from_default;
        }
    } else {
        warn!("there are no currently selected elements");
        return Vec::default();
    }

    Vec::new()
}

fn search_default_style(element_name: &BasicSelector, attribute: &str) -> Option<Vec<Value>> {
    let config = consume_context::<AppConfiguration>();
    let default_style = config.default_style;
    let default_rules = &*default_style.peek();
    let default_category_rules = default_rules.get(&*config.inspector_type.peek()).unwrap();

    let style = default_category_rules.get_declaration(&Selector::from_raw(element_name.get_raw()));
    if let Some(block) = style {
        debug!("found element style {:?}", block);
        if let Some(declaration) = block.find_attribute(attribute) {
            debug!(
                "attribute value found for {}: {:?}",
                attribute,
                declaration.get_value()
            );
            Some(declaration.get_value().to_vec())
        } else {
            None
        }
    } else {
        None
    }
}

fn search_editing_style(selector: &Selector, attribute: &str) -> Option<Vec<Value>> {
    let config = consume_context::<AppConfiguration>();
    let current_rules = &*config.editing_stylesheet.peek();
    let current_category_rules = current_rules.get(&*config.inspector_type.peek()).unwrap();

    debug!("searching editing rules for {}", selector.to_string());
    let style = current_category_rules.get_declaration(selector);
    if let Some(block) = style {
        debug!("found element style {:?}", block);
        if let Some(declaration) = block.find_attribute(attribute) {
            debug!(
                "attribute value found for {}: {:?}",
                attribute,
                declaration.get_value()
            );
            return Some(declaration.get_value().to_vec());
        }
    }

    let element_name = selector
        .get_last()
        .expect("Selector is not supposed to be empty.")
        .0;
    debug!("searching editing rules for {}", element_name.to_string());
    let style = current_category_rules.get_declaration(&Selector::from_raw(element_name.get_raw()));
    if let Some(block) = style {
        debug!("found element style {:?}", block);
        if let Some(declaration) = block.find_attribute(attribute) {
            debug!(
                "attribute value found for {}: {:?}",
                attribute,
                declaration.get_value()
            );
            return Some(declaration.get_value().to_vec());
        }
    }

    None
}
