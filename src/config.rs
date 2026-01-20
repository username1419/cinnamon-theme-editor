use dioxus::{
    html::{geometry::Coordinates, input_data::MouseButtonSet},
    signals::Signal,
    stores::Store,
};

use crate::app::io::{
    parse::StyleSheet,
    parser::selector::{Selector, SelectorCategory},
};

#[derive(Clone)]
pub struct AppConfiguration {
    pub is_editing: Signal<bool>,
    pub default_style: Signal<String>,
    pub editing_stylesheet: Store<StyleSheet>,
    pub mouse_state: Signal<MouseState>,
    pub inspector_type: Signal<SelectorCategory>,
    // used to automatically assign inspector components' ids
    pub current_element: Signal<u32>,
    pub selected_element: Signal<Option<Selector>>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}

#[derive(Clone)]
pub struct Ancestry(pub Vec<String>);
