use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::inspector_utils::InspectorUtil;
use crate::app::io::parser::declaration_block::DeclarationBlock;
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
    let editing_style = config.element_style;

    let is_style_override = use_signal(|| true);
    let class = props.class;
    let this_style = use_signal(|| DeclarationBlock::from_raw(String::new()));
    let selected = use_signal(|| false);
    let style = use_memo(move || {
        if !selected() {
            this_style()
        } else {
            editing_style()
        }
    });

    // NOTE: (mostly) vibed
    let is_multi_select = use_signal(|| false);
    let this_selection_group = use_signal(|| 0u32);

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

    let _ancestry_attr = ancestry_attr.clone();
    // Effect to auto-deselect when another single element is selected
    use_effect(move || {
        InspectorUtil::inspector_component_select_effect(
            is_style_override,
            this_style,
            selected,
            this_selection_group,
            _ancestry_attr.clone(),
            style,
        );
    });

    rsx! {
        div {
            class: "CinnamonGenericContainer {class}",
            style: "{style.read().to_string()};{props.style};",
            onclick: move |evt| InspectorUtil::inspector_component_onclick(
                evt,
                ancestry_attr.clone(),
                is_multi_select,
                this_selection_group,
            ),
            {props.children}
        }
    }
}
