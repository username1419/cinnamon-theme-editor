use dioxus::html::geometry::Coordinates;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use dioxus_desktop::tao::keyboard::ModifiersState;
use dioxus_desktop::{HotKeyState, use_global_shortcut};
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct Shortcut {
    key: KeyCode,
    modifiers: ModifiersState,
}

impl Shortcut {
    // NOTE: builder pattern would be more readable but idk
    pub fn new(key: KeyCode, modifiers: ModifiersState) -> Self {
        Shortcut { key, modifiers }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuButtonProps {
    id: String,
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    shortcut: Option<Shortcut>,
    #[props(optional)]
    text: Option<String>,
}

#[component]
pub fn MenuButton(props: MenuButtonProps) -> Element {
    if let Some(shortcut) = props.shortcut.as_ref() {
        use_global_shortcut((shortcut.modifiers, shortcut.key), move |state| {
            if state == HotKeyState::Released {
                return;
            }
            // tower of doom? more like mountain of doom LMAO
            // NOTE: actually in hindsight it would be better if we get the cursor
            // position instead
            props.onclick.call(MouseEvent::new(
                Rc::new(
                    SerializedMouseData::new(
                        None,
                        Default::default(),
                        Coordinates::new(
                            Point2D::default(),
                            Point2D::default(),
                            Point2D::default(),
                            Point2D::default(),
                        ),
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
            id: props.id,
            onclick: move |event| { props.onclick.call(event) },
            "{text}"
            span { class: "shortcut-text", "{shortcut_label}" }
        }
    }
}
