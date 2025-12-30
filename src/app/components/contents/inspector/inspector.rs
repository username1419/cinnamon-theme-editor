use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::config::AppConfiguration;

#[component]
pub fn Inspector() -> Element {
    let config = use_context::<AppConfiguration>();
    rsx! {
        div { class: "inspector" }
    }
}
