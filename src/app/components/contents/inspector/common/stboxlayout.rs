use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::config::AppConfiguration;

#[derive(Clone, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[component]
pub fn StBoxLayout(children: Element, orientation: Orientation) -> Element {
    let config = use_context::<AppConfiguration>();

    rsx! {
        div { class: "stboxlayout", {children} }
    }
}
