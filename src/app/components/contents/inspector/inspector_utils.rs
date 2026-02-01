use crate::{
    app::io::parser::{declaration_block::DeclarationBlock, selector::Selector},
    config::AppConfiguration,
};
use dioxus::{
    core::consume_context,
    html::MouseEvent,
    prelude::debug,
    signals::{Memo, ReadableExt, Signal, WritableExt},
};

pub struct InspectorUtil;
impl InspectorUtil {
    pub fn inspector_component_onclick(
        evt: MouseEvent,
        mut selected: Signal<bool>,
        mut use_original_style: Signal<bool>,
        mut is_style_override: Signal<bool>,
        ancestry_attr: Selector,
        mut component_style: Signal<DeclarationBlock>,
        dyn_style: Memo<DeclarationBlock>,
    ) {
        let config = consume_context::<AppConfiguration>();
        let mut editing_stylesheet = config.editing_stylesheet;
        let mut editing_style = config.element_style;
        let mut num_selected = config.element_selected;
        if selected() {
            // TODO: probably make the footer show selected components or something
            *selected.write() = false;
            *num_selected.write() -= 1;
            *component_style.write() = dyn_style.read().clone();
            *use_original_style.write() = true;
            editing_stylesheet
                .write()
                .append_rule(ancestry_attr.clone(), dyn_style().clone());
        }
        if is_style_override() {
            *editing_style.write() = dyn_style.read().cloned();
            *is_style_override.write() = false;
            *use_original_style.write() = false;
        }
        debug!("{} clicked", ancestry_attr.to_string());
        *selected.write() = true;
        *num_selected.write() += 1;
        evt.stop_propagation();
    }
}
