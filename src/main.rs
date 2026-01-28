use core::panic;
use dioxus::html::input_data::{MouseButton, MouseButtonSet};
use dioxus::logger::tracing::Level;
use dioxus_desktop::wry::dpi::PhysicalPosition;
use dioxus_desktop::{LogicalSize, WindowBuilder};
pub mod app;
pub mod config;
pub mod helper;
use crate::app::io::parse::StyleSheet;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use crate::app::io::parser::selector::SelectorCategory;
use crate::config::MouseState;
use crate::helper::Helper;
use crate::{
    app::components::{
        contents::{main_contents::MainContent, toolbar::Toolbar},
        decorations::titlebar::Titlebar,
    },
    config::AppConfiguration,
};
use dioxus::{logger, prelude::*};
const FAVICON: Asset = asset!("/assets/favicon.ico");
const STYLE_COLORS: Asset = asset!("/assets/styling/colors.scss");
const MAIN_STYLE: Asset = asset!("/assets/styling/main.scss");
const TITLEBAR_STYLE: Asset = asset!("/assets/styling/titlebar.scss");
const TOOLBAR_STYLE: Asset = asset!("/assets/styling/toolbar.scss");
const OVERLAY_STYLE: Asset = asset!("/assets/styling/overlay.scss");
const COLOR_PICKER_STYLE: Asset = asset!("/assets/styling/color-picker.scss");

const INSPECTOR_PANEL_STYLE: Asset = asset!("/assets/styling/inspector/panel.scss");
#[cfg(debug_assertions)]
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
                logger::init(Level::TRACE).unwrap();
                is_logger_init = true;
            }
            "--debug" => {
                println!("Starting log at debug level");
                logger::init(Level::DEBUG).unwrap();
                is_logger_init = true;
            }
            _ => {
                unknown_args.push(arg);
            }
        }
    }

    if !is_logger_init {
        if cfg!(debug_assertions) {
            logger::init(Level::DEBUG).unwrap();
        }
        logger::initialize_default();
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
    let default_style = use_signal(|| String::new());
    let editing_stylesheet = use_store(|| StyleSheet::default());
    let mut mouse_state = use_signal(|| MouseState {
        coordinates: Helper::to_coord(PhysicalPosition::default()),
        mouse_down: MouseButtonSet::default(),
    });
    let is_editing = use_signal(|| false);
    let inspector_type = use_signal(|| SelectorCategory::default());
    let current_element = use_signal(|| 0);
    let element_selected = use_signal(|| 0);
    let element_style = use_signal(|| DeclarationBlock::default());

    use_context_provider(|| AppConfiguration {
        is_editing,
        default_style,
        editing_stylesheet,
        mouse_state,
        inspector_type,
        current_element,
        element_selected,
        element_style,
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_STYLE }
        document::Link { rel: "stylesheet", href: TITLEBAR_STYLE }
        document::Link { rel: "stylesheet", href: TOOLBAR_STYLE }
        document::Link { rel: "stylesheet", href: OVERLAY_STYLE }
        document::Link { rel: "stylesheet", href: COLOR_PICKER_STYLE }
        document::Link { rel: "stylesheet", href: INSPECTOR_PANEL_STYLE }

        div {
            class: "window",
            onmousemove: move |event| {
                let mut mouse_state = mouse_state.write();
                mouse_state.coordinates = event.coordinates();
                mouse_state.mouse_down = event.held_buttons();
            },
            onmouseenter: move |_| mouse_state.write().mouse_down = MouseButtonSet::default(),
            onmouseleave: move |_| mouse_state.write().mouse_down = MouseButtonSet::default(),
            onmousedown: move |event| {
                // this is a very unnecessarily fancy way to do this
                mouse_state.write().mouse_down
                    |= event.trigger_button().unwrap_or(MouseButton::Unknown);
            },
            onmouseup: move |event| {
                mouse_state
                    .write()
                    .mouse_down
                    .remove(event.trigger_button().unwrap_or(MouseButton::Unknown));
            },
            Titlebar {}
            Toolbar {}
            MainContent {}
        }
    }
}
