use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_config::color_picker::{ColorPicker, HSLColor};
use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::{Value, ValueUnit};
use crate::app::io::parser::selector::Selector;
use crate::config::AppConfiguration;

#[component]
pub fn StyleInput() -> Element {
    let config = use_context::<AppConfiguration>();
    let default_style = config.default_style;
    let element_style = config.element_style;
    let editable_properties = [
        // https://jonathan.bergknoff.com/journal/modifying-theme-colors-linux-mint-cinnamon/
        ("background-color", 0, vec![]),
        (
            "border-width",
            4,
            vec![
                "border-bottom-width",
                "border-left-width",
                "border-right-width",
                "border-top-width",
            ],
        ),
        (
            "border-radius",
            4,
            vec![
                "border-top-left-radius",
                "border-top-right-radius",
                "border-bottom-right-radius",
                "border-bottom-left-radius",
            ],
        ),
        (
            "margin",
            4,
            vec!["margin-top", "margin-right", "margin-bottom", "margin-left"],
        ),
        (
            "padding",
            4,
            vec![
                "padding-top",
                "padding-right",
                "padding-bottom",
                "padding-left",
            ],
        ),
        // NOTE: i aint doin box-shadow
    ];

    rsx! {
        div {
            class: "style-input",
        }
    }
}

#[component]
// NOTE: this looks like ass so im not using it anymore
fn BackgroundColorInput() -> Element {
    let config = use_context::<AppConfiguration>();
    let default_style = config.default_style;
    let mut element_style = config.element_style;
    let mut element_color = use_signal(|| {
        if let Some(declaration) = element_style.peek().find_attribute("background-color") {
            let color = declaration.value[0].get_value();
            if let Some(col) = HSLColor::from_css_property(color.clone()) {
                return col;
            }
        }
        let style = &*default_style.peek();
        let category_style = style.get(&*config.inspector_type.peek()).unwrap();
        let selected_elements = &*config.selected_elements.peek();
        let style;
        if selected_elements.len() > 1 {
            // TODO: yea you heard 'im
            warn!("hey this isnt implemented yet");
            return HSLColor::default();
        } else {
            let element_name = selected_elements
                .iter()
                .next()
                .unwrap()
                .get_last()
                .expect("Selector is not supposed to be empty.")
                .0;
            style = category_style.get_declaration(&Selector::from_raw(&element_name.get_raw()));
        }

        if let Some(style) = style {
            let attribute = style.find_attribute("background-color");
            if let Some(attribute) = attribute {
                if let Some(col) = HSLColor::from_css_property(attribute.to_string()) {
                    return col;
                }
            }
        }
        HSLColor::default()
    });

    let mut popup_opened = use_signal(|| false);
    let mut color_change_link = use_signal(|| {
        let element_color = *element_color.peek();
        vec![
            Value::from_raw_single(&element_color.get_hue().to_string()),
            Value::from_raw_single(&element_color.get_saturation().to_string()),
            Value::from_raw_single(&element_color.get_lightness().to_string()),
            Value::from_raw_single(&element_color.get_alpha().to_string()),
        ]
    });

    use_effect(move || {
        let element_color = element_color();
        let values = Value::from_raw(&element_color.as_css_property());
        element_style
            .write()
            .set_style_attribute(Property::from_raw("background-color"), values);

        color_change_link.with_mut(|vec| {
            vec.iter_mut().enumerate().for_each(|(idx, val)| {
                let component = match idx {
                    0 => element_color.get_hue(),
                    1 => element_color.get_saturation(),
                    2 => element_color.get_lightness(),
                    3 => element_color.get_alpha(),
                    _ => panic!("idx out of range"),
                }
                .to_string();

                if component.ne(val.get_value()) {
                    val.set_value(component);
                }
            });
        });
    });

    rsx! {
        div {
            class: "property-input background-color-input",
            div {
                span { "background-color" }
                button {
                    class: "background-color-edit-button",
                    onclick: move |_| {
                        *popup_opened.write() = true;
                    },
                    div {
                        class: "background-color-display",
                        // WARN: hey uhhhh this might possibly have default values when it cant find
                        // styles so look out for that
                        style: "background-color: {element_color().as_css_property()}",
                    }
                    div {
                        class: "popup background-color-select",
                        style: if !popup_opened() { "display: none" },
                        onpointerleave: move |_| {
                            *popup_opened.write() = false;
                        },
                        ColorPicker {
                            color: element_color,
                        }
                    }
                }
            }
            div {
                class: "property-input-base",
                TextField {
                    // NOTE: this is for hsla
                    connected_values: vec![TextType::Number(None, ValueUnit::None, false); 4],
                    oninput: move |(vec, modified): (Vec<Value>, usize)| -> Vec<Value> {
                        element_color.with_mut(|element_color| {
                            let component_value = vec[modified].get_value().parse().unwrap();
                            match modified {
                                0 => element_color.set_hue(component_value),
                                1 => element_color.set_saturation(component_value),
                                2 => element_color.set_lightness(component_value),
                                3 => element_color.set_alpha(component_value),
                                _ => panic!("modified index not in range of vec"),
                            }
                        });
                        vec
                    },
                    change_signal: color_change_link,
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum TextType {
    /// label, the default unit used, should the unit be modified
    Number(Option<String>, ValueUnit, bool),
    Text(Option<String>),
}

/// hey btw the oninput function results are fed back into the TextField
#[component]
fn TextField(
    connected_values: Vec<TextType>,
    oninput: Callback<(Vec<Value>, usize), Vec<Value>>,
    change_signal: Signal<Vec<Value>>,
) -> Element {
    let mut linked_value = use_signal(|| {
        connected_values
            .iter()
            .map(|t| {
                if let TextType::Number(_, unit, _) = t {
                    Value::from_raw_single(&format!("0{}", unit.to_string()))
                } else {
                    Value::from_raw_single("")
                }
            })
            .collect::<Vec<Value>>()
    });

    let discard_nondigit =
        |str: String| -> String { str.chars().filter(|c| c.is_digit(10)).collect() };

    use_effect(move || {
        let components = change_signal();
        linked_value.set(components);
    });

    let get_label = |connected_val: TextType| match connected_val {
        TextType::Number(label, _, _) => label,
        TextType::Text(label) => label,
    };

    rsx! {
        for (index, use_type) in connected_values.into_iter().enumerate() {
            div {
                class: "value-input",
                span { "{get_label(use_type.clone()).unwrap_or_default()}" }
                input {
                    class: "value-input-field",
                    value: "{linked_value()[index].get_value()}",
                    oninput: move |evt| {
                        let value = evt.value();
                        let mut vec = linked_value.peek().cloned();
                        vec[index].set_value(
                            match &use_type {
                                TextType::Number(label, _, _) => {
                                    let v = discard_nondigit(value);
                                    debug!("value with label {:?} changed via inputfield to {}", label, v);
                                    v
                                },
                                TextType::Text(label) => {
                                    let v = value;
                                    debug!("value with label {:?} changed via inputfield to {}", label, v);
                                    v
                                },
                            }
                        );
                        *linked_value.write() = oninput((vec, index))
                    }
                }
                if let TextType::Number(_, _, is_modifiable) = &use_type {
                    select {
                        value: "{linked_value()[index].get_unit()}",
                        disabled: !is_modifiable,
                        onchange: move |evt| {
                            linked_value.write()[index].set_unit(ValueUnit::from_str(evt.value().as_str()));
                        },
                        for unit in ValueUnit::PATTERNS {
                            option {
                                value: "{unit.0}",
                                "{unit.0}"
                            }
                        }
                    }
                }
            }
        }
    }
}
