pub(crate) mod color;
pub mod color_picker;
pub mod editor_section;
pub mod property_conf_utils;
pub mod style_input;

use crate::app::components::contents::property_editor::style_input::StyleInput;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::Value;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use tokio::task::spawn_blocking;

use crate::app::components::contents::property_editor::color::HSLColor;
use crate::app::components::contents::property_editor::color_picker::ColorPicker;
use crate::app::components::contents::property_editor::property_conf_utils::find_element_attribute;
use crate::config::{AppConfiguration, PropertyConfiguration};

#[component]
pub fn PropertyEditor() -> Element {
    let config = use_context::<AppConfiguration>();
    let num_selected = config.num_element_selected;
    let selected = config.selected_elements;
    let mut editing_style = config.element_style;

    let mut change = use_signal(|| false);
    let mut current_col = use_signal(HSLColor::default);
    use_effect(move || {
        let _ = selected.read();
        let conf = consume_context::<PropertyConfiguration>();

        let mut color_switch = conf.color_switch;
        let mut history = conf.color_history;
        let mut writelock = history.write();
        let index = writelock.len() - 1;
        writelock[index] = *current_col.peek();
        writelock.rotate_right(1);
        color_switch.set(true);
        debug!(
            "Added color {} to history",
            current_col.peek().as_css_property()
        );

        let notify = config.elements_notify.peek().clone();
        let confirm_notify = config.elements_notify_updated.peek().clone();
        spawn_blocking(move || async move {
            notify.notified().await;
            confirm_notify.notified().await;
        });

        if !*change.peek() {
            debug!("bgcolor change skipped");
            *change.write() = true;
            return;
        }
        debug!("finding bgcolor for elements: {:?}", selected.peek());
        let mut set = false;
        let attr = find_element_attribute("background-color");
        if let Some(attr) = attr.first() {
            let col_str = attr.to_string();
            if let Some(hsl_col) = dbg!(HSLColor::from_css_property(col_str)) {
                debug!("found color {:?}", hsl_col);
                *current_col.write() = hsl_col;
                editing_style.write().set_style_attribute(
                    Property::from_raw("background-color"),
                    Value::from_raw(&hsl_col.as_css_property()),
                );
                set = true;
            }
        }

        if !set {
            debug!("color not found, falling back to default");
            *current_col.write() = HSLColor::default();
            editing_style.write().clear();
        }
    });

    rsx! {
        div { class: "property-editor",
            if num_selected() > 0 {
                ColorPicker {
                    color: current_col,
                    on_color_change: move |col: HSLColor| {
                        let mut wl = editing_style.write();
                        wl.append(DeclarationBlock::from_raw(format!("background-color: {}", col.as_css_property())));
                        drop(wl);
                    }
                }
                StyleInput {  }
            } else {
                span { "Select an element to begin" }
            }
        }
    }
}
