use dioxus::signals::Signal;

use crate::app::io::parse::StyleSheet;

#[derive(Default, Clone)]
pub struct AppConfiguration {
    pub stylesheet: Signal<StyleSheet>,
}
