use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{button::ButtonImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct StButton {}

    #[glib::object_subclass]
    impl ObjectSubclass for StButton {
        const NAME: &'static str = "StButton";
        type Type = super::StButton;
        type ParentType = gtk::Button;
    }

    impl ButtonImpl for StButton {}
    impl ObjectImpl for StButton {}
    impl WidgetImpl for StButton {}
}

glib::wrapper! {
    pub struct StButton(ObjectSubclass<imp::StButton>)
        @extends gtk::Widget, gtk::Button,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible, gtk::Actionable;
}

impl StButton {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StButton>()
            .property("css-name", "StButton")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
