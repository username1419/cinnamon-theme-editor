use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::inspector_utils::InspectorUtil;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use crate::app::io::parser::selector::Selector;
use crate::config::{Ancestry, AppConfiguration};

#[derive(Clone, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Props, PartialEq, Clone)]
pub struct StBoxLayoutProps {
    class: String,
    #[props(optional)]
    style: Option<String>,
    #[props(optional)]
    selector: Option<String>,
    children: Element,
    orientation: Orientation,
}

#[component]
pub fn StBoxLayout(props: StBoxLayoutProps) -> Element {
    let config = use_context::<AppConfiguration>();
    let mut current_element = config.current_element;
    *current_element.write() += 1;
    let editing_style = config.element_style;
    let element_id = *current_element.peek();
    let class = props.class;
    let use_original_style = use_signal(|| true);
    let is_style_override = use_signal(|| true);
    let this_style = use_signal(|| DeclarationBlock::from_raw(props.style.unwrap_or_default()));
    let style = use_memo(move || {
        if use_original_style() {
            this_style()
        } else {
            editing_style()
        }
    });
    let orientation = props.orientation;
    let selected = use_signal(|| false);

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
    let ancestry_attr =
        use_hook(move || Selector::from_raw(format!(".inspector {}", ancestry.join(">")).as_str()));

    rsx! {
        div {
            id: "{element_id}",
            class: "StBoxLayout {class}",
            style: "{style.read().to_string()}",
            onclick: move |evt| InspectorUtil::inspector_component_onclick(
                evt,
                selected,
                use_original_style,
                is_style_override,
                ancestry_attr.clone(),
                this_style,
                style,
            ),
            {props.children}
        }
    }
}
