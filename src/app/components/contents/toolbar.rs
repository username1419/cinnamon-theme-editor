use std::time::Duration;

use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::toolbar_menu::file_menu::FileMenu;

#[component]
pub fn Toolbar() -> Element {
    let mouse_exit_timeout = use_hook(|| Duration::from_millis(100));

    rsx! {
        div { class: "toolbar",
            FileMenu { mouse_exit_timeout }
        }
    }
}
