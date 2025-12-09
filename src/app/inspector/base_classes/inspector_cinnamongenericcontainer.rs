use gtk::{Align, glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct CinnamonGenericContainer {}

    #[glib::object_subclass]
    impl ObjectSubclass for CinnamonGenericContainer {
        const NAME: &'static str = "CinnamonGenericContainer";
        type Type = super::CinnamonGenericContainer;
        // closest equivalent is gtk::BoxLayout, which is not subclassible
        type ParentType = adw::Bin;
    }

    impl BinImpl for CinnamonGenericContainer {}
    impl ObjectImpl for CinnamonGenericContainer {}
    impl WidgetImpl for CinnamonGenericContainer {}
}

glib::wrapper! {
    pub struct CinnamonGenericContainer(ObjectSubclass<imp::CinnamonGenericContainer>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

struct CinnamonGenericContainerBuilder {
    valign: Align,
    halign: Align,
}

impl Default for CinnamonGenericContainerBuilder {
    fn default() -> Self {
        Self {
            valign: Align::Start,
            halign: Align::Start,
        }
    }
}

impl CinnamonGenericContainerBuilder {
    pub fn build(self) -> CinnamonGenericContainer {
        glib::Object::builder::<CinnamonGenericContainer>()
            .property("css-name", "CinnamonGenericContainer")
            .property("valign", self.valign)
            .property("halign", self.halign)
            .build()
    }

    pub fn with_valign(&mut self, valign: Align) {
        self.valign = valign;
    }

    pub fn with_halign(&mut self, halign: Align) {
        self.halign = halign;
    }
}

impl CinnamonGenericContainer {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<CinnamonGenericContainer>()
            .property("css-name", "CinnamonGenericContainer")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }

    pub fn builder() -> CinnamonGenericContainerBuilder {
        CinnamonGenericContainerBuilder::default()
    }
}
