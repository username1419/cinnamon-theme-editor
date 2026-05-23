use std::sync::Arc;

use crate::{
    app::io::parser::{declaration_block::DeclarationBlock, selector::Selector},
    config::AppConfiguration,
};
use dioxus::{
    core::consume_context,
    html::{ModifiersInteraction, MouseEvent},
    prelude::debug,
    signals::{ReadableExt, Signal, SyncSignal, WritableExt, WritableHashSetExt},
};
use tokio::sync::Notify;

pub struct InspectorUtil;
impl InspectorUtil {
    pub fn inspector_component_onclick(
        evt: MouseEvent,
        mut selected: SyncSignal<bool>,
        ancestry_attr: Selector,
    ) {
        let config = consume_context::<AppConfiguration>();
        let notifier = config.elements_notify.peek();
        let mut notify_confirm = config.elements_notify_confirm;
        let notifier_updated = config.elements_notify_updated.peek().clone();
        let mut num_selected = config.num_element_selected;
        let mut selected_elements = config.selected_elements;
        let element_style = config.element_style.peek().cloned();
        let mut editing_stylesheet = config.editing_stylesheet;
        let inspector_type = config.inspector_type.peek().cloned();

        let number_elements = *config.count_element.peek();
        let is_ctrl = evt.modifiers().ctrl();
        let is_selected = *selected.peek();

        if is_ctrl {
            debug!("control modifier is enabled");
            if is_selected {
                debug!("removing element from selected list");
                editing_stylesheet
                    .write()
                    .get_mut(&inspector_type)
                    .unwrap()
                    .append_rule(ancestry_attr.clone(), element_style);
                selected_elements.remove(&ancestry_attr);
                *num_selected.write() -= 1;
                selected.set(false);
            } else {
                debug!("adding element to selected list");
                selected_elements.insert(ancestry_attr.clone());
                *num_selected.write() += 1;
                selected.set(true);
            }
        } else {
            debug!("control modifier is disabled");
            let ancestry_attr = ancestry_attr.clone();
            debug!("notifying all other inspector elements");
            let confirm_notifier = Arc::new((Notify::new(), Notify::new()));
            notify_confirm.set(Some(confirm_notifier.clone()));
            notifier.notify_waiters();
            tokio::spawn(async move {
                for i in 0..number_elements {
                    confirm_notifier.0.notify_one();
                    confirm_notifier.1.notified().await;
                    debug!("confirm notification received: {}", i + 1);
                }
                debug!("all components have been notified. continuing...");
                notifier_updated.notify_waiters();

                if is_selected {
                    debug!("removing element from selected list");
                    editing_stylesheet
                        .write()
                        .get_mut(&inspector_type)
                        .unwrap()
                        .append_rule(ancestry_attr.clone(), element_style);
                    selected_elements.remove(&ancestry_attr);
                    *num_selected.write() -= 1;
                    selected.set(false);
                } else {
                    debug!("adding element to selected list");
                    selected_elements.insert(ancestry_attr);
                    *num_selected.write() += 1;
                    selected.set(true);
                }

                debug!("dropping confirm notifier");
                notify_confirm.set(None);
                drop(confirm_notifier);
            });
        }

        debug!("{} clicked", ancestry_attr.to_string());
        evt.stop_propagation();
    }

    pub async fn inspector_component_background_watcher(
        mut selected: SyncSignal<bool>,
        selector: Selector,
        mut current_style: Signal<DeclarationBlock>,
    ) -> () {
        let config = consume_context::<AppConfiguration>();

        let confirm = config.elements_notify_confirm;
        let notifier = config.elements_notify.peek().cloned();
        let mut num_selected = config.num_element_selected;
        let mut selected_elements = config.selected_elements;
        let element_style = config.element_style;
        let mut editing_stylesheet = config.editing_stylesheet;
        let inspector_type = config.inspector_type;
        let mut is_dirty = config.is_dirty;

        loop {
            debug!(
                "{}: waiting for notification from selected element",
                selector
            );
            notifier.notified().await;
            debug!("{}: received notification from selected element", selector);

            if *selected.peek() {
                debug!("removing selection from selected elements list");
                let style = element_style.peek().cloned();
                current_style.set(style.clone());
                editing_stylesheet
                    .write()
                    .get_mut(&inspector_type.peek().clone())
                    .unwrap()
                    .append_rule(selector.clone(), style);

                *num_selected.write() -= 1;
                selected_elements.remove(&selector);
                selected.set(false);

                if !*is_dirty.peek() {
                    is_dirty.set(true);
                }
            }

            if let Some(t) = confirm.peek().clone() {
                let (confirm, notify) = t.as_ref();
                debug!("notifying original");
                confirm.notified().await;
                notify.notify_last(); // WARN: sketchy
            }
        }
    }
}
