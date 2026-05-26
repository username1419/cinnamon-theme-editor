use dioxus::{core::Element, prelude::*};

#[component]
pub fn EditorSection(class: String, label: String, children: Element) -> Element {
    let mut style = use_signal(String::new);

    rsx! {
        div {
            class: "collapsible-container",
            onclick: move |_| {
                if &style() == "display: none" {
                    style.set("display: flex".to_string());
                } else {
                    style.set("display: none".to_string());
                }
            },
            div {
                class: "collapsible-label",
                span { "{label}" },
                span { style: "right: 0%; margin-left: auto", if &style() == "display: none" { "+" } else { "-" } },
            }
            div {
                class: "collapsible {class}",
                style: "{style}",
                onclick: move |evt| evt.stop_propagation(),
                {children}
            }
        }
    }
}
