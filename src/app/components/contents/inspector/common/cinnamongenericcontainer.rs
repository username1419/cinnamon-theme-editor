use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::io::parser::selector::Selector;
use crate::config::{Ancestry, AppConfiguration};

#[derive(Props, Clone, PartialEq)]
pub struct CinnamonGenericContainerProps {
    class: String,
    style: String,
    children: Element,
}

#[component]
pub fn CinnamonGenericContainer(props: CinnamonGenericContainerProps) -> Element {
    let config = use_context::<AppConfiguration>();
    let mut selected_element = config.selected_element;
    let mut current_element = config.current_element;
    *current_element.write() += 1;
    let element_id = *current_element.peek();

    let class = props.class;
    let style = props.style;

    // NOTE: (mostly) vibed

    // read existing ancestry (if any)
    let mut ancestry: Vec<String> = try_use_context::<Ancestry>()
        .map(|a| a.0)
        .unwrap_or_default();

    // trim and store own class
    let own = class.trim().to_string();

    // append own class to ancestry for descendants
    ancestry.push(own.clone());

    // provide ancestry for descendants
    use_context_provider(|| Ancestry(ancestry.clone()));

    // full ancestry for custom attribute
    let ancestry_attr = use_hook(move || ancestry.join(">"));

    rsx! {
        div {
            id: "{element_id}",
            class: "CinnamonGenericContainer {class}",
            style: "{style}",
            onclick: move |evt| {
                selected_element.set(Some(Selector::from_raw(&*ancestry_attr)));
                debug!("{ancestry_attr} clicked");
                evt.stop_propagation();
            },
            {props.children}
        }
    }
}
