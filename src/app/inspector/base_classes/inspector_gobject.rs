use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{box_::BoxImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct GObject {}

    #[glib::object_subclass]
    impl ObjectSubclass for GObject {
        const NAME: &'static str = "GObject";
        type Type = super::GObject;
        type ParentType = gtk::Box;
    }

    impl BoxImpl for GObject {}
    impl ObjectImpl for GObject {}
    impl WidgetImpl for GObject {}
}

glib::wrapper! {
    pub struct GObject(ObjectSubclass<imp::GObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl GObject {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<GObject>()
            .property("css-name", "GObject")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
