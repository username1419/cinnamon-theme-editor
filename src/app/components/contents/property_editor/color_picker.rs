use crate::app::components::contents::property_editor::color::HSLColor;
use dioxus::html::geometry::ElementSpace;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use tokio::time::{self, Instant};

use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::Value;
use crate::config::{AppConfiguration, PropertyConfiguration};

#[component]
pub fn ColorPicker(
    color: Signal<HSLColor>,
    on_color_change: Option<Callback<HSLColor>>,
) -> Element {
    let app_config = use_context::<AppConfiguration>();
    let property_config = use_context::<PropertyConfiguration>();
    let mut editing_style = app_config.element_style;
    let mut selected_color = color;

    let mut saturation_lightness_select_rect = use_signal(|| (0.0, 0.0));
    let mut cursor_pos: Signal<Point2D<f64, ElementSpace>> = use_signal(Point2D::origin);
    let history = property_config.color_history;
    let mut color_switch = property_config.color_switch;

    let refresh_rate = time::Duration::from_secs_f64(1.0 / 60.0);
    let refresh_rate_slow = time::Duration::from_secs_f64(1.0 / 10.0);
    let mut last_time = use_signal(Instant::now);
    let mut last_time_slow = use_signal(Instant::now);

    let cursor_style = use_memo(move || {
        let bounds = *saturation_lightness_select_rect.peek();
        format!(
            "left: {}%; top: {}%; ",
            (cursor_pos().x / bounds.0 * 100.0).clamp(0.0, 100.0),
            (cursor_pos().y / bounds.1 * 100.0).clamp(0.0, 100.0)
        )
    });
    let color_preview_style = use_memo(move || {
        let color = selected_color();
        format!("background-color: {};", color.as_css_property())
    });
    let complementary_color_preview_style = use_memo(move || {
        let color = selected_color().get_complementary();
        format!("background-color: {};", color.as_css_property())
    });

    use_effect(move || {
        if color_switch() {
            let color = color.peek();
            let bounds = *saturation_lightness_select_rect.peek();
            let saturation = *color.get_saturation() as f64 / 100.0;
            let lightness = *color.get_lightness() as f64 / 100.0;
            // Inverse of the drag mapping in onmousemove (HSV value ↔ HSL lightness).
            let value_denom = 1.0 - saturation / 2.0;
            let normalized_y = if value_denom.abs() < f64::EPSILON {
                0.0
            } else {
                (1.0 - lightness / value_denom).clamp(0.0, 1.0)
            };
            let sl_coord = Point2D::new(
                (saturation * bounds.0).clamp(0.0, bounds.0),
                (normalized_y * bounds.1).clamp(0.0, bounds.1),
            );
            debug!("setting cursor pos to {:?}", sl_coord);
            *cursor_pos.write() = sl_coord;
            color_switch.set(false);
        }
    });

    rsx! {
        div { class: "color-picker",
            div { class: "saturation-brightness-picker-group color-previews",
                div {
                    class: "saturation-brightness-picker",
                    style: r#"background: linear-gradient(transparent, black), linear-gradient(to right, white, transparent), hsl({selected_color().get_hue()}, 100%, 50%); "#,
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
                            if !app_config.mouse_state.peek().mouse_down.contains(MouseButton::Primary) {
                                return;
                            }

                            // NOTE: reducing refresh rate on my machine only decrases cpu/gpu
                            // usage by between 0-0.5%
                            let delta = time::Instant::now().duration_since(*last_time.peek());
                            if delta < refresh_rate {
                                return;
                            }

                            let relative_coord = event.element_coordinates();
                            *cursor_pos.write() = relative_coord;

                            // NOTE: changing this refresh rate doesnt really improve anything
                            // either, its probably something ive fucked up in auto reactivity
                            *last_time.write() = time::Instant::now();
                            let delta_s = time::Instant::now().duration_since(*last_time_slow.peek());
                            if delta_s < refresh_rate_slow {
                                return;
                            }

                            let bounds = *saturation_lightness_select_rect.peek();
                            // normalize cursor position relative to bounding box
                            let normalized_x = (relative_coord.x / bounds.0).clamp(0.0, 1.0);
                            let normalized_y = (relative_coord.y / bounds.1).clamp(0.0, 1.0);

                            let saturation = normalized_x;
                            let value = 1.0 - normalized_y;
                            // conversion to hsl bc im stupid
                            let lightness = ((value * (1.0 - saturation / 2.0)) * 100.0) as u16;
                            let saturation_percent = (saturation * 100.0) as u16;

                            {
                                let mut selected_color = selected_color.write();
                                selected_color.set_lightness(lightness);
                                selected_color.set_saturation(saturation_percent);
                                if let Some(callback) = on_color_change {
                                    callback(*selected_color);
                                }
                            }
                            *last_time_slow.write() = time::Instant::now();
                        }
                    },
                    div {
                        class: "saturation-brightness-cursor",
                        style: r#"{cursor_style}{color_preview_style}"#,
                    }
                }
                div {
                    class: "color-preview",
                    div {
                        class: "color-preview-original",
                        title: "Current color",
                        style: "{color_preview_style}",
                    }
                    div {
                        class: "color-preview-complementary",
                        title: "Complementary color",
                        style: "{complementary_color_preview_style}",
                        onclick: move |_| {
                            let color = selected_color.peek().get_complementary();
                            debug!("Set selected color to {} by complementary", color.as_css_property());
                            let values = vec![
                                Value::from_raw_single(
                                    color.as_css_property()
                                        .as_str(),
                                ),
                            ];
                            editing_style
                                .write()
                                .set_style_attribute(Property::from_raw("background-color"), values);
                            selected_color.set(color);
                        },
                        "C"
                    }
                    // TODO: split-complementary/analogus colors mb
                }
            }
            input {
                // NOTE: i wanna show the selected hue on thumb but that would require
                // reimplementing range input which i dont wanna do
                r#type: "range",
                class: "hue-selector",
                min: 0,
                max: 359,
                value: "{selected_color().get_hue()}",
                oninput: move |element| selected_color.write().set_hue(element.value().parse().unwrap()),
            }
            input {
                r#type: "range",
                class: "alpha-selector",
                min: 0,
                max: 100,
                value: "{selected_color().get_alpha()}",
                oninput: move |element| selected_color.write().set_alpha(element.value().parse().unwrap()),
            }
            div { class: "color-history",
                for (index, color) in history().into_iter().enumerate() {
                    div {
                        id: "color-{index}",
                        style: r#"background-color: hsl({color.get_hue()}, {color.get_saturation()}%, {color.get_lightness()}%, {color.get_alpha()}%);"#,
                        onclick: move |_| {
                            debug!("Set selected color to {} by history", color.as_css_property());
                            selected_color.set(color);
                            let values = vec![
                                Value::from_raw_single(
                                    color.as_css_property()
                                        .as_str(),
                                ),
                            ];
                            editing_style
                                .write()
                                .set_style_attribute(Property::from_raw("background-color"), values);
                        },
                    }
                }
            }
        }
    }
}
