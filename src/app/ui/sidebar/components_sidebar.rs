use adw::glib;
use adw::subclass::prelude::{ObjectImpl, WidgetImpl};
use adw::subclass::prelude::{ObjectSubclass, ObjectSubclassIsExt};
use gtk::prelude::WidgetExt;
use gtk::{Label, ListBox, Orientation};
use log::*;

use crate::app::io::parse::StyleSheet;

mod imp {
    use std::cell::RefCell;

    use gtk::subclass::box_::BoxImpl;

    use super::*;

    #[derive(Debug, Default)]
    pub struct ComponentSideBar {
        pub list_box: RefCell<ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComponentSideBar {
        const NAME: &'static str = "ComponentSideBar";
        type Type = super::ComponentSideBar;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for ComponentSideBar {}
    impl WidgetImpl for ComponentSideBar {}
    impl BoxImpl for ComponentSideBar {}
}

glib::wrapper! {
    pub struct ComponentSideBar(ObjectSubclass<imp::ComponentSideBar>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl ComponentSideBar {
    pub fn new() -> Self {
        trace!("Creating ComponentSideBar");
        glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .build()
    }

    pub fn clear(&self) {
        let imp = self.imp();

        while let Some(child) = imp.list_box.borrow_mut().first_child() {
            imp.list_box.borrow_mut().remove(&child);
        }
    }

    pub fn populate(&self, file: StyleSheet) {
        let imp = self.imp();

        todo!();

        //components.iter().for_each(|component| {
        //    imp.list_box
        //        .borrow_mut()
        //        .append(&Label::builder().label(component).build())
        //});
    }
}
