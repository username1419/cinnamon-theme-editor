use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::tao::keyboard::ModifiersState;
use dioxus_desktop::{HotKeyState, use_global_shortcut, use_window};
use std::rc::Rc;

use crate::helper::Helper;

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
    let shortcut = props.shortcut;

    rsx! {
        if let Some(shortcut) = shortcut {
            ShortcutHandler { shortcut: shortcut, onclick: props.onclick }
        }
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

#[component]
fn ShortcutHandler(shortcut: Shortcut, onclick: EventHandler<MouseEvent>) -> Element {
    let window = use_window();
    use_global_shortcut((shortcut.modifiers, shortcut.key), move |state| {
        if state == HotKeyState::Released {
            return;
        }
        // tower of doom? more like mountain of doom LMAO
        onclick.call(MouseEvent::new(
            Rc::new(
                SerializedMouseData::new(
                    None,
                    Default::default(),
                    Helper::to_coord(window.cursor_position().unwrap_or_default()),
                    Modifiers::default(),
                )
                .into(),
            ),
            false,
        ));
    })
    .is_err()
    .then(|| error!("Failed to initialize shortcut {:?}", shortcut));

    rsx! {}
}
