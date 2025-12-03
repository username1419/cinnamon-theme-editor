use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{box_::BoxImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct StLabel {}

    #[glib::object_subclass]
    impl ObjectSubclass for StLabel {
        const NAME: &'static str = "StLabel";
        type Type = super::StLabel;
        type ParentType = gtk::Box;
    }

    impl BoxImpl for StLabel {}
    impl ObjectImpl for StLabel {}
    impl WidgetImpl for StLabel {}
}

glib::wrapper! {
    pub struct StLabel(ObjectSubclass<imp::StLabel>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl StLabel {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StLabel>()
            .property("css-name", "StLabel")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
