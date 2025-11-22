use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Label, glib};
use log::trace;

mod imp {
    use std::cell::RefCell;

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::{Label, subclass::widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct ComponentSidebarItem {
        pub label: RefCell<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComponentSidebarItem {
        const NAME: &'static str = "ComponentSidebarItem";
        type Type = super::ComponentSidebarItem;
        type ParentType = adw::Bin;
    }

    impl BinImpl for ComponentSidebarItem {}
    impl ObjectImpl for ComponentSidebarItem {}
    impl WidgetImpl for ComponentSidebarItem {}
}

glib::wrapper! {
    pub struct ComponentSidebarItem(ObjectSubclass<imp::ComponentSidebarItem>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl ComponentSidebarItem {
    pub fn new(label: Label) -> Self {
        trace!("Creating ComponentSidebarItem with label {}", label.label());

        let this = glib::Object::builder::<Self>()
            .property("child", label.clone())
            .build();

        this.imp().label.replace(label);
        this
    }
}
