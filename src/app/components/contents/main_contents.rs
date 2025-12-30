use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_config::color_picker::ColorPicker;
use crate::app::components::contents::sidebar::Sidebar;

#[component]
pub fn MainContent() -> Element {
    rsx! {
        div { class: "content",
            Sidebar {}
            ColorPicker {}
        }
    }
}
