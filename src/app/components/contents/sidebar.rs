use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div { class: "sidebar" }
    }
}
