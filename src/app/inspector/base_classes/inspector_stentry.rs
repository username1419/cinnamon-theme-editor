use gtk::{glib, prelude::WidgetExt};

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{box_::BoxImpl, entry::EntryImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct StEntry {}

    #[glib::object_subclass]
    impl ObjectSubclass for StEntry {
        const NAME: &'static str = "StEntry";
        type Type = super::StEntry;
        type ParentType = gtk::Entry;
    }

    impl EntryImpl for StEntry {}
    impl ObjectImpl for StEntry {}
    impl WidgetImpl for StEntry {}
}

glib::wrapper! {
    pub struct StEntry(ObjectSubclass<imp::StEntry>)
        @extends gtk::Widget, gtk::Entry,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible, gtk::CellEditable, gtk::Editable;
    // WARN: CellEditable is decprecated, however is required for gtk::Entry
}

impl StEntry {
    pub fn new(with_css_classes: &[&str]) -> Self {
        let this = glib::Object::builder::<StEntry>()
            .property("css-name", "StEntry")
            .build();
        this.set_css_classes(with_css_classes);

        this
    }
}
