use std::time::Duration;

use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use tokio::time::sleep;

use crate::app::components::contents::toolbar_menu::menu_button::{MenuButton, Shortcut};

fn on_create_new() {
    todo!("uhhhhhhh");
}

#[component]
pub fn FileMenu(mouse_exit_timeout: Duration) -> Element {
    // NOTE: im sure theres a way to use 1 signal to do this but im stupid
    let mut toolbar_file_active = use_signal(|| false);
    let mut toolbar_file_menu_hover = use_signal(|| false);

    rsx! {
        button {
            id: "toolbar-file",
            class: "toolbar-button",
            onclick: move |_| {
                let active = *toolbar_file_active.read();
                toolbar_file_active.set(!active);

            },
            onmouseleave: move |_| {
                let mut active = toolbar_file_active.clone();
                async move {
                    sleep(mouse_exit_timeout).await;
                    active.set(false);
                }
            },
            "File"
        }
        if *toolbar_file_active.read() || *toolbar_file_menu_hover.read() {
            div {
                id: "toolbar-file-menu",
                class: "toolbar-menu",
                onmouseover: move |_| {
                    toolbar_file_menu_hover.set(true);
                },
                onmouseleave: move |_| {
                    let mut active = toolbar_file_menu_hover.clone();
                    async move {
                        sleep(mouse_exit_timeout).await;
                        active.set(false);
                    }
                },
                MenuButton {
                    id: "crete-new-button",
                    onclick: move |_| {
                        on_create_new();
                    },
                    shortcut: Shortcut::new(Code::KeyN, true, false, false),
                    text: "Create new",
                }
            }
        }
    }
}
