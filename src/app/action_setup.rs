use adw::{Application, ApplicationWindow};
use gtk::{
    FileChooserDialog, ResponseType,
    gio::{SimpleAction, prelude::ActionMapExt},
    glib::{MainContext, Variant, VariantTy},
};
use log::trace;

use crate::app::actions::{create_new::create_new, switch_inspector::switch_inspector};

pub struct ActionBuilder<'a> {
    name: &'a str,
    variant_type: Option<&'a VariantTy>,
    on_activate: Box<dyn Fn(&SimpleAction, Option<&Variant>)>,
}

impl<'a> ActionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            name: "",
            variant_type: None,
            on_activate: Box::new(|_, _| todo!()),
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn with_variant(mut self, variant: &'a VariantTy) -> Self {
        self.variant_type = Some(variant);
        self
    }

    pub fn on_activate<F>(mut self, on_activate: F) -> Self
    where
        F: Fn(&SimpleAction, Option<&Variant>) + 'static,
    {
        self.on_activate = Box::new(on_activate);
        self
    }

    pub fn on_activate_async<F, Fut>(mut self, on_activate: F) -> Self
    where
        F: Fn(Option<Variant>) -> Fut + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        // NOTE: theres no `self` because of lifetime issues
        self.on_activate = Box::new(move |_, variant| {
            let variant = variant.cloned();
            MainContext::default().spawn_local(on_activate(variant));
        });
        self
    }

    pub fn build(self) -> SimpleAction {
        let action = SimpleAction::new(self.name, self.variant_type.map(|t| t.as_ref()));
        action.connect_activate(self.on_activate);

        action
    }

    pub fn add_to(self, app: &impl ActionMapExt) {
        app.add_action(&self.build());
    }
}

// TODO: mb an enum for names would be better in case i misspell or smth
pub fn setup_actions(app: Application, window: ApplicationWindow) {
    trace!("Setting up actions");

    let window_rc_open_exist = window.clone();
    ActionBuilder::new()
        .name("open-existing")
        .on_activate(move |_, _| {
            // WARN: deprecated ui element
            FileChooserDialog::new(
                Some("Open theme"),
                Some(&window_rc_open_exist),
                gtk::FileChooserAction::SelectFolder,
                &[("Cancel", ResponseType::Cancel)],
            );
            todo!();
        })
        .add_to(&app);

    let w_rc = window.clone();
    ActionBuilder::new()
        .name("create-new")
        .on_activate_async(move |v| create_new(w_rc.clone(), v))
        .add_to(&app);

    let w_rc = window.clone();
    ActionBuilder::new()
        .name("switch-inspector")
        .with_variant(&VariantTy::INT32)
        .on_activate(move |_, v| switch_inspector(w_rc.clone(), v))
        .add_to(&app);
}
