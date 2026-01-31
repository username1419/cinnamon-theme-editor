use std::time::Duration;

use dioxus::core::Element;
use dioxus::prelude::*;
use dioxus_desktop::tao::keyboard::ModifiersState;
use rfd::FileDialog;
use tokio::time::sleep;

use crate::app::components::contents::toolbar_menu::menu_button::{MenuButton, Shortcut};
use crate::app::io::read::{self, is_theme_exist};
use crate::config::AppConfiguration;

#[component]
pub fn FileMenu(mouse_exit_timeout: Duration) -> Element {
    // NOTE: im sure theres a way to use 1 signal to do this but im stupid
    let mut toolbar_file_active = use_signal(|| false);
    let mut toolbar_file_menu_hover = use_signal(|| false);

    let mut choose_theme_name_overlay_active = use_signal(|| false);
    let mut input = use_signal(|| String::default());
    let mut config = use_context::<AppConfiguration>();

    let mut create_new_theme = move |name| {
        let theme_exists = is_theme_exist(&name);
        if theme_exists.is_err() || theme_exists.as_ref().is_ok_and(|e| e.eq(&true)) {
            if theme_exists.is_err() {
                error!("{}", theme_exists.unwrap_err());
                return;
            }
            error!("Theme already exists");
        }
        // NOTE: picking the default theme comes later because users might be
        // confused with opening a theme
        // though idk the actual effectiveness of this cuz the first couple of times i
        // still got confused
        let folder = FileDialog::new()
            .set_title("Choose a default theme")
            .set_directory("/usr/share/themes/")
            // BUG: original window will be flagged as inactive by cinnamon, idk how
            // to fix it
            .pick_folder();
        if folder.is_none() {
            return;
        }
        let folder = folder.unwrap();
        info!("Picked default theme {:?}", folder);
        let style = read::create_as_edit(name, folder);
        match style {
            // close the overlay
            Ok(stylesheet) => {
                // BUG: dioxus force trims style tags to a certain
                // character limit so it literally doesnt work
                let (editing, default) = stylesheet.to_webview_safe();
                config
                    .default_style
                    .set(default.unwrap_or_default().to_string_categories());
                debug!("{:#?}", config.default_style);
                config.editing_stylesheet.set(editing);
                config.is_editing.set(true);
            }
            Err(err) => {
                error!("Error encountered on read: {}", err);
            }
        }
    };

    let mut open_existing_theme = move || {
        let folder = FileDialog::new()
            .set_title("Choose a theme")
            .set_directory("/usr/share/themes/")
            // BUG: original window will be flagged as inactive by cinnamon, idk how
            // to fix it
            .pick_folder();
        if folder.is_none() {
            return;
        }
        let folder = folder.unwrap();
        info!("Picked theme {:?}", folder);
        let style = read::open_existing(folder);
        match style {
            // close the overlay
            Ok(stylesheet) => {
                let (editing, default) = stylesheet.to_webview_safe();
                config
                    .default_style
                    .set(default.unwrap_or_default().to_string_categories());
                debug!("{:#?}", config.default_style);
                config.editing_stylesheet.set(editing);
                config.is_editing.set(true);
            }
            Err(err) => {
                error!("Error encountered on read: {}", err);
            }
        }
    };

    rsx! {
        button {
            id: "toolbar-file",
            class: "toolbar-button",
            onclick: move |_| {
                let active = *toolbar_file_active.read();
                toolbar_file_active.set(!active);

            },
            onmouseenter: move |_| {
                if *toolbar_file_menu_hover.read() {
                    toolbar_file_active.set(true);
                }
            },
            onmouseleave: move |_| async move {
                sleep(mouse_exit_timeout).await;
                toolbar_file_active.set(false);
            },
            "File"
        }
        div {
            id: "toolbar-file-menu",
            class: "toolbar-menu",
            style: if !(*toolbar_file_active.read() || *toolbar_file_menu_hover.read()) { "display: none" },
            onmouseenter: move |_| {
                toolbar_file_menu_hover.set(true);
            },
            onmouseleave: move |_| async move {
                sleep(mouse_exit_timeout).await;
                toolbar_file_menu_hover.set(false);
            },
            MenuButton {
                id: "create-new-button",
                onclick: move |_| {
                    info!("create-new-button triggered");
                    *choose_theme_name_overlay_active.write() = true;
                },
                shortcut: Shortcut::new(KeyCode::N, ModifiersState::CONTROL),
                text: "Create new theme",
                tooltip: "Placeholder tooltip",
            }
            MenuButton {
                id: "open-theme-button",
                onclick: move |_| async move {
                    info!("open-theme-button triggered");
                    open_existing_theme();
                },
                shortcut: Shortcut::new(KeyCode::O, ModifiersState::CONTROL),
                text: "Open theme",
            }
            MenuButton {
                id: "placeholder-button1",
                onclick: move |_| {
                    info!("placeholder-button1 triggered");
                    todo!();
                    ()
                },
                text: "Placeholder 1",
            }
            MenuButton {
                id: "placeholder-button2",
                onclick: move |_| {
                    info!("placeholder-button2 triggered");
                    todo!();
                    ()
                },
                text: "Placeholder 2",
            }
            MenuButton {
                id: "export-theme-button",
                onclick: move |_| {
                    info!("export-theme-button triggered");
                    todo!();
                    ()
                },
                shortcut: Shortcut::new(KeyCode::E, ModifiersState::CONTROL),
                text: "Export theme",
            }
        }

        if *choose_theme_name_overlay_active.read() {
            // HACK: keep rendering until explicitly stopped
            div {
                id: "choose-theme-name-overlay",
                class: "overlay focus-block",
                style: if !*choose_theme_name_overlay_active.read() { "display: none" },
                // NOTE: dialog isnt use bc modal implementation isnt available on desktop (to my
                // understanding)
                div {
                    id: "choose-theme-name-submenu",
                    class: "overlay-menu text-dialog",
                    span { "Enter theme name" }
                    form {
                        onsubmit: move |e| {
                            // TODO: perform input sanitization
                            info!(
                                "onsumbit action for overlay-menu-theme-name-submenu form with value \"{}\"",
                                input.read()
                            );
                            let name = input.read().cloned();
                            input.clear();
                            e.prevent_default();
                            async move {
                                create_new_theme(name);
                                *choose_theme_name_overlay_active.write() = false;
                            }
                        },
                        oncancel: move |_| {
                            // close the overlay
                            *choose_theme_name_overlay_active.write() = false;
                        },
                        input {
                            r#type: "text",
                            id: "chose-theme-name-input",
                            class: "text-input",
                            "placeholder": "Choose theme",
                            required: true,
                            minlength: 1,
                            onmounted: move |element| async move {
                                let _ = element.set_focus(true).await;
                            },
                            oninput: move |element| input.set(element.value()),
                        }
                        div { class: "text-dialog-choices",
                            button {
                                class: "confirm-choice suggested-action",
                                r#type: "submit",
                                "Choose"
                            }
                            button { class: "cancel-choice", r#type: "cancel", "Cancel" }
                        }
                    }
                }
            }
        }
    }
}
