use core::panic;
use dioxus_desktop::{LogicalSize, WindowBuilder};
use simple_logger::SimpleLogger;
pub mod app;
pub mod helper;
use crate::app::components::{
    contents::{main_contents::MainContent, toolbar::Toolbar},
    decorations::titlebar::Titlebar,
};
use crate::app::io::parse::StyleSheet;
use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const STYLE_COLORS: Asset = asset!("/assets/styling/colors.scss");
const MAIN_STYLE: Asset = asset!("/assets/styling/main.scss");
const TITLEBAR_STYLE: Asset = asset!("/assets/styling/titlebar.scss");
const TOOLBAR_STYLE: Asset = asset!("/assets/styling/toolbar.scss");
const OVERLAY_STYLE: Asset = asset!("/assets/styling/overlay.scss");
fn main() {
    if cfg!(windows) {
        panic!("Unsupported on Windows");
    }
    match std::env::var("DESKTOP_SESSION") {
        Ok(value) => {
            if value != "cinnamon" {
                panic!(
                    "This program is only compatible with the Cinnamon Desktop Environment. Current desktop environment: {}.",
                    value
                );
            }
        }
        Err(error) => panic!("{}", error),
    }
    let args = std::env::args().skip(1);
    let mut unknown_args = Vec::new();
    let mut is_logger_init = false;
    for arg in args {
        match arg.as_str() {
            "--trace" => {
                println!("Starting log at trace level");
                SimpleLogger::new().init().unwrap();
                is_logger_init = true;
            }
            _ => {
                unknown_args.push(arg);
            }
        }
    }

    if !is_logger_init {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    }
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus_desktop::Config::new().with_window(
                WindowBuilder::new()
                    // TODO: disable window rounding upon maximize
                    .with_decorations(false)
                    .with_inner_size(LogicalSize::new(1250, 750))
                    // NOTE: resizing isnt possible currently in dioxus 7.2.0
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
    let stylesheet = Signal::new(StyleSheet::default());
    use_context_provider(|| stylesheet);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_STYLE }
        document::Link { rel: "stylesheet", href: TITLEBAR_STYLE }
        document::Link { rel: "stylesheet", href: TOOLBAR_STYLE }
        document::Link { rel: "stylesheet", href: OVERLAY_STYLE }

        div { class: "window",
            Titlebar {}
            Toolbar {}
            MainContent {}
        }
    }
}
