use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct StIcon {}

    #[glib::object_subclass]
    impl ObjectSubclass for StIcon {
        const NAME: &'static str = "StIcon";
        type Type = super::StIcon;
        // closest equivalent is gtk::Image, which is not subclassible
        type ParentType = adw::Bin;
    }

    impl BinImpl for StIcon {}
    impl ObjectImpl for StIcon {}
    impl WidgetImpl for StIcon {}
}

glib::wrapper! {
    pub struct StIcon(ObjectSubclass<imp::StIcon>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl StIcon {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StIcon>()
            .property("css-name", "StIcon")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
