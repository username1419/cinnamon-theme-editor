use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::panel::Panel;
use crate::app::components::contents::inspector::property_editor::PropertyEditor;
use crate::config::AppConfiguration;

#[component]
pub fn Inspector() -> Element {
    let config = use_context::<AppConfiguration>();
    let default_style = config.default_style;
    let editing_stylesheet = config.editing_stylesheet;
    let selected_element = config.selected_element;
    rsx! {
        div { class: "inspector",
            style { "{default_style.read()}" }
            style { "{editing_stylesheet.read().to_string()}" }
            // NOTE: inspector contents should autohide when not selected
            Panel {}
        }
        PropertyEditor {}
    }
}
