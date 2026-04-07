use std::u8;

use dioxus::html::geometry::ElementSpace;
use dioxus::html::geometry::euclid::Point2D;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::time::{self, Instant};

use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::Value;
use crate::config::AppConfiguration;

#[derive(Clone, Copy, PartialEq)]
pub struct HSLColor {
    hue: u16,
    saturation: u16,
    lightness: u16,
    alpha: u16,
}

impl HSLColor {
    pub fn set_hue(&mut self, hue: u16) {
        self.hue = hue;
    }

    pub fn set_lightness(&mut self, lightness: u16) {
        self.lightness = lightness;
    }

    pub fn set_saturation(&mut self, saturation: u16) {
        self.saturation = saturation;
    }

    pub fn set_alpha(&mut self, alpha: u16) {
        self.alpha = alpha;
    }

    pub fn get_hue(&self) -> &u16 {
        &self.hue
    }

    pub fn get_lightness(&self) -> &u16 {
        &self.lightness
    }

    pub fn get_saturation(&self) -> &u16 {
        &self.saturation
    }

    pub fn get_alpha(&self) -> &u16 {
        &self.alpha
    }

    pub fn new(hue: u16, saturation: u16, lightness: u16, alpha: u16) -> Self {
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

    pub fn get_complementary(&self) -> HSLColor {
        let mut col = self.clone();
        col.hue = (col.hue + 180) % 360;
        col
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> HSLColor {
        let (r, g, b, a) = (r as f64, g as f64, b as f64, a as f64);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        // Saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        } as u16;
        let l = l as u16;

        // Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h } as u16;

        // Alpha
        let a = ((a as f64) / 255.0 * 100.0) as u16;

        HSLColor {
            hue: h,
            saturation: s,
            lightness: l,
            alpha: a,
        }
    }

    const RGB_MATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"rgb\((\d+),(\d+),(\d+)\)"#).expect("Unable to compile regex: RGB_MATCH")
    });

    const RGBA_MATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"rgb\((\d+),(\d+),(\d+),(\d+)\)"#)
            .expect("Unable to compile regex: RGBA_MATCH")
    });

    const HSL_MATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"hsla\((\d+),(\d+),(\d+)\)"#).expect("Unable to compile regex: HSL_MATCH")
    });

    const HSLA_MATCH: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"hsla\((\d+),(\d+),(\d+),(\d+)\)"#)
            .expect("Unable to compile regex: HSLA_MATCH")
    });

    /// Tries to convert a CSS color to an internal representation. Currently supports rgb(),
    /// rgba(), hsl(), hsla(), and shortened rgb/rgba values.
    ///
    /// Please do not use this in anywhere performance is needed it uses like 4 different regex
    /// checks
    pub(crate) fn from_css_property(color: String) -> Option<Self> {
        let color = color
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if Self::HSLA_MATCH.is_match(&*color) {
            let components = Self::HSLA_MATCH.captures(&*color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            Some(HSLColor::new(
                components[0],
                components[1],
                components[2],
                components[3],
            ))
        } else if Self::HSL_MATCH.is_match(&*color) {
            let components = Self::HSL_MATCH.captures(&*color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            Some(HSLColor::new(
                components[0],
                components[1],
                components[2],
                100,
            ))
        } else if Self::RGB_MATCH.is_match(&*color) {
            let components = Self::RGB_MATCH.captures(&*color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            Some(HSLColor::from_rgba(
                components[0],
                components[1],
                components[2],
                u8::MAX,
            ))
        } else if Self::RGBA_MATCH.is_match(&*color) {
            let components = Self::RGBA_MATCH.captures(&*color).unwrap();
            let mut components = components.iter();
            components.next();
            let components = components
                .map(|c| c.unwrap().as_str().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            Some(HSLColor::from_rgba(
                components[0],
                components[1],
                components[2],
                components[3],
            ))
        } else if color.chars().next() == Some('#') {
            let mut c = Vec::new();

            let mut r = color.chars().skip(1).collect::<String>();
            let contains_alpha = if r.len() == 6 { false } else { true };
            let mut g = r.split_off(2);
            let mut b = g.split_off(2);

            c.push(u8::from_str_radix(&*r, 16).unwrap());
            c.push(u8::from_str_radix(&*g, 16).unwrap());

            if contains_alpha {
                let a = b.split_off(2);
                c.push(u8::from_str_radix(&*b, 16).unwrap());
                c.push(u8::from_str_radix(&*a, 16).unwrap());
            }
            c.push(u8::from_str_radix(&*b, 16).unwrap());
            c.push(u8::MAX);

            Some(HSLColor::from_rgba(c[0], c[1], c[2], c[3]))
        } else {
            None
        }
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
pub fn ColorPicker(
    color: Signal<HSLColor>,
    on_color_change: Option<Callback<HSLColor>>,
) -> Element {
    // TODO: set default as original element color somehow
    let config = use_context::<AppConfiguration>();
    let mut editing_style = config.element_style;
    let mut selected_color = color;

    let mut saturation_lightness_select_rect = use_signal(|| (0.0, 0.0));
    let mut cursor_pos: Signal<Point2D<f64, ElementSpace>> = use_signal(|| Point2D::origin());
    let mut history = config.color_history;
    let mut color_switch = config.color_switch;

    let refresh_rate = time::Duration::from_secs_f64(1.0 / 60.0);
    let refresh_rate_slow = time::Duration::from_secs_f64(1.0 / 10.0);
    let mut last_time = use_signal(|| Instant::now());
    let mut last_time_slow = use_signal(|| Instant::now());

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
            // BUG: this happens 1 more time than it should sometimes
            let mut writelock = history.write();
            let index = writelock.len() - 1;
            writelock[index] = *selected_color.peek();
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

                            let relative_coord = event.element_coordinates();
                            *cursor_pos.write() = relative_coord.clone();

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
                            debug!("Set selected color to {} by history", color.as_css_property());
                            selected_color.set(color.clone());
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
