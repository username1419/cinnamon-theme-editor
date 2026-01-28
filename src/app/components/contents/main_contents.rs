use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::inspector::Inspector;
use crate::app::components::contents::sidebar::Sidebar;

#[component]
pub fn MainContent() -> Element {
    rsx! {
        div { class: "content",
            Sidebar {}
            Inspector {}
        }
    }
}
