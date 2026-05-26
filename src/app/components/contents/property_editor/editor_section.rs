use std::time::Duration;

use dioxus::{core::Element, prelude::*};
use tokio::time;

#[component]
pub fn EditorSection(class: String, label: String, children: Element) -> Element {
    let mut style = use_signal(|| String::from("display: none"));
    let mut height_style = use_signal(move || format!("max-height: {}%", 0));
    let mut opened = use_signal(String::new);

    rsx! {
        div {
            class: "collapsible-container",
            onclick: move |_| async move {
                if &style() == "display: none" {
                    style.set("display: flex".to_string());
                    opened.set("open".to_string());
                    height_style.set(format!("max-height: {}%", 100));
                } else {
                    opened.set(String::new());
                    height_style.set(format!("max-height: {}%", 0));
                    time::sleep(Duration::from_secs_f64(0.2)).await;
                    style.set("display: none".to_string());
                }
            },
            div {
                class: "collapsible-label",
                span { "{label}" },
                span { style: "right: 0%; margin-left: auto; margin-right: 4px", if &style() == "display: none" { "[+]" } else { "[—]" } },
            }
            div {
                class: "collapsible-wrapper {opened}",
                div {
                    class: "collapsible {class}",
                    style: "{height_style}; {style}",
                    onclick: move |evt| {
                        evt.stop_propagation();
                    },
                    {children}
                }
            }
        }
    }
}
