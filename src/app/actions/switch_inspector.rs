use adw::{
    ApplicationWindow, NavigationSplitView, ToolbarView,
    prelude::{AdwApplicationWindowExt, NavigationPageExt},
};
use gtk::glib::{
    Variant,
    object::{Cast, CastNone},
};
use log::trace;

use crate::{
    app::{inspector::inspector_page::Inspector, io::parser::selector::SelectorCategory},
    helper::Helper,
};

pub fn switch_inspector(app_window: ApplicationWindow, variant: Option<&Variant>) {
    let toolbar_view = app_window.content().and_downcast::<ToolbarView>().unwrap();
    let navigationsplit_view = Helper::find_descendant(toolbar_view.upcast_ref(), "NavSplitView")
        .and_downcast::<NavigationSplitView>()
        .unwrap();
    let nav_page = navigationsplit_view.content().unwrap();

    let inspector = nav_page.downcast::<Inspector>();
    if variant.is_none() {
        return;
    }
    let mut inspector = inspector.unwrap();

    if let Some(value) = variant.unwrap().get() {
        let cat = match value {
            0 => SelectorCategory::Panel,
            1 => SelectorCategory::Menu,
            2 => SelectorCategory::Window,
            3 => SelectorCategory::Calendar,
            4 => SelectorCategory::Dialog,
            5 => SelectorCategory::Entry,
            6 => SelectorCategory::Sound,
            7 => SelectorCategory::GroupWindow,
            8 => SelectorCategory::Other,
            _ => panic!(
                "value is invalid range, expected {}, got {}",
                SelectorCategory::VALUES.len(),
                value
            ),
        };

        inspector.set_category(cat);
    }
}
