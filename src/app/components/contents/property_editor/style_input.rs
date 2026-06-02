use dioxus::prelude::*;
use dioxus::{
    core::Element,
    prelude::{component, rsx},
};

use crate::app::components::contents::property_editor::color::HSLColor;
use crate::app::components::contents::property_editor::color_picker::ColorPicker;
use crate::app::components::contents::property_editor::editor_section::EditorSection;
use crate::app::components::contents::property_editor::property_conf_utils::find_element_attribute;
use crate::app::io::parser::declaration_block::DeclarationBlock;
use crate::app::io::parser::property::Property;
use crate::app::io::parser::property_value::{Value, ValueUnit};
use crate::app::io::parser::selector::Selector;
use crate::config::{AppConfiguration, PropertyConfiguration};

fn load_property_values(properties: &[&str]) -> Vec<Value> {
    properties
        .iter()
        .map(|prop| {
            find_element_attribute(prop)
                .into_iter()
                .next()
                .unwrap_or_else(|| Value::from_raw_single("0px"))
        })
        .collect()
}

fn append_properties(element_style: &mut DeclarationBlock, properties: &[&str], values: &[Value]) {
    for (prop, val) in properties.iter().zip(values.iter()) {
        element_style.append(DeclarationBlock::from_raw(format!("{prop}: {val}")));
    }
}

fn number_fields(labels: &[&str]) -> Vec<TextType> {
    labels
        .iter()
        .map(|label| TextType::Number(Some(label.to_string()), ValueUnit::Px, true))
        .collect()
}

fn load_border_side_values(property: &str) -> Vec<Value> {
    let mut values = find_element_attribute(property);
    if values.is_empty() {
        return vec![
            Value::from_raw_single("0px"),
            Value::from_raw_single("none"),
            Value::from_raw_single("transparent"),
        ];
    }
    while values.len() < 3 {
        values.push(Value::from_raw_single(""));
    }
    values.truncate(3);
    values
}

#[component]
pub fn StyleInput() -> Element {
    let config = use_context::<AppConfiguration>();
    #[allow(unused)]
    let default_style = config.default_style;
    let mut element_style = config.element_style;

    let property_config = use_context::<PropertyConfiguration>();
    let current_color = property_config.current_bg_color;
    let selected = config.selected_elements;

    rsx! {
        div {
            class: "style-input",
            EditorSection {
                class: "background-color-input".to_string(),
                label: "Background".to_string(),
                ColorPicker {
                    color: current_color,
                    on_color_change: move |col: HSLColor| {
                        let mut wl = element_style.write();
                        wl.append(DeclarationBlock::from_raw(format!("background-color: {}", col.as_css_property())));
                    }
                }
                StylePropertyTextField {
                    properties: &["background-color"],
                    labels: &[],
                    element_style,
                    selected,
                    use_text: true,
                }
            }
            EditorSection {
                class: "border-input".to_string(),
                label: "Border".to_string(),
                BorderSideTextField {
                    property: "border-top",
                    side_label: "top",
                    element_style,
                    selected,
                }
                BorderSideTextField {
                    property: "border-right",
                    side_label: "right",
                    element_style,
                    selected,
                }
                BorderSideTextField {
                    property: "border-bottom",
                    side_label: "bottom",
                    element_style,
                    selected,
                }
                BorderSideTextField {
                    property: "border-left",
                    side_label: "left",
                    element_style,
                    selected,
                }
                StylePropertyTextField {
                    properties: &[
                        "border-top-left-radius",
                        "border-top-right-radius",
                        "border-bottom-right-radius",
                        "border-bottom-left-radius",
                    ],
                    labels: &["TL", "TR", "BR", "BL"],
                    element_style,
                    selected,
                    use_text: false,
                }
            }
            EditorSection {
                class: "margin-input".to_string(),
                label: "Margin".to_string(),
                StylePropertyTextField {
                    properties: &["margin-top", "margin-right", "margin-bottom", "margin-left"],
                    labels: &["top", "right", "bottom", "left"],
                    element_style,
                    selected,
                    use_text: false,
                }
            }
            EditorSection {
                class: "padding-input".to_string(),
                label: "Padding".to_string(),
                StylePropertyTextField {
                    properties: &["padding-top", "padding-right", "padding-bottom", "padding-left"],
                    labels: &["top", "right", "bottom", "left"],
                    element_style,
                    selected,
                    use_text: false,
                }
            }
        }
    }
}

#[component]
fn BorderSideTextField(
    property: &'static str,
    side_label: &'static str,
    element_style: Signal<DeclarationBlock>,
    selected: SyncSignal<std::collections::HashSet<Selector>>,
) -> Element {
    let mut change_signal = use_signal(|| load_border_side_values(property));

    use_effect(move || {
        let _ = selected.read();
        change_signal.set(load_border_side_values(property));
    });

    let connected_values = vec![
        TextType::Number(Some(String::from("width")), ValueUnit::Px, true),
        TextType::Text(Some(String::from("type"))),
        TextType::Text(Some(String::from("color"))),
    ];

    rsx! {
        div {
            class: "property-input-base border-side-input",
            div {
                class: "property-input border-side-input",
                TextField {
                    connected_values,
                    change_signal,
                    oninput: move |(vec, _modified): (Vec<Value>, usize)| {
                        let css_value = vec
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        element_style.write().append(DeclarationBlock::from_raw(format!(
                            "{property}: {css_value}"
                        )));
                        vec
                    },
                }
            }
        }
    }
}

#[component]
fn StylePropertyTextField(
    properties: &'static [&'static str],
    labels: &'static [&'static str],
    element_style: Signal<DeclarationBlock>,
    selected: SyncSignal<std::collections::HashSet<Selector>>,
    use_text: bool,
) -> Element {
    let mut change_signal = use_signal(|| load_property_values(properties));

    use_effect(move || {
        let _ = selected.read();
        change_signal.set(load_property_values(properties));
    });

    let connected_values = if use_text {
        vec![TextType::Text(None)]
    } else {
        number_fields(labels)
    };

    rsx! {
        div {
            class: "property-input-base",
            TextField {
                connected_values,
                change_signal,
                oninput: move |(vec, _modified): (Vec<Value>, usize)| {
                    append_properties(&mut element_style.write(), properties, &vec);
                    vec
                },
            }
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
            let color = declaration.get_value()[0].get_value();
            if let Some(col) = HSLColor::from_css_property(color.clone()) {
                return col;
            }
        }
        let style = &*default_style.peek();
        let category_style = style.get(&*config.inspector_type.peek()).unwrap();
        let selected_elements = &*config.selected_elements.peek();
        if selected_elements.len() > 1 {
            // TODO: yea you heard 'im
            warn!("hey this isnt implemented yet");
            return HSLColor::default();
        }

        let element_name = selected_elements
            .iter()
            .next()
            .unwrap()
            .get_last()
            .expect("Selector is not supposed to be empty.")
            .0;
        let style = category_style.get_declaration(&Selector::from_raw(element_name.get_raw()));

        if let Some(style) = style {
            let attribute = style.find_attribute("background-color");
            if let Some(attribute) = attribute
                && let Some(col) = HSLColor::from_css_property(attribute.to_string())
            {
                return col;
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
#[allow(dead_code)]
enum TextType {
    /// label, the default unit used, should the unit be modified
    Number(Option<String>, ValueUnit, bool),
    Text(Option<String>),
}

/// hey btw the oninput function returned values are fed back into the TextField
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
                    Value::from_raw_single(&format!("0{}", unit))
                } else {
                    Value::from_raw_single("")
                }
            })
            .collect::<Vec<Value>>()
    });

    let discard_nondigit =
        |str: String| -> String { str.chars().filter(|c| c.is_ascii_digit()).collect() };

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
