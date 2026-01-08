use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::config::AppConfiguration;

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
    children: Element,
    orientation: Orientation,
}

#[component]
pub fn StBoxLayout(props: StBoxLayoutProps) -> Element {
    let config = use_context::<AppConfiguration>();
    let class = props.class;
    let style = props.style.unwrap_or_default();
    let orientation = props.orientation;

    rsx! {
        div { class: "StBoxLayout {class}", style: "{style}", {props.children} }
    }
}
