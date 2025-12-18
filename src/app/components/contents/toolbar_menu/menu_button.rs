use std::rc::Rc;

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
        }
    }
}
