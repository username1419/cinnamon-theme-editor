use dioxus::html::geometry::{Coordinates, euclid::Point2D};
use dioxus_desktop::wry::dpi::PhysicalPosition;

use crate::app::io::parser::selector::SelectorCategory;

pub struct Helper; // idk why im doing this but it feels like its right to my oop addicted brain
impl Helper {
    pub fn to_coord(pos: PhysicalPosition<f64>) -> Coordinates {
        let screen = Point2D::new(pos.x, pos.y);
        let client = Point2D::new(pos.x, pos.y);
        let element = Point2D::new(pos.x, pos.y);
        let page = Point2D::new(pos.x, pos.y);

        Coordinates::new(screen, client, element, page)
    }

    pub fn get_heirarchy_key(class: &String) -> SelectorCategory {
        let class = class.to_lowercase();

        if class.contains("panel") {
            return SelectorCategory::Panel;
        }

        SelectorCategory::Other
    }
}
