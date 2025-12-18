use dioxus::prelude::dioxus_signals;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::use_window;

use crate::app::components::decorations::window_controls::WindowControls;

#[component]
pub fn Titlebar() -> Element {
    let wcontrol_hover = use_signal(|| false);

    rsx! {
        div {
            class: "titlebar",
            onmousedown: move |_| {
                if *wcontrol_hover.read() {
                    return;
                }
                use_window().drag();
            },
            div { class: "titlebar-left" }
            div { class: "titlebar-center",
                span { class: "title", "Cinnamon Theme Editor" }
            }
            div { class: "titlebar-right",
                WindowControls { wcontrol_hover }
            }
        }
    }
}
