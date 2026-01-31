use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

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
    let mut editing_style = config.element_style;
    let element_id = *current_element.peek();
    let class = props.class;
    let mut use_original_style = use_signal(|| true);
    let mut is_style_override = use_signal(|| true);
    let mut this_style = use_signal(|| DeclarationBlock::from_raw(props.style.unwrap_or_default()));
    let style = use_memo(move || {
        if use_original_style() {
            this_style()
        } else {
            editing_style()
        }
    });
    let orientation = props.orientation;
    let mut selected = use_signal(|| false);
    let mut num_selected = config.element_selected;

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
        use_hook(move || Selector::from_raw(format!("inspector {}", ancestry.join(">")).as_str()));

    rsx! {
        div {
            id: "{element_id}",
            class: "StBoxLayout {class}",
            style: "{style.read().to_string()}",
            onclick: move |evt| {
                let ancestry_attr = ancestry_attr.clone();
                async move {
                    if selected() {
                        // TODO: probably make the footer show selected components or something
                        *selected.write() = false;
                        *num_selected.write() -= 1;
                        *this_style.write() = style.read().clone();
                        *use_original_style.write() = true;
                    }
                    if is_style_override() {
                        *editing_style.write() = style.read().cloned();
                        *is_style_override.write() = false;
                        *use_original_style.write() = false;
                    }
                    debug!("{} clicked", ancestry_attr.to_string());
                    *selected.write() = true;
                    *num_selected.write() += 1;
                    evt.stop_propagation();
                }
            },
            {props.children}
        }
    }
}
