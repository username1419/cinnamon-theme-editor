use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    #[derive(Default)]
    pub struct StBoxLayout {}

    #[glib::object_subclass]
    impl ObjectSubclass for StBoxLayout {
        const NAME: &'static str = "StBoxLayout";
        type Type = super::StBoxLayout;
        // closest equivalent is gtk::BoxLayout, which is not subclassible
        type ParentType = adw::Bin;
    }

    impl BinImpl for StBoxLayout {}
    impl ObjectImpl for StBoxLayout {}
    impl WidgetImpl for StBoxLayout {}
}

glib::wrapper! {
    pub struct StBoxLayout(ObjectSubclass<imp::StBoxLayout>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl StBoxLayout {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StBoxLayout>()
            .property("css-name", "StBoxLayout")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
