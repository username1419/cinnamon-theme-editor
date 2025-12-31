use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::io::parser::selector::SelectorCategory;
use crate::config::AppConfiguration;

#[component]
pub fn Sidebar() -> Element {
    let config = use_context::<AppConfiguration>();
    let stylesheet = config.stylesheet;
    rsx! {
        div { class: "sidebar",
            if *config.is_editing.read() {
                for (index , category) in SelectorCategory::VALUES.iter().enumerate() {
                    div { id: "sidebar-category-{index}",
                        span { "{category:?}" }
                    }
                }
            }
        }
    }
}
