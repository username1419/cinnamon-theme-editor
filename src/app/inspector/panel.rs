use adw::prelude::BinExt;
use gtk::{Box, Overlay, glib, prelude::WidgetExt};

use super::{
    base_classes::inspector_cinnamongenericcontainer::CinnamonGenericContainer,
    inspector_page::InspectorContent,
};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct PanelPreview {}

    #[glib::object_subclass]
    impl ObjectSubclass for PanelPreview {
        const NAME: &'static str = "PanelPreview";
        type Type = super::PanelPreview;
        type ParentType = adw::Bin;
    }

    impl BinImpl for PanelPreview {}
    impl ObjectImpl for PanelPreview {}
    impl WidgetImpl for PanelPreview {}
}

glib::wrapper! {
    pub struct PanelPreview(ObjectSubclass<imp::PanelPreview>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl PanelPreview {}

impl InspectorContent for PanelPreview {
    fn initialize() -> Self {
        let this = glib::Object::builder::<PanelPreview>()
            .property("css-name", "PanelPreview")
            .build();

        let panel_left = CinnamonGenericContainer::builder()
            .with_valign(gtk::Align::Center)
            .with_halign(gtk::Align::Start)
            .with_css_classes(&["panel-left"])
            .build();
        let panel_bottom = CinnamonGenericContainer::builder()
            .with_valign(gtk::Align::End)
            .with_halign(gtk::Align::Center)
            .with_css_classes(&["panel-bottom"])
            .build();
        let panel_right = CinnamonGenericContainer::builder()
            .with_valign(gtk::Align::Center)
            .with_halign(gtk::Align::End)
            .with_css_classes(&["panel-right"])
            .build();
        let panel_top = CinnamonGenericContainer::builder()
            .with_valign(gtk::Align::Start)
            .with_halign(gtk::Align::Center)
            .with_css_classes(&["panel-top"])
            .build();

        let stage = Overlay::new();
        stage.add_overlay(&panel_left);
        stage.add_overlay(&panel_right);
        stage.add_overlay(&panel_top);
        stage.add_overlay(&panel_bottom);

        this.set_child(Some(&stage));
        this
    }

    fn refresh() -> () {
        todo!()
    }
}
