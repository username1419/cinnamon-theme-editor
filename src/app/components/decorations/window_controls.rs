use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::use_window;

#[component]
pub fn WindowControls(wcontrol_hover: Signal<bool>) -> Element {
    rsx! {
        div { class: "window-controls",
            button {
                class: "wcontrol-minimize",
                onclick: move |_| {
                    use_window().set_minimized(true);
                },
                onmouseover: move |_| {
                    wcontrol_hover.set(true);
                },
                onmouseleave: move |_| {
                    wcontrol_hover.set(false);
                },
            }
            button {
                class: "wcontrol-maxmize",
                onclick: move |_| {
                    let w = use_window();
                    w.set_maximized(!w.is_maximized());
                },
                onmouseover: move |_| {
                    wcontrol_hover.set(true);
                },
                onmouseleave: move |_| {
                    wcontrol_hover.set(false);
                },
            }
            button {
                class: "wcontrol-exit",
                onclick: move |_| {
                    use_window().close();
                },
                onmouseover: move |_| {
                    wcontrol_hover.set(true);
                },
                onmouseleave: move |_| {
                    wcontrol_hover.set(false);
                },
            }
        }
    }
}
