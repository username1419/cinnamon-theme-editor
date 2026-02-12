use dioxus::html::geometry::euclid::Point2D;
use dioxus::html::geometry::{ClientSpace, ElementSpace};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use tokio::time::{self, Instant};

use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::Value;
use crate::config::AppConfiguration;

#[component]
pub fn ColorPicker() -> Element {
    // TODO: set default as original element color somehow
    let config = use_context::<AppConfiguration>();
    let mouse_state = config.mouse_state;
    let mut editing_style = config.element_style;
    let mut selected_hue = use_signal(|| 0 as u32);
    let mut selected_saturation = use_signal(|| 0);
    let mut selected_lightness = use_signal(|| 100);
    let mut selected_alpha = use_signal(|| 100);

    let mut saturation_lightness_select_rect = use_signal(|| (0.0, 0.0));
    let mut cursor_pos: Signal<Point2D<f64, ElementSpace>> = use_signal(|| Point2D::origin());
    let mut history = use_signal(|| [(0 as u32, 100 as i32, 100 as i32, 100 as i32); 10]);

    let refresh_rate = time::Duration::from_secs_f64(1.0 / 60.0);
    let refresh_rate_slow = time::Duration::from_secs_f64(1.0 / 10.0);
    let mut last_time = use_signal(|| Instant::now());
    let mut last_time_slow = use_signal(|| Instant::now());
    let mut offset: Signal<Point2D<f64, ClientSpace>> = use_signal(|| Point2D::origin());

    rsx! {
        div { class: "color-picker",
            div { class: "saturation-brightness-picker-group color-previews",
                div {
                    class: "saturation-brightness-picker",
                    style: r#"background: linear-gradient(transparent, black), linear-gradient(to right, white, transparent), hsl({selected_hue}, 100%, 50%); "#,
                    onresize: move |e| {
                        if let Ok(bounds) = e.get_content_box_size() {
                            saturation_lightness_select_rect.set(bounds.to_tuple());
                        }
                    },
                    onmousemove: move |event| {
                        // PERF: high cpu & gpu usage when dragging
                        // ~4% CPU (Intel i5-13420H)
                        // ~7% GPU (GeForce RTX 4050 Mobile)
                        async move {
                            if !config.mouse_state.peek().mouse_down.contains(MouseButton::Primary) {
                                return;
                            }

                            // NOTE: reducing refresh rate on my machine only decrases cpu/gpu
                            // usage by between 0-0.5%
                            let delta = time::Instant::now().duration_since(*last_time.peek());
                            if delta < refresh_rate {
                                return;
                            }

                            let bounds = *saturation_lightness_select_rect.peek();

                            let global_coord = mouse_state.peek().coordinates.client();
                            if offset.peek().eq(&Point2D::origin()) {
                                let relative_coord = event.element_coordinates();
                                *offset.write() = Point2D::new(global_coord.x - relative_coord.x, global_coord.y - relative_coord.y);
                            }
                            let offset = offset.peek();
                            let relative_coord: Point2D<f64, ElementSpace> = Point2D::new(global_coord.x - offset.x, global_coord.y - offset.y);
                            *cursor_pos.write() = relative_coord.clone();

                            // NOTE: changing this refresh rate doesnt really improve anything
                            // either, its probably something ive fucked up in auto reactivity
                            *last_time.write() = time::Instant::now();
                            let delta_s = time::Instant::now().duration_since(*last_time_slow.peek());
                            if delta_s < refresh_rate_slow {
                                return;
                            }

                            // normalize cursor position relative to bounding box
                            let normalized_x = (relative_coord.x / bounds.0).clamp(0.0, 1.0);
                            let normalized_y = (relative_coord.y / bounds.1).clamp(0.0, 1.0);

                            let saturation = normalized_x;
                            let value = 1.0 - normalized_y;
                            // conversion to hsl bc im stupid
                            let lightness = ((value * (1.0 - saturation / 2.0)) * 100.0) as u32;
                            let saturation_percent = (saturation * 100.0) as u32;
                            if lightness > 90 {
                                debug!("{:?}", relative_coord);
                                debug!("{} {}", normalized_x, normalized_y);
                            }

                            *selected_lightness.write() = lightness;
                            *selected_saturation.write() = saturation_percent;

                            let values = vec![
                                Value::from_raw_single(
                                    format!(
                                        "hsla({selected_hue}, {selected_saturation}%, {selected_lightness}%, {selected_alpha}%)",
                                    )
                                        .as_str(),
                                ),
                            ];
                            editing_style
                                .write()
                                .set_style_attribute(Property::from_raw("background-color"), values);
                            *last_time_slow.write() = time::Instant::now();
                        }
                    },
                    div {
                        class: "saturation-brightness-cursor",
                        style: r#"left: {cursor_pos().x}px; top: {cursor_pos().y}px;background-color: hsl({selected_hue}, {selected_saturation}%, {selected_lightness}%);"#,
                    }
                }
                div {
                    class: "color-preview",
                    style: "background-color: hsla({selected_hue}, {selected_saturation}%, {selected_lightness}%, {selected_alpha}%);",
                }
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
            input {
                r#type: "range",
                class: "alpha-selector",
                min: 0,
                max: 100,
                value: "{selected_alpha}",
                oninput: move |element| selected_alpha.set(element.value().parse().unwrap()),
            }
            div { class: "color-history",
                for (index , color) in history().iter().enumerate() {
                    div {
                        id: "color-{index}",
                        style: r#"background-color: hsl({color.0}, {color.1}%, {color.2}%, {color.3}%);"#,
                        onclick: move |_| {
                            // TODO:
                            info!("TODO: change color");
                        },
                    }
                }
            }
                // TODO: (split-/)complementary/analogus colors mb
        // TODO: color history
        }
    }
}
