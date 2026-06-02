use dioxus::prelude::*;

use crate::config::AppConfiguration;

#[derive(Props, Clone, PartialEq, Eq, Default)]
pub struct StatusbarProps {}

#[component]
pub fn Statusbar(props: StatusbarProps) -> Element {
    rsx! {
        div {
            class: "statusbar",

            div {
                class: "statusbar-part start"
            }
            div {
                class: "statusbar-part middle"
            }
            div {
                class: "statusbar-part end",
                SelectionViewer {},
            }
        }
    }
}

#[component]
fn SelectionViewer() -> Element {
    let config = use_context::<AppConfiguration>();
    let selected = config.selected_elements;
    let selection_text = use_memo(move || {
        let selected = selected();
        let selected_count = selected.iter().len();
        let mut selected_list = selected
            .iter()
            .map(|e| e.to_string() + ", ")
            .collect::<String>();

        if selected_list.len() > 18 {
            selected_list.truncate(18);
            selected_list.push_str("...");
        }

        format!("Editing {} elements: {}", selected_count, selected_list)
    });

    rsx! {
        div {
            class: "selection-viewer",
            span { class: "content", "{selection_text}" }
        }
    }
}
