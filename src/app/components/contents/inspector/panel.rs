use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::common::cinnamongenericcontainer::CinnamonGenericContainer;
use crate::app::components::contents::inspector::common::stboxlayout::{Orientation, StBoxLayout};
use crate::app::dconf;
use crate::app::io::parser::selector::SelectorCategory;
use crate::config::AppConfiguration;

#[component]
pub fn Panel() -> Element {
    let config = use_context::<AppConfiguration>();
    let selected_category = config.inspector_type;
    let panel_properties = dconf::CinnamonSettings::get_enabled_panels()
        .inspect_err(|e| error!("Error while retrieving panel properties: {}", e))
        .unwrap_or_default();
    let panels_height = dconf::CinnamonSettings::get_panels_height()
        .inspect_err(|e| error!("Error while retreiving panels height: {}", e))
        .unwrap_or_default();
    let panels_height = use_hook(|| {
        // [top, left, right, bottom]
        let mut out = [0 as u8; 4];
        // PERF: this is o(n*m) but the size of this isnt that big so its probably fine
        // also i cant think of any other way to do this (theres likely a better way to)
        panel_properties
            .into_iter()
            .filter(|v| v.1 == 0)
            .for_each(|t| {
                let size = panels_height
                    .iter()
                    .find_map(|t1| if t1.0 == t.0 { Some(t1.1) } else { None })
                    .unwrap_or(0);
                match t.2.as_str() {
                    "top" => out[0] = size,
                    "left" => out[1] = size,
                    "right" => out[2] = size,
                    "bottom" => out[3] = size,
                    _ => (),
                }
            });

        out
    });
    rsx! {
        // NOTE: we gotta treat certain widget names as classes for this to work
        div {
            class: "stage inspector-content",
            style: if selected_category.read().ne(&SelectorCategory::Panel) { "display: none" },
            CinnamonGenericContainer { class: "panel-top", style: "height: {panels_height[0]}px",
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
            CinnamonGenericContainer { class: "panel-left", style: "width: {panels_height[1]}px",
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
            CinnamonGenericContainer { class: "panel-right", style: "width: {panels_height[2]}px",
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
            CinnamonGenericContainer { class: "panel-bottom", style: "height: {panels_height[3]}px",
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
