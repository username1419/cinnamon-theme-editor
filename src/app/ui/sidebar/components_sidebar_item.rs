use std::str::FromStr;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Label, glib, prelude::WidgetExt};
use log::trace;

use crate::app::io::parser::selector::SelectorCategory;

mod imp {
    use std::cell::RefCell;

    use adw::subclass::{
        bin::BinImpl,
        prelude::{ObjectImpl, ObjectSubclass},
    };
    use gtk::subclass::widget::WidgetImpl;

    use super::*;

    pub struct ComponentSidebarItem {
        pub category: RefCell<SelectorCategory>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComponentSidebarItem {
        const NAME: &'static str = "ComponentSidebarItem";
        type Type = super::ComponentSidebarItem;
        type ParentType = adw::Bin;
    }

    impl Default for ComponentSidebarItem {
        fn default() -> Self {
            Self {
                category: RefCell::new(SelectorCategory::Other),
            }
        }
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
    pub fn new(category: SelectorCategory) -> Self {
        let label = Label::builder()
            .label(
                match category {
                    SelectorCategory::GroupWindow => String::from_str("Group Window").unwrap(),
                    c => format!("{:?}", c),
                }
                .as_str(),
            )
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Center)
            .build();
        trace!("Creating ComponentSidebarItem with label {}", label.label());

        let this = glib::Object::builder::<Self>()
            .property("child", label.clone())
            .build();

        this.add_css_class("sidebar-item");
        this.imp().category.replace(category);
        this
    }
}
