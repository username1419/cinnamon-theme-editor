use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_config::style_input::StyleInput;
use crate::config::AppConfiguration;

#[component]
pub fn PropertyEditor() -> Element {
    let config = use_context::<AppConfiguration>();
    let num_selected = config.num_element_selected;

    rsx! {
        div { class: "property-editor",
            if num_selected() > 0 {
                StyleInput {  }
            } else {
                span { "Select an element to begin" }
            }
        }
    }
}
