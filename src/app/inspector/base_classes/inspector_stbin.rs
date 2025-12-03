use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{box_::BoxImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct StBin {}

    #[glib::object_subclass]
    impl ObjectSubclass for StBin {
        const NAME: &'static str = "StBin";
        type Type = super::StBin;
        // WARN: maybe adw::Bin?
        type ParentType = gtk::Box;
    }

    impl BoxImpl for StBin {}
    impl ObjectImpl for StBin {}
    impl WidgetImpl for StBin {}
}

glib::wrapper! {
    pub struct StBin(ObjectSubclass<imp::StBin>)
        @extends gtk::Widget, gtk::Box,
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
