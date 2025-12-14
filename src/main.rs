use core::panic;
use dioxus_desktop::{WindowBuilder, use_window, window};
use simple_logger::SimpleLogger;
use std::thread::Scope;
pub mod app;
pub mod helper;
use dioxus::{core::LaunchConfig, html::g::cx, prelude::*};
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
fn main() {
    if cfg!(windows) {
        panic!("Unsupported on Windows");
    }
    match std::env::var("DESKTOP_SESSION") {
        Ok(value) => {
            if value != "cinnamon" {
                panic!(
                    "This program is only compatible with the Cinnamon Desktop Environment. Current desktop environment: {}.",
                    value,
                );
            }
        }
        Err(error) => panic!("{}", error),
    }
    let args = std::env::args().skip(1);
    let mut unknown_args = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--trace" => {
                println!("Starting log at trace level");
                SimpleLogger::new().init().unwrap();
            }
            _ => {
                unknown_args.push(arg);
            }
        }
    }
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus_desktop::Config::new().with_window(
                WindowBuilder::new()
                    .with_decorations(false)
                    .with_resizable(true)
                    .with_transparent(true),
            ),
        )
        .launch(App);
}
/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {

        class: "window",
            div { class: "titlebar", "data-tauri-drag-region": "true",
                div {
            class: "titlebar-left",
        }
        div {
        class: "titlebar-center",

                span { class: "title", "Cinnamon Theme Editor" }}
        div {
            class: "titlebar-right",
                div { class: "window-controls",
                    button {
                        class: "wcontrol-minimize",
                        onclick: move |_| {
                            use_window().set_minimized(true);
                        },
                    }
                    button {
                        class: "wcontrol-maxmize",
                        onclick: move |_| {
                            let w = use_window();
                            w.set_maximized(!w.is_maximized());
                        },
                    }
                    button {
                        class: "wcontrol-exit",
                        onclick: move |_| {
                            use_window().close();
                        },
                    }
                }
            }
        }}
    }
}
