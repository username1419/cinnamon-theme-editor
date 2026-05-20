use crate::app::components::contents::property_config::style_input::StyleInput;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use tokio::task::spawn_blocking;

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
    let mut current_col = use_signal(HSLColor::default);
    use_effect(move || {
        let _ = selected.read();
        let conf = consume_context::<AppConfiguration>();

        let mut color_switch = conf.color_switch;
        let mut history = conf.color_history;
        if color_switch() {
            let mut writelock = history.write();
            let index = writelock.len() - 1;
            writelock[index] = *current_col.peek();
            writelock.rotate_right(1);
            *color_switch.write() = false;
            debug!(
                "Added color {} to history",
                current_col.peek().as_css_property()
            );
        }

        let notify = conf.elements_notify.peek().clone();
        let confirm_notify = conf.elements_notify_confirm.peek().clone();
        spawn_blocking(move || async move {
            notify.notified().await;
            if let Some(confirm) = confirm_notify.clone()
                && let Some(confirm) = confirm.upgrade()
            {
                confirm.notified().await;
            }
        });

        if !*change.peek() {
            debug!("bgcolor change skipped");
            *change.write() = true;
            return;
        }
        *change.write() = false;
        debug!("finding bgcolor for elements: {:?}", selected.peek());
        let mut set = false;
        let attr = find_element_attribute("background-color");
        if let Some(attr) = attr.first() {
            let col_str = attr.to_string();
            if let Some(hsl_col) = dbg!(HSLColor::from_css_property(col_str)) {
                debug!("found color {:?}", hsl_col);
                *current_col.write() = hsl_col;
                set = true;
            }
        }

        if !set {
            debug!("color not found, falling back to default");
            *current_col.write() = HSLColor::default();
        }
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
