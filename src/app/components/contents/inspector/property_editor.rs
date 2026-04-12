use crate::app::components::contents::property_config::style_input::StyleInput;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_config::color_picker::{ColorPicker, HSLColor};
use crate::app::components::contents::property_config::property_conf_utils::find_element_attribute;
use crate::config::AppConfiguration;

#[component]
pub fn PropertyEditor() -> Element {
    let config = use_context::<AppConfiguration>();
    let num_selected = config.num_element_selected;
    let selected = config.selected_elements;
    let mut editing_style = config.element_style;

    let mut change = use_signal(|| true);
    let mut current_col = use_signal(|| {
        // NOTE: this is a bad idea
        debug!("init: finding bgcolor for elements: {:?}", selected.peek());
        let attr = find_element_attribute("background-color");
        if let Some(attr) = attr.get(0) {
            let col_str = attr.to_string();
            if let Some(hsl_col) = HSLColor::from_css_property(col_str) {
                debug!("init: found color {:?}", hsl_col);
                return hsl_col;
            }
        }

        debug!("init: color not found, falling back to default");
        return HSLColor::default();
    });
    use_effect(move || {
        let _ = selected.read();
        if !*change.peek() {
            debug!("bgcolor change skipped");
            *change.write() = true;
            return;
        }
        *change.write() = false;
        debug!("finding bgcolor for elements: {:?}", selected.peek());
        let attr = find_element_attribute("background-color");
        if let Some(attr) = attr.get(0) {
            let col_str = attr.to_string();
            if let Some(hsl_col) = HSLColor::from_css_property(col_str) {
                debug!("found color {:?}", hsl_col);
                *current_col.write() = hsl_col;
            }
        }

        debug!("color not found, falling back to default");
        *current_col.write() = HSLColor::default();
    });

    rsx! {
        div { class: "property-editor",
            if num_selected() > 0 {
                ColorPicker {
                    color: current_col,
                    on_color_change: move |col: HSLColor| {
                        editing_style.write().append(DeclarationBlock::from_raw(format!("background-color: {}", col.as_css_property())));
                    }
                }
                StyleInput {  }
            } else {
                span { "Select an element to begin" }
            }
        }
    }
}
