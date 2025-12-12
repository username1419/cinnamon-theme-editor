use adw::{
    PreferencesGroup, PreferencesPage, PreferencesRow,
    prelude::{PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
};
use gtk::{
    Label, ListBoxRow, glib,
    pango::{AttrList, Attribute},
    prelude::WidgetExt,
};

use crate::app::dconf::{self, CinnamonSettings};

mod imp {

    use adw::subclass::{
        dialog::AdwDialogImpl,
        preferences_dialog::PreferencesDialogImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct InspectorSettingsDialog {}

    #[glib::object_subclass]
    impl ObjectSubclass for InspectorSettingsDialog {
        const NAME: &'static str = "InspectorSettingsDialog";
        type Type = super::InspectorSettingsDialog;
        type ParentType = adw::PreferencesDialog;
    }

    impl AdwDialogImpl for InspectorSettingsDialog {}
    impl PreferencesDialogImpl for InspectorSettingsDialog {}
    impl ObjectImpl for InspectorSettingsDialog {}
    impl WidgetImpl for InspectorSettingsDialog {}
}

glib::wrapper! {
    pub struct InspectorSettingsDialog(ObjectSubclass<imp::InspectorSettingsDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl InspectorSettingsDialog {
    pub fn initialize() -> Self {
        let this = glib::Object::builder::<InspectorSettingsDialog>().build();

        this.add(&Self::init_internal_settings());
        this.add(&Self::init_external_settings());

        this
    }

    /// Settings page for internal inspector configurations, eg. inspector background, displayed
    /// panels, etc.
    fn init_internal_settings() -> PreferencesPage {
        let page = PreferencesPage::builder().title("Inspector").build();

        let general = PreferencesGroup::builder().title("General").build();
        page.add(&general);

        let panel = PreferencesGroup::builder().title("Panel").build();
        page.add(&panel);

        page
    }

    /// Settings page for external configurations, preferrably from dconf. Read-only.
    fn init_external_settings() -> PreferencesPage {
        let page = PreferencesPage::builder().title("System").build();

        let general = PreferencesGroup::builder().title("General").build();
        page.add(&general);

        let panel = PreferencesGroup::builder().title("Panel").build();
        page.add(&panel);
        let panels = CinnamonSettings::get_enabled_panels().ok();

        page
    }
}
