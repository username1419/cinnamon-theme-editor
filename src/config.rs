use dioxus::{html::geometry::Coordinates, signals::Signal, stores::Store};

use crate::app::io::parse::StyleSheet;

#[derive(Clone)]
pub struct AppConfiguration {
    pub stylesheet: Store<StyleSheet>,
    pub mouse_state: Signal<Coordinates>,
}
