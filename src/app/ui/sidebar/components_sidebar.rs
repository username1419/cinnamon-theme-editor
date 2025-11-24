use adw::glib;
use adw::prelude::WidgetExt;
use adw::subclass::prelude::{ObjectImpl, WidgetImpl};
use adw::subclass::prelude::{ObjectSubclass, ObjectSubclassIsExt};
use gtk::{Label, ListBox};
use log::*;

use crate::app::io::parse::StyleSheet;
use crate::app::io::parser::selector::SelectorCategory;

use super::components_sidebar_item::ComponentSidebarItem;

mod imp {
    use std::cell::RefCell;

    use adw::subclass::bin::BinImpl;

    use super::*;

    #[derive(Debug, Default)]
    pub struct ComponentSideBar {
        pub list_box: RefCell<ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComponentSideBar {
        const NAME: &'static str = "ComponentSideBar";
        type Type = super::ComponentSideBar;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for ComponentSideBar {}
    impl WidgetImpl for ComponentSideBar {}
    impl BinImpl for ComponentSideBar {}
}

glib::wrapper! {
    pub struct ComponentSideBar(ObjectSubclass<imp::ComponentSideBar>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl ComponentSideBar {
    pub fn new() -> Self {
        trace!("Creating ComponentSideBar");

        // TODO: im sure theres a better way to do this but im stupid
        let list_box = ListBox::builder().visible(true).build();
        let this = glib::Object::builder::<Self>()
            .property("child", list_box.clone())
            .build();

        this.imp().list_box.replace(list_box);
        this
    }

    pub fn clear(&self) {
        let imp = self.imp();

        while let Some(child) = imp.list_box.borrow_mut().first_child() {
            imp.list_box.borrow_mut().remove(&child);
        }
    }

    pub fn populate(&self, file: StyleSheet) {
        let imp = self.imp();
        let listbox = imp.list_box.borrow_mut();

        SelectorCategory::VALUES.into_iter().for_each(|category| {
            listbox.append(&ComponentSidebarItem::new(category));
        });
        // TODO: tabs or smth like that i actually havent thought about it LMAO

        //components.iter().for_each(|component| {
        //    imp.list_box
        //        .borrow_mut()
        //        .append(&Label::builder().label(component).build())
        //});
    }
}
