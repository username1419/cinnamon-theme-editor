use adw::Application;
use adw::gio::prelude::{ApplicationExt, ApplicationExtManual};
use adw::glib::{ExitCode, SignalHandlerId};
use gtk::{gdk, gio, glib};

/// Wrapper for adw::Application
pub struct App {
    application: Application,
}

impl App {
    pub fn new() -> Self {
        let resource_bytes = include_bytes!("../../data/resources.gresource");
        let resources =
            gio::Resource::from_data(&glib::Bytes::from_static(resource_bytes)).unwrap();
        gio::resources_register(&resources);

        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/usr1419/cinnamon-desktop-editor/resources/style.css");

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        Self {
            application: Application::builder()
                .application_id("com.usr1419.cinnamon-desktop-editor")
                .build(),
        }
    }

    pub fn run(&self) -> ExitCode {
        self.application.run()
    }

    pub fn run_with_args(&self, args: &[String]) -> ExitCode {
        self.application.run_with_args(args)
    }

    pub fn connect_activate<F: Fn(&Application) + 'static>(&self, f: F) -> SignalHandlerId {
        self.application.connect_activate(f)
    }

    pub fn inner(&self) -> &Application {
        &self.application
    }
}
