use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_config::color_picker::ColorPicker;
use crate::config::AppConfiguration;

#[component]
pub fn PropertyEditor(selected_element: Signal<Option<MountedData>>) -> Element {
    let config = use_context::<AppConfiguration>();
    rsx! {
        div { class: "property-editor",
            if selected_element.read().is_some() {
                ColorPicker {}
            } else {
                span { "Select an element to begin" }
            }
        }
    }
}
