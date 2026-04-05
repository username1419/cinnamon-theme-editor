use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use once_cell::sync::Lazy;

use crate::app::components::contents::inspector::common::cinnamongenericcontainer::CinnamonGenericContainer;
use crate::app::components::contents::inspector::common::stboxlayout::{Orientation, StBoxLayout};
use crate::app::dconf;
use crate::app::io::parser::selector::SelectorCategory;
use crate::config::AppConfiguration;

static PANEL_PROPERTIES: Lazy<Vec<(u8, u8, String)>> = Lazy::new(|| {
    dconf::CinnamonSettings::get_enabled_panels()
        .inspect_err(|e| error!("Error while retrieving panel properties: {}", e))
        .unwrap_or_default()
});

static PANELS_HEIGHT: Lazy<Vec<(u8, u8)>> = Lazy::new(|| {
    dconf::CinnamonSettings::get_panels_height()
        .inspect_err(|e| error!("Error while retreiving panels height: {}", e))
        .unwrap_or_default()
});

#[component]
pub fn Panel() -> Element {
    let config = use_context::<AppConfiguration>();
    let selected_category = config.inspector_type;
    let panels_height = use_hook(|| {
        let mut heights = PANEL_PROPERTIES
            .iter()
            .filter_map(|(panel_id, monitor_id, position)| {
                if 0.ne(monitor_id) {
                    return None;
                }

                let height = PANELS_HEIGHT
                    .iter()
                    .find(|(panel_id2, _)| panel_id == panel_id2)
                    .unwrap_or(&(0, 0))
                    .1;
                dbg!(Some((position.clone(), height)))
            })
            .collect::<HashMap<String, u8>>();
        for (pos, height) in vec![("right", 0), ("top", 0), ("bottom", 0), ("left", 0)] {
            if heights.get(pos).is_none() {
                heights.insert(pos.to_string(), height);
            }
        }

        heights
    });
    rsx! {
        div {
            class: "stage inspector-content",
            style: if selected_category.read().ne(&SelectorCategory::Panel) { "display: none" },
            CinnamonGenericContainer { class: "panel-top", style: "height: {panels_height[\"top\"]}px",
                StBoxLayout { class: "panelLeft", orientation: Orientation::Horizontal,
                    div {}
                }
                StBoxLayout {
                    class: "panelCenter",
                    orientation: Orientation::Horizontal,
                    div {}
                }
                StBoxLayout { class: "panelRight", orientation: Orientation::Horizontal,
                    div {}
                }
            }
            CinnamonGenericContainer { class: "panel-left", style: "width: {panels_height[\"left\"]}px",
                StBoxLayout { class: "panelLeft", orientation: Orientation::Vertical,
                    div {}
                }
                StBoxLayout { class: "panelCenter", orientation: Orientation::Vertical,
                    div {}
                }
                StBoxLayout { class: "panelRight", orientation: Orientation::Vertical,
                    div {}
                }
            }
            CinnamonGenericContainer { class: "panel-right", style: "width: {panels_height[\"right\"]}px",
                StBoxLayout { class: "panelLeft", orientation: Orientation::Vertical,
                    div {}
                }
                StBoxLayout { class: "panelCenter", orientation: Orientation::Vertical,
                    div {}
                }
                StBoxLayout { class: "panelRight", orientation: Orientation::Vertical,
                    div {}
                }
            }
            CinnamonGenericContainer { class: "panel-bottom", style: "height: {panels_height[\"bottom\"]}px",
                StBoxLayout { class: "panelLeft", orientation: Orientation::Horizontal,
                    div {}
                }
                StBoxLayout {
                    class: "panelCenter",
                    orientation: Orientation::Horizontal,
                    div {}
                }
                StBoxLayout { class: "panelRight", orientation: Orientation::Horizontal,
                    div {}
                }
            }
        }
    }
}
