use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::config::AppConfiguration;

#[component]
pub fn CinnamonGenericContainer(class: String, style: String, children: Element) -> Element {
    let config = use_context::<AppConfiguration>();
    rsx! {
        div { class: "CinnamonGenericContainer {class}", style: "{style}", {children} }
    }
}
