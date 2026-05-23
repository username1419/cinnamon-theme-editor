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
    let editing_style = config.element_style;
    let class = props.class;
    let this_style = use_signal(|| DeclarationBlock::from_raw(props.style.unwrap_or_default()));
    #[allow(unused)]
    let orientation = props.orientation;
    let selected = use_signal_sync(|| false);
    let style = use_memo(move || {
        if !selected() {
            this_style()
        } else {
            editing_style()
        }
    });

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

    let _ancestry_attr = ancestry_attr.clone();
    let __ancestry_attr = ancestry_attr.clone();

    use_hook(|| {
        *consume_context::<AppConfiguration>().count_element.write() += 1;
        debug!("registered element {}", own);
    });

    use_future(move || {
        let _ancestry_attr = _ancestry_attr.clone();
        InspectorUtil::inspector_component_background_watcher(selected, _ancestry_attr, this_style)
    });

    rsx! {
        div {
            class: "StBoxLayout {class}",
            style: "{style().to_string()}",
            onclick: move |evt| InspectorUtil::inspector_component_onclick(
                evt,
                selected,
                __ancestry_attr.clone(),
            ),
            {props.children}
        }
    }
}
