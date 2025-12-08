use gtk::{glib, prelude::WidgetExt};

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

impl CinnamonGenericContainer {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<CinnamonGenericContainer>()
            .property("css-name", "CinnamonGenericContainer")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
