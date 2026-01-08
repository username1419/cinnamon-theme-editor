use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::inspector::common::cinnamongenericcontainer::CinnamonGenericContainer;
use crate::app::components::contents::inspector::common::stboxlayout::{Orientation, StBoxLayout};
use crate::app::io::parser::selector::SelectorCategory;
use crate::config::AppConfiguration;

#[component]
pub fn Panel() -> Element {
    let config = use_context::<AppConfiguration>();
    let selected_category = config.inspector_type;
    rsx! {
        // NOTE: we gotta treat certain widget names as classes for this to work
        div {
            class: "stage inspector-content",
            style: if selected_category.read().ne(&SelectorCategory::Panel) { "display: none" },
            CinnamonGenericContainer { class: "panel-top",
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
            CinnamonGenericContainer { class: "panel-left",
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
            CinnamonGenericContainer { class: "panel-right",
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
            CinnamonGenericContainer { class: "panel-bottom",
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
