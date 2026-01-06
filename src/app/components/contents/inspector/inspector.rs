use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::property_editor::PropertyEditor;
use crate::config::AppConfiguration;

#[component]
pub fn Inspector() -> Element {
    let config = use_context::<AppConfiguration>();
    let mut selected_element = use_signal(|| None);
    rsx! {
        div { class: "inspector" }
        PropertyEditor { selected_element }
    }
}
