use log::trace;

use adw::{
    ApplicationWindow, NavigationSplitView,
    prelude::{AdwApplicationWindowExt, NavigationPageExt},
};
use gtk::{
    FileDialog,
    gio::{File, prelude::FileExt},
    glib::{
        Variant,
        object::{Cast, CastNone},
    },
};
use log::debug;

use crate::{
    app::{
        io::read,
        ui::{
            dialog::text_entry_dialog::TextEntryDialog,
            sidebar::components_sidebar::ComponentSideBar,
        },
    },
    helper::Helper,
};

pub fn create_new(window: ApplicationWindow, v: Option<Variant>) -> impl Future<Output = ()> {
    async move {
        trace!("Creating theme chooser FileDialog...");
        let file_dialog = FileDialog::builder()
            .title("Choose default theme")
            .accept_label("Choose theme")
            .initial_folder(&File::for_path("/usr/share/themes/"))
            .build();

        let response = file_dialog.select_folder_future(Some(&window)).await;
        if response.is_err() {
            return;
        }
        let default_theme_path = response.unwrap().path().expect("Failed to unwrap path");
        debug!("Submit default fallback theme {:#?}", default_theme_path);
        let name = TextEntryDialog::new(window.as_ref(), "Enter new theme name", "Theme name")
            .await
            .unwrap();
        debug!("Submit new theme creation name with name: {}", name);
        if name.is_empty() {
            return;
        }

        let file = read::create_as_edit(name.to_string(), default_theme_path);
        if let Err(error) = file {
            log::error!("Error creating theme: {}", error);
            panic!();
        }
        let file = file.unwrap();

        let toolbar_view = window.content().unwrap();
        let navigationsplit_view =
            Helper::find_descendant(toolbar_view.upcast_ref(), "NavSplitView")
                .and_downcast::<NavigationSplitView>()
                .unwrap();
        let nav_page = navigationsplit_view.sidebar().unwrap();

        let sidebar = nav_page.child().and_downcast::<ComponentSideBar>().unwrap(); // what the fuck
        // this is only 4 nested layers

        sidebar.populate(file);
    }
}
