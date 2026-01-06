use dioxus::{
    html::{geometry::Coordinates, input_data::MouseButtonSet},
    signals::Signal,
    stores::Store,
};

use crate::app::io::{parse::StyleSheet, parser::selector::SelectorCategory};

#[derive(Clone)]
pub struct AppConfiguration {
    pub is_editing: Signal<bool>,
    pub default_style: Signal<String>,
    pub editing_stylesheet: Store<StyleSheet>,
    pub mouse_state: Signal<MouseState>,
    pub inspector_type: Signal<SelectorCategory>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}
