use std::path::Path;

use adw::{
    Application, ApplicationWindow, MessageDialog, NavigationPage, builders::MessageDialogBuilder,
    prelude::NavigationPageExt,
};
use gtk::{
    FileChooserDialog, FileDialog, FileFilter, ResponseType,
    ffi::GtkFileDialog,
    gio::{
        File, SimpleAction,
        prelude::{ActionMapExt, FileExt},
    },
    glib::{MainContext, Variant, object::CastNone},
    prelude::{DialogExtManual, FileChooserExt, GtkWindowExt},
};
use log::{debug, trace};

use crate::app::{
    io::read,
    ui::{
        dialog::text_entry_dialog::TextEntryDialog, sidebar::components_sidebar::ComponentSideBar,
    },
};

struct ActionBuilder<'a> {
    name: &'a str,
    on_activate: Box<dyn Fn(&SimpleAction, Option<&Variant>)>,
}

impl<'a> ActionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            name: "",
            on_activate: Box::new(|_, _| todo!()),
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
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
        self.on_activate = Box::new(move |_, variant| {
            let variant = variant.cloned();
            MainContext::default().spawn_local(on_activate(variant));
        });
        self
    }

    pub fn build(self) -> SimpleAction {
        let action = SimpleAction::new(self.name, None);
        action.connect_activate(self.on_activate);

        action
    }

    pub fn add_to(self, app: &Application) {
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

    let window_rc_create_new = window.clone();
    ActionBuilder::new()
        .name("create-new")
        .on_activate_async(move |_| {
            let window_rc = window_rc_create_new.clone();
            async move {
                trace!("Creating theme chooser FileDialog...");
                let file_dialog = FileDialog::builder()
                    .title("Choose default theme")
                    .accept_label("Choose theme")
                    .initial_folder(&File::for_path("/usr/share/themes/"))
                    .build();

                let response = file_dialog.select_folder_future(Some(&window_rc)).await;
                if response.is_err() {
                    return;
                }
                let default_theme_path = response.unwrap().path().expect("Failed to unwrap path");
                debug!("Submit default fallback theme {:#?}", default_theme_path);
                let name =
                    TextEntryDialog::new(window_rc.as_ref(), "Enter new theme name", "Theme name")
                        .await
                        .unwrap();
                debug!("Submit new theme creation name with name: {}", name);
                if name.is_empty() {
                    return;
                }

                let file = read::create_as_edit(name.to_string(), default_theme_path);

                let binding = window_rc
                    .child()
                    .and_downcast::<adw::ToolbarView>()
                    .unwrap()
                    .content()
                    .and_downcast::<adw::NavigationSplitView>()
                    .unwrap()
                    .sidebar()
                    .and_downcast::<NavigationPage>()
                    .unwrap()
                    .child();
                let sidebar = binding.and_downcast_ref::<ComponentSideBar>().unwrap(); // what the fuck
                // this is only 4 nested layers

                sidebar.populate(file);
            }
        })
        .add_to(&app);
}
