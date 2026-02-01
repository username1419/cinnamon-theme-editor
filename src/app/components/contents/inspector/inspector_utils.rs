use crate::{
    app::io::parser::{declaration_block::DeclarationBlock, selector::Selector},
    config::AppConfiguration,
};
use dioxus::{
    core::consume_context,
    html::{ModifiersInteraction, MouseEvent},
    prelude::debug,
    signals::{Memo, ReadableExt, Signal, WritableExt},
};

pub struct InspectorUtil;
impl InspectorUtil {
    pub fn inspector_component_onclick(
        evt: MouseEvent,
        mut selected: Signal<bool>,
        mut is_style_override: Signal<bool>,
        ancestry_attr: Selector,
        mut component_style: Signal<DeclarationBlock>,
        dyn_style: Memo<DeclarationBlock>,
        mut is_multi_select: Signal<bool>,
        mut this_selection_group: Signal<u32>,
    ) {
        let config = consume_context::<AppConfiguration>();
        let mut is_dirty = config.is_dirty;
        let mut editing_stylesheet = config.editing_stylesheet;
        let mut editing_style = config.element_style;
        let mut num_selected = config.num_element_selected;
        let mut selection_group = config.selection_group;
        let is_ctrl = evt.modifiers().ctrl();
        *is_multi_select.write() = is_ctrl;

        if selected() {
            // Clicking on already selected element - deselect it
            *selected.write() = false;
            *num_selected.write() = num_selected().saturating_sub(1);
            *component_style.write() = dyn_style.read().clone();
            editing_stylesheet
                .write()
                .append_rule(ancestry_attr.clone(), dyn_style().clone());
            if !is_dirty() {
                *is_dirty.write() = true;
            }
            return;
        }

        // Selecting a new element
        if !is_ctrl {
            // Single-select mode: start a new selection session
            // This will trigger use_effect in other components to deselect
            let new_group = selection_group() + 1;
            *selection_group.write() = new_group;
            *this_selection_group.write() = new_group;
            *num_selected.write() = 1;
        } else {
            // Multi-select mode: join current session
            *this_selection_group.write() = selection_group();
            *num_selected.write() += 1;
        }

        if is_style_override() {
            *editing_style.write() = dyn_style.read().cloned();
            *is_style_override.write() = false;
        }

        debug!("{} clicked", ancestry_attr.to_string());
        *selected.write() = true;
        evt.stop_propagation();
    }

    pub fn inspector_component_select_effect(
        mut is_style_override: Signal<bool>,
        mut this_style: Signal<DeclarationBlock>,
        editing_style: DeclarationBlock,
        mut selected: Signal<bool>,
        this_selection_group: Signal<u32>,
        ancestry_attr: Selector,
        dyn_style: Memo<DeclarationBlock>,
    ) {
        let config = consume_context::<AppConfiguration>();
        let current_group = config.selection_group;
        let mut editing_stylesheet = config.editing_stylesheet;
        let mut is_dirty = config.is_dirty;
        let session = current_group();
        let my_session = this_selection_group();
        let is_selected = selected();

        // If there's a new selection session and we're not part of it, deselect
        if is_selected && session > 0 && session != my_session {
            *selected.write() = false;
            *this_style.write() = editing_style.clone();
            *is_style_override.write() = false;
            editing_stylesheet
                .write()
                .append_rule(ancestry_attr.clone(), dyn_style().clone());
            if !is_dirty() {
                *is_dirty.write() = true;
            }
        }
    }
}
