use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct StBin {}

    #[glib::object_subclass]
    impl ObjectSubclass for StBin {
        const NAME: &'static str = "StBin";
        type Type = super::StBin;
        type ParentType = adw::Bin;
    }

    impl BinImpl for StBin {}
    impl ObjectImpl for StBin {}
    impl WidgetImpl for StBin {}
}

glib::wrapper! {
    pub struct StBin(ObjectSubclass<imp::StBin>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl StBin {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StBin>()
            .property("css-name", "StBin")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
