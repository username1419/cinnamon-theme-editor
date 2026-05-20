use crate::{
    app::io::parser::{declaration_block::DeclarationBlock, selector::Selector},
    config::AppConfiguration,
};
use dioxus::{
    core::consume_context,
    html::{ModifiersInteraction, MouseEvent},
    prelude::{debug, warn},
    signals::{Memo, ReadableExt, Signal, WritableExt},
};

pub struct InspectorUtil;
impl InspectorUtil {
    pub fn inspector_component_onclick(
        evt: MouseEvent,
        selected: Signal<bool>,
        ancestry_attr: Selector,
        mut is_multi_select: Signal<bool>,
        mut this_selection_group: Signal<u32>,
    ) {
        let config = consume_context::<AppConfiguration>();
        let mut num_selected = config.num_element_selected;
        let mut selection_group = config.selection_group;
        let is_ctrl = evt.modifiers().ctrl();
        *is_multi_select.write() = is_ctrl;

        // Selecting a new element
        if !is_ctrl {
            // Single-select mode: start a new selection session
            // This will trigger use_effect in other components to deselect
            let new_group = selection_group() + 1;
            *selection_group.write() = new_group;
            if selected.peek().eq(&false) {
                *this_selection_group.write() = new_group;
                *num_selected.write() = 1;
            }
        } else {
            // Multi-select mode: join current session
            *this_selection_group.write() = selection_group();
            *num_selected.write() += 1;
        }

        debug!("{} clicked", ancestry_attr.to_string());
        evt.stop_propagation();
    }

    pub fn inspector_component_select_effect(
        mut is_style_override: Signal<bool>,
        mut this_style: Signal<DeclarationBlock>,
        mut selected: Signal<bool>,
        this_selection_group: Signal<u32>,
        ancestry_attr: Selector,
        dyn_style: Memo<DeclarationBlock>,
    ) {
        let config = consume_context::<AppConfiguration>();

        if !*config.is_editing.peek() {
            return;
        }

        let current_group = config.selection_group;
        let mut editing_stylesheet = config.editing_stylesheet;
        let mut editing_style = config.element_style;
        let mut color_switch = config.color_switch;
        let mut is_dirty = config.is_dirty;
        let mut selected_elements = config.selected_elements;
        let current_group = current_group();
        let this_group = this_selection_group.peek();
        let is_selected = selected();

        let mut selected_elements_wl = selected_elements.write();

        // If there's a new selection session and we're not part of it, deselect
        if is_selected && current_group > 0 && this_group.ne(&current_group) {
            debug!("Deselected element {}", ancestry_attr.to_string());
            selected.set(false);
            *color_switch.write() = true;
            *this_style.write() = editing_style.peek().clone();
            *is_style_override.write() = true;
            editing_stylesheet
                .write()
                .get_mut(&*config.inspector_type.peek())
                .unwrap()
                .append_rule(ancestry_attr.clone(), dyn_style.peek().clone());
            debug!(
                "Set element {} style as {}",
                ancestry_attr.to_string(),
                dyn_style.peek().to_string()
            );
            if selected_elements_wl.remove(&ancestry_attr) {
                debug!("Removed element {} from selected_elements", ancestry_attr);
            } else {
                warn!(
                    "Element {} not found in selected_elements, {:?}. Something went wrong.",
                    ancestry_attr, *selected_elements_wl
                );
            }
            if is_dirty.peek().eq(&false) {
                *is_dirty.write() = true;
            }
        }

        if is_style_override.peek().eq(&true) && current_group > 0 && this_group.eq(&current_group)
        {
            selected.set(true);
            *editing_style.write() = this_style.peek().cloned();
            *is_style_override.write() = false;
            selected_elements_wl.insert(ancestry_attr.clone());
            debug!("Added element {} to selected_elements", ancestry_attr);
        }
    }
}
