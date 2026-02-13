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

#[derive(Clone, Copy, PartialEq)]
pub struct HSLColor {
    hue: u32,
    saturation: u32,
    lightness: u32,
    alpha: u32,
}

impl HSLColor {
    pub fn set_hue(&mut self, hue: u32) {
        self.hue = hue;
    }

    pub fn set_lightness(&mut self, lightness: u32) {
        self.lightness = lightness;
    }

    pub fn set_saturation(&mut self, saturation: u32) {
        self.saturation = saturation;
    }

    pub fn set_alpha(&mut self, alpha: u32) {
        self.alpha = alpha;
    }

    pub fn get_hue(&mut self) -> &u32 {
        &self.hue
    }

    pub fn get_lightness(&mut self) -> &u32 {
        &self.lightness
    }

    pub fn get_saturation(&mut self) -> &u32 {
        &self.saturation
    }

    pub fn get_alpha(&mut self) -> &u32 {
        &self.alpha
    }

    pub fn new(hue: u32, saturation: u32, lightness: u32, alpha: u32) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    pub fn to_normalized(&self) -> (f64, f64, f64, f64) {
        (
            self.hue as f64,
            (self.saturation as f64 / 100.0).clamp(0.0, 1.0),
            (self.lightness as f64 / 100.0).clamp(0.0, 1.0),
            (self.alpha as f64 / 100.0).clamp(0.0, 1.0),
        )
    }

    pub fn to_rgb(&self) -> (u8, u8, u8, u8) {
        let (h, s, l, a) = self.to_normalized();

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            (a * 255.0) as u8,
        )
    }

    pub fn as_css_property(&self) -> String {
        format!(
            "hsla({}, {}%, {}%, {}%)",
            self.hue, self.saturation, self.lightness, self.alpha
        )
    }
}

impl Default for HSLColor {
    fn default() -> Self {
        Self {
            hue: 0,
            saturation: 100,
            lightness: 100,
            alpha: 100,
        }
    }
}

#[component]
pub fn ColorPicker() -> Element {
    // TODO: set default as original element color somehow
    let config = use_context::<AppConfiguration>();
    let mouse_state = config.mouse_state;
    let mut editing_style = config.element_style;
    let mut selected_color = use_signal(|| HSLColor::default());

    let mut saturation_lightness_select_rect = use_signal(|| (0.0, 0.0));
    let mut cursor_pos: Signal<Point2D<f64, ElementSpace>> = use_signal(|| Point2D::origin());
    let mut history = config.color_history;
    let mut color_switch = config.color_switch;

    let refresh_rate = time::Duration::from_secs_f64(1.0 / 60.0);
    let refresh_rate_slow = time::Duration::from_secs_f64(1.0 / 10.0);
    let mut last_time = use_signal(|| Instant::now());
    let mut last_time_slow = use_signal(|| Instant::now());
    let mut offset: Signal<Point2D<f64, ClientSpace>> = use_signal(|| Point2D::origin());

    let mut cursor_style = use_signal(|| String::new());
    let mut color_preview_style = use_signal(|| String::new());

    use_effect(move || {
        if color_switch() {
            let mut writelock = history.write();
            let index = writelock.len() - 1;
            writelock[index] = selected_color.cloned();
            writelock.rotate_right(1);
            *color_switch.write() = false;
            debug!(
                "Added color {} to history",
                selected_color.peek().as_css_property()
            );
        }
    });

    rsx! {
        div { class: "color-picker",
            div { class: "saturation-brightness-picker-group color-previews",
                div {
                    class: "saturation-brightness-picker",
                    style: r#"background: linear-gradient(transparent, black), linear-gradient(to right, white, transparent), hsl({selected_color().hue}, 100%, 50%); "#,
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
                            *cursor_style.write() = format!("left: {}px; top: {}px;", relative_coord.x, relative_coord.y);

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

                            {
                                let mut selected_color = selected_color.write();
                                selected_color.set_lightness(lightness);
                                selected_color.set_saturation(saturation_percent);
                            }
                            *color_preview_style.write() = format!("background-color: {};", selected_color.peek().as_css_property());

                            let values = vec![
                                Value::from_raw_single(
                                    selected_color.peek().as_css_property()
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
                        style: r#"{cursor_style}{color_preview_style}"#,
                    }
                }
                div {
                    class: "color-preview",
                    style: "{color_preview_style}",
                }
            }
            input {
                // NOTE: i wanna show the selected hue on thumb but that would require
                // reimplementing range input which i dont wanna do
                r#type: "range",
                class: "hue-selector",
                min: 0,
                max: 359,
                value: "{selected_color().hue}",
                oninput: move |element| selected_color.write().set_hue(element.value().parse().unwrap()),
            }
            input {
                r#type: "range",
                class: "alpha-selector",
                min: 0,
                max: 100,
                value: "{selected_color().alpha}",
                oninput: move |element| selected_color.write().set_alpha(element.value().parse().unwrap()),
            }
            div { class: "color-history",
                for (index , color) in history().into_iter().enumerate() {
                    div {
                        id: "color-{index}",
                        style: r#"background-color: hsl({color.hue}, {color.saturation}%, {color.lightness}%, {color.alpha}%);"#,
                        onclick: move |_| {
                            debug!("Set selected color to {}", selected_color.peek().as_css_property());
                            selected_color.set(color.clone());
                            *color_preview_style.write() = format!("background-color: {};", color.as_css_property());
                            let (_, s, l, _) = color.to_normalized();
                            let bounds = saturation_lightness_select_rect.peek();
                            *cursor_style.write() = format!("left: {}px; top: {}px;", l * bounds.1, s * bounds.0);
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
                // TODO: (split-/)complementary/analogus colors mb
        // TODO: color history
        }
    }
}
