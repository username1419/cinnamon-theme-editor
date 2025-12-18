use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

#[derive(Props, PartialEq, Clone)]
pub struct MenuButtonProps {
    id: String,
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    shortcut_trigger: Option<EventHandler<KeyboardEvent>>,
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
                if let Some(trigger) = props.shortcut_trigger {
                    trigger.call(event)
                }
            },
            "{text}"
        }
    }
}
