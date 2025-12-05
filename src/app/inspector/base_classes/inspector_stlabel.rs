use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct StLabel {}

    #[glib::object_subclass]
    impl ObjectSubclass for StLabel {
        const NAME: &'static str = "StLabel";
        type Type = super::StLabel;
        // closest equivalent is gtk::Label, which is not subclassible
        type ParentType = adw::Bin;
    }

    impl BinImpl for StLabel {}
    impl ObjectImpl for StLabel {}
    impl WidgetImpl for StLabel {}
}

glib::wrapper! {
    pub struct StLabel(ObjectSubclass<imp::StLabel>)
        @extends gtk::Widget, adw::Bin,
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
