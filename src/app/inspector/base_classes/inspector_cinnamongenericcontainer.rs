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
    css_classes: Vec<String>,
}

impl Default for CinnamonGenericContainerBuilder {
    fn default() -> Self {
        Self {
            valign: Align::Start,
            halign: Align::Start,
            css_classes: Vec::default(),
        }
    }
}

impl CinnamonGenericContainerBuilder {
    pub fn build(self) -> CinnamonGenericContainer {
        let this = glib::Object::builder::<CinnamonGenericContainer>()
            .property("css-name", "CinnamonGenericContainer")
            .property("valign", self.valign)
            .property("halign", self.halign)
            .build();
        // NOTE: theres probably a better way to do this but im stupid
        self.css_classes
            .iter()
            .for_each(|class| this.add_css_class(&class));

        this
    }

    pub fn with_valign(mut self, valign: Align) -> Self {
        self.valign = valign;

        self
    }

    pub fn with_halign(mut self, halign: Align) -> Self {
        self.halign = halign;

        self
    }

    pub fn with_css_classes(mut self, css_classes: &[&str]) -> Self {
        self.css_classes = css_classes.into_iter().map(|s| s.to_string()).collect();

        self
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
