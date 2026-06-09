use dioxus::prelude::*;

use crate::config::AppConfiguration;

#[derive(Props, Clone, PartialEq, Eq, Default)]
pub struct StatusbarProps {}

#[component]
#[allow(unused)]
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

const SELECTION_CHAR_LIMIT: usize = 35;

#[component]
fn SelectionViewer() -> Element {
    let config = use_context::<AppConfiguration>();
    let selected = config.selected_elements;
    let mut selection_text = use_signal(String::new);
    let mut selected_list = use_signal(String::new);

    use_effect(move || {
        // NOTE: it doesnt check for selection update but i dont care
        let selected = selected();
        let selected_count = selected.iter().len();
        let mut _selected_list = if selected_count == 0 {
            String::new()
        } else {
            selected
                .iter()
                .map(|e| e.to_export_safe().to_string() + ", ")
                .collect::<String>()
        };

        if _selected_list.len() > SELECTION_CHAR_LIMIT {
            _selected_list.truncate(SELECTION_CHAR_LIMIT);
            _selected_list.push_str("...");
        }

        *selection_text.write() =
            format!("Editing {} elements: {}", selected_count, _selected_list);
        *selected_list.write() = _selected_list;
    });

    rsx! {
        div {
            class: "selection-viewer",
            span { class: "selection-text", title: "{selected_list}", "{selection_text}" }
        }
    }
}
