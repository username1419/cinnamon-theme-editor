use adw::{prelude::NavigationPageExt, subclass::prelude::ObjectSubclassIsExt};
use gtk::{
    CssProvider,
    glib::{self},
    prelude::{StyleContextExt, WidgetExt},
};
use log::{trace, warn};

use crate::app::io::{parse::StyleSheet, parser::selector::SelectorCategory};

use super::panel::PanelPreview;

pub trait InspectorContent {
    fn initialize() -> Self;
    fn refresh() -> ();
}

mod imp {

    use std::cell::RefCell;

    use adw::subclass::prelude::{NavigationPageImpl, ObjectImpl, ObjectSubclass};
    use gtk::subclass::widget::WidgetImpl;

    use crate::app::io::{parse::StyleSheet, parser::selector::SelectorCategory};

    use super::*;

    #[derive(Default)]
    pub struct Inspector {
        pub inspector_type: RefCell<Option<SelectorCategory>>,
        pub stylesheet: RefCell<StyleSheet>,
        pub current_css_provider: RefCell<CssProvider>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Inspector {
        const NAME: &'static str = "Inspector";
        type Type = super::Inspector;
        type ParentType = adw::NavigationPage;
    }

    impl NavigationPageImpl for Inspector {}
    impl ObjectImpl for Inspector {}
    impl WidgetImpl for Inspector {}
}

glib::wrapper! {
    /// Looking Glass-style inspector to observe and modify Cinnamon user interface style
    /// properties.
    pub struct Inspector(ObjectSubclass<imp::Inspector>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Accessible;
}

// TODO: mb we could have a right-click menu that enables the user to change the css properties of
// child widgets
// idk how much work that entails though
impl Inspector {
    pub fn new() -> Self {
        let this = glib::Object::builder::<Inspector>()
            .property("css-name", "Inspector")
            .build();

        let provider = this
            .imp()
            .current_css_provider
            .replace(this.imp().current_css_provider.take().clone());

        // WARN: deprecated
        // the only other way to do this that i can think of would be a massive pain in the ass so
        this.style_context().add_provider(&provider, 1);

        this
    }

    pub fn set_stylesheet(&mut self, stylesheet: StyleSheet) {
        self.imp()
            .current_css_provider
            .borrow()
            .load_from_path(stylesheet.get_source().clone());
        self.imp().stylesheet.replace(stylesheet);
        self.refresh_content();
    }

    pub fn set_category(&mut self, category: SelectorCategory) {
        let imp = self.imp();
        let repl = imp.inspector_type.replace(Some(category));
        trace!("Replaced Inspector type {:?} with {:?}", repl, category);

        self.rebuild_content();
    }

    pub fn rebuild_content(&mut self) {
        trace!(
            "Rebuild Inspector contents for {:?} Preview",
            self.imp().inspector_type.borrow().clone()
        );
        self.set_child(Some(&match self.imp().inspector_type.borrow().unwrap() {
            SelectorCategory::Panel => PanelPreview::initialize(),
            SelectorCategory::Menu => todo!(),
            SelectorCategory::Window => todo!(),
            SelectorCategory::Calendar => todo!(),
            SelectorCategory::Dialog => todo!(),
            SelectorCategory::Entry => todo!(),
            SelectorCategory::Sound => todo!(),
            SelectorCategory::GroupWindow => todo!(),
            SelectorCategory::Other => todo!(),
        }));
    }

    pub fn refresh_content(&mut self) {
        warn!("todo");
        return;

        if let Some(child) = self.child() {
            child;
        }
    }
}
