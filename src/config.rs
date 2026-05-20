use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Weak},
};

use dioxus::{
    html::{geometry::Coordinates, input_data::MouseButtonSet},
    signals::{Signal, SyncSignal},
};
use tokio::sync::Notify;

use crate::app::io::{
    parse::StyleSheet,
    parser::selector::{Selector, SelectorCategory},
};
use crate::app::{
    components::contents::property_config::color_picker::HSLColor,
    io::parser::declaration_block::DeclarationBlock,
};

#[derive(Clone)]
pub struct AppConfiguration {
    pub is_dirty: Signal<bool>,
    pub is_editing: Signal<bool>,
    pub default_style: Signal<HashMap<SelectorCategory, StyleSheet>>,
    pub editing_stylesheet: SyncSignal<HashMap<SelectorCategory, StyleSheet>>,
    /// Cursor position relative to viewport
    pub mouse_state: Signal<MouseState>,
    pub inspector_type: Signal<SelectorCategory>,
    /// used to automatically assign inspector components' ids. do not use
    pub count_element: Signal<u32>,
    pub element_style: Signal<DeclarationBlock>,
    /// Collection of css selectors all current selected elements use
    pub selected_elements: SyncSignal<HashSet<Selector>>,
    /// Number of elements selected in the inspector.
    pub num_element_selected: SyncSignal<u32>,
    pub color_history: Signal<[HSLColor; 10]>,
    pub color_switch: Signal<bool>,
    pub elements_notify: Signal<Arc<Notify>>,
    pub elements_notify_confirm: SyncSignal<Option<Weak<Notify>>>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}

#[derive(Clone)]
pub struct Ancestry(pub Vec<String>);
