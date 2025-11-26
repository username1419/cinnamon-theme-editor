use std::cell::RefCell;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    Box, Widget,
    glib::{self, object::IsA},
    prelude::{BoxExt, WidgetExt},
};
use log::trace;

mod imp {

    use adw::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use gtk::subclass::{box_::BoxImpl, widget::WidgetImpl};

    use super::*;

    #[derive(Default)]
    pub struct FooterBar {
        pub start: RefCell<Box>,
        pub middle: RefCell<Box>,
        pub end: RefCell<Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FooterBar {
        const NAME: &'static str = "FooterBar";
        type Type = super::FooterBar;
        type ParentType = Box;
    }

    impl BoxImpl for FooterBar {}
    impl ObjectImpl for FooterBar {}
    impl WidgetImpl for FooterBar {}
}

glib::wrapper! {
    pub struct FooterBar(ObjectSubclass<imp::FooterBar>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

impl FooterBar {
    pub fn new() -> Self {
        let this = glib::Object::builder::<Self>()
            .property("css-name", "footer")
            .build();
        this.add_css_class("footer");
        let imp = this.imp().to_owned();
        imp.start
            .replace(Box::builder().hexpand(true).vexpand(true).build());
        this.append(&imp.start.borrow().clone());

        imp.middle.replace(
            Box::builder()
                .halign(gtk::Align::Center)
                .hexpand(true)
                .vexpand(true)
                .build(),
        );
        this.append(&imp.middle.borrow().clone());

        imp.end.replace(
            Box::builder()
                .halign(gtk::Align::End)
                .hexpand(true)
                .vexpand(true)
                .build(),
        );
        this.append(&imp.end.borrow().clone());

        this
    }

    pub fn pack_start(&mut self, widget: &impl IsA<Widget>) {
        let imp = self.imp();
        imp.start.borrow_mut().append(widget);
    }

    pub fn pack_middle(&mut self, widget: &impl IsA<Widget>) {
        let imp = self.imp();
        imp.middle.borrow_mut().append(widget);
    }

    pub fn pack_end(&mut self, widget: &impl IsA<Widget>) {
        let imp = self.imp();
        imp.end.borrow_mut().append(widget);
    }
}
