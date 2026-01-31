use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::inspector::Inspector;
use crate::app::components::contents::sidebar::Sidebar;
use crate::config::AppConfiguration;

#[component]
pub fn MainContent() -> Element {
    let config = use_context::<AppConfiguration>();
    let is_editing = config.is_editing;

    rsx! {
        div { class: "content",
            if is_editing() {
                Sidebar {}
                Inspector {}
            } else {
                span { "Create or open a theme to edit" }
            }
        }
    }
}
