use std::time::Duration;

use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::tao::keyboard::ModifiersState;
use tokio::time::sleep;

use crate::app::components::contents::toolbar_menu::menu_button::{MenuButton, Shortcut};

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
        div {
            id: "toolbar-file-menu",
            class: "toolbar-menu",
            style: if !(*toolbar_file_active.read() || *toolbar_file_menu_hover.read()) { "display: none" },
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
                id: "create-new-button",
                onclick: move |_| {
                    log::info!("create-new-button triggered");
                },
                shortcut: Shortcut::new(KeyCode::N, ModifiersState::CONTROL),
                text: "Create new theme",
            }
            MenuButton {
                id: "open-theme-button",
                onclick: move |_| {
                    log::info!("open-theme-button triggered");
                },
                shortcut: Shortcut::new(KeyCode::O, ModifiersState::CONTROL),
                text: "Open theme",
            }
            MenuButton {
                id: "placeholder-button1",
                onclick: move |_| {
                    log::info!("placeholder-button1 triggered");
                },
                text: "Placeholder 1",
            }
            MenuButton {
                id: "placeholder-button2",
                onclick: move |_| {
                    log::info!("placeholder-button2 triggered");
                },
                text: "Placeholder 2",
            }
            MenuButton {
                id: "export-theme-button",
                onclick: move |_| {
                    log::info!("export-theme-button triggered");
                },
                shortcut: Shortcut::new(KeyCode::E, ModifiersState::CONTROL),
                text: "Export theme",
            }
        }
    }
}
