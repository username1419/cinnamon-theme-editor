use dioxus::html::geometry::Coordinates;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::tao::keyboard::ModifiersState;
use dioxus_desktop::wry::dpi::PhysicalPosition;
use dioxus_desktop::{HotKeyState, use_global_shortcut, use_window};
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct Shortcut {
    key: KeyCode,
    modifiers: ModifiersState,
}

impl Shortcut {
    pub fn new(key: KeyCode, modifiers: ModifiersState) -> Self {
        Shortcut { key, modifiers }
    }
}

fn to_coord(pos: PhysicalPosition<f64>) -> Coordinates {
    let screen = Point2D::new(pos.x, pos.y);
    let client = Point2D::new(pos.x, pos.y);
    let element = Point2D::new(pos.x, pos.y);
    let page = Point2D::new(pos.x, pos.y);

    Coordinates::new(screen, client, element, page)
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuButtonProps {
    id: String,
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    tooltip: Option<String>,
    #[props(optional)]
    shortcut: Option<Shortcut>,
    #[props(optional)]
    text: Option<String>,
}

#[component]
pub fn MenuButton(props: MenuButtonProps) -> Element {
    let window = use_window();
    if let Some(shortcut) = props.shortcut.as_ref() {
        use_global_shortcut((shortcut.modifiers, shortcut.key), move |state| {
            if state == HotKeyState::Released {
                return;
            }
            // tower of doom? more like mountain of doom LMAO
            props.onclick.call(MouseEvent::new(
                Rc::new(
                    SerializedMouseData::new(
                        None,
                        Default::default(),
                        to_coord(window.cursor_position().unwrap_or_default()),
                        Modifiers::default(),
                    )
                    .into(),
                ),
                false,
            ));
        })
        .is_err()
        .then(|| log::error!("Failed to initialize shortcut {:?}", shortcut));
    }
    let text = use_hook(|| props.text.unwrap_or_default());
    let shortcut_label = use_hook(|| {
        props
            .shortcut
            .clone()
            .map(|shortcut| {
                let mut label = String::new();
                label.push_str(format!("{:?}", shortcut.key).as_str());
                if shortcut.modifiers.shift_key() {
                    label = format!("Shift+{}", label);
                }
                if shortcut.modifiers.alt_key() {
                    label = format!("Alt+{}", label);
                }
                if shortcut.modifiers.control_key() {
                    label = format!("Ctrl+{}", label);
                }
                label
            })
            .unwrap_or_default()
    });

    rsx! {
        button {
            class: "toolbar-menu-button",
            title: props.tooltip,
            id: props.id,
            onclick: move |event| { props.onclick.call(event) },
            "{text}"
            span { class: "shortcut-text", "{shortcut_label}" }
        }
    }
}
