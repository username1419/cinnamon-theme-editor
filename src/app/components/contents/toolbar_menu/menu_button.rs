use std::rc::Rc;
use std::str::FromStr;

use dioxus::html::geometry::Coordinates;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

#[derive(PartialEq, Clone)]
pub struct Shortcut {
    key: Code,
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl Shortcut {
    // NOTE: builder pattern would be more readable but idk
    pub fn new(key: Code, ctrl: bool, shift: bool, alt: bool) -> Self {
        Shortcut {
            key,
            ctrl,
            shift,
            alt,
        }
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
    let text = use_hook(|| props.text.unwrap_or_default());
    let shortcut_label = use_hook(|| {
        props
            .shortcut
            .clone()
            .map(|shortcut| {
                let mut label = String::new();
                label.push_str(shortcut.key.to_string().as_str());
                if shortcut.shift {
                    label = format!("Shift+{}", label);
                }
                if shortcut.alt {
                    label = format!("Alt+{}", label);
                }
                if shortcut.ctrl {
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
            onkeydown: move |event| {
                if let Some(shortcut) = props.shortcut.as_ref() {
                    let match_key = event.code() == shortcut.key;
                    let match_ctrl = event.modifiers().ctrl() == shortcut.ctrl;
                    let match_shift = event.modifiers().shift() == shortcut.shift;
                    let match_alt = event.modifiers().alt() == shortcut.alt;

                    // tower of doom? more like mountain of doom LMAO
                    // NOTE: actually in hindsight it would be better if we get the cursor
                    // position instead
                    if match_key && match_alt && match_shift && match_ctrl {
                        props
                            .onclick
                            .call(
                                MouseEvent::new(
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
                                ),
                            );
                    }
                }
            },
            "{text}"
            span { class: "shortcut-text", "{shortcut_label}" }
        }
    }
}
