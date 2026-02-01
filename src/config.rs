use std::collections::HashMap;

use dioxus::{
    html::{geometry::Coordinates, input_data::MouseButtonSet},
    signals::Signal,
    stores::Store,
};

use crate::app::io::parser::{declaration_block::DeclarationBlock, selector::Selector};
use crate::app::io::{parse::StyleSheet, parser::selector::SelectorCategory};

#[derive(Clone)]
pub struct AppConfiguration {
    pub is_dirty: Signal<bool>,
    pub is_editing: Signal<bool>,
    pub default_style: Signal<HashMap<SelectorCategory, String>>,
    pub editing_stylesheet: Store<StyleSheet>,
    /// Cursor position relative to viewport
    pub mouse_state: Signal<MouseState>,
    pub inspector_type: Signal<SelectorCategory>,
    /// used to automatically assign inspector components' ids. do not use
    pub current_element: Signal<u32>,
    pub element_style: Signal<DeclarationBlock>,
    /// Number of elements selected in the inspector.
    pub num_element_selected: Signal<u32>,
    pub selection_group: Signal<u32>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}

#[derive(Clone)]
pub struct Ancestry(pub Vec<String>);
