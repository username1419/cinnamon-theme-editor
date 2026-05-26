use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
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
    components::contents::property_editor::color::HSLColor,
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
    /// used to track inspector components. do not edit
    pub count_element: Signal<u32>,
    pub element_style: Signal<DeclarationBlock>,
    /// Collection of css selectors all current selected elements use
    pub selected_elements: SyncSignal<HashSet<Selector>>,
    /// Number of elements selected in the inspector.
    pub num_element_selected: SyncSignal<u32>,
    /// Notification for internal registered selection events. Notifies waiters immediately after
    /// an element is selected, and nothing has changed.
    pub elements_notify: Signal<Arc<Notify>>,
    /// Notification for internal registered selection events. The first holds the listening queue
    /// for non-clicked elements during update, the second `Notify` holds the notification listener
    /// for the clicked element. **Do not** use outside
    /// `app::components::contents::inspector::inspector_utils`
    pub elements_notify_confirm: SyncSignal<Option<Arc<(Notify, Notify)>>>,
    /// Notification for internal registered selection events. Notifies waiters immediately after
    /// selected_elements has been updated to only contain currently selected elements.
    pub elements_notify_updated: Signal<Arc<Notify>>,
}

#[derive(Clone)]
pub struct PropertyConfiguration {
    pub color_history: Signal<[HSLColor; 10]>,
    pub color_switch: Signal<bool>,
    pub current_bg_color: Signal<HSLColor>,
}

#[derive(Debug)]
pub struct MouseState {
    pub coordinates: Coordinates,
    pub mouse_down: MouseButtonSet,
}

#[derive(Clone)]
pub struct Ancestry(pub Vec<String>);
