use dioxus::{
    html::{geometry::Coordinates, input_data::MouseButtonSet},
    signals::Signal,
    stores::Store,
};

use crate::app::io::parse::StyleSheet;

#[derive(Clone)]
pub struct AppConfiguration {
    pub is_editing: Signal<bool>,
    pub stylesheet: Store<StyleSheet>,
    pub mouse_state: Signal<MouseState>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}
