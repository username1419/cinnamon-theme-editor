use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::config::AppConfiguration;

const CURSOR_SIZE: f64 = 8.0;

#[component]
pub fn ColorPicker() -> Element {
    let config = use_context::<AppConfiguration>();
    let mut selected_hue = use_signal(|| 0 as u32);
    let mut selected_saturation = use_signal(|| 0);
    let mut selected_lightness = use_signal(|| 100);

    let mut saturation_lightness_select_rect = use_signal(|| (0.0, 0.0));
    let mut cursor_pos = use_signal(|| (0.0, 0.0));

    rsx! {
        div { class: "color-picker",
            div {
                class: "saturation-brightness-picker",
                style: r#"background: linear-gradient(transparent, black), linear-gradient(to right, white, transparent), hsl({selected_hue}, 100%, 50%); "#,
                onresize: move |e| {
                    if let Ok(bounds) = e.get_content_box_size() {
                        saturation_lightness_select_rect.set(bounds.to_tuple());
                    }
                },
                onmousemove: move |event| {
                    if !config.mouse_state.read().mouse_down.contains(MouseButton::Primary) {
                        return;
                    }
                    let relative_coord = event.element_coordinates();
                    let absolute_coord = event.client_coordinates();
                    let offset = (
                        absolute_coord.x - relative_coord.x,
                        absolute_coord.y - relative_coord.y,
                    );
                    let bounds = *saturation_lightness_select_rect.read();

                    let cursor_x = (relative_coord.x + offset.0 - CURSOR_SIZE)
                        .clamp(offset.0 - CURSOR_SIZE, offset.0 + bounds.0 + CURSOR_SIZE);
                    let cursor_y = (relative_coord.y + offset.1 - CURSOR_SIZE)
                        .clamp(offset.1 - CURSOR_SIZE, offset.1 + bounds.1 + CURSOR_SIZE);

                    cursor_pos.set((cursor_x, cursor_y));
                    let saturation = (cursor_x - offset.0 - CURSOR_SIZE) / (bounds.0 + CURSOR_SIZE);
                    let value = 1.0

                        // conversion to hsl bc im stupid
                        - ((cursor_y - offset.1 - CURSOR_SIZE) / (bounds.1 + CURSOR_SIZE));
                    selected_lightness.set(((value * (1.0 - saturation / 2.0)) * 100.0) as u32);
                    selected_saturation.set((saturation * 100.0) as u32);
                },
            }
            div {
                class: "saturation-brightness-cursor",
                style: r#"left: {cursor_pos.read().0}px; top: {cursor_pos.read().1}px;background-color: hsl({selected_hue}, {selected_saturation}%, {selected_lightness}%);"#,
            }
            input {
                // NOTE: i wanna show the selected hue on thumb but that would require
                // reimplementing range input which i dont wanna do
                r#type: "range",
                class: "hue-selector",
                min: 0,
                max: 359,
                value: "{selected_hue}",
                oninput: move |element| selected_hue.set(element.value().parse().unwrap()),
            }
                // TODO: (split-/)complementary/analogus colors mb
        // TODO: color history
        // TODO: alpha range input
        }
    }
}
