use adw::{
    gio::glib::JoinHandle,
    glib::GString,
    prelude::{MessageDialogExt, MessageDialogExtManual},
};
use gtk::{
    Entry, Window, glib,
    prelude::{EditableExt, EntryExt, GtkWindowExt},
};
use log::trace;

pub struct TextEntryDialog {}
impl TextEntryDialog {
    /// Creates a new `adw::MessageDialog` with a `GtkEntry` child, a canel button, and a submit
    /// button for the provided `adw::Window`.
    ///
    /// Returns a `JoinHandle<GString>` containing the submitted response. The reponse may be
    /// empty.
    pub fn new(window: &Window, title: &str, placeholder: &str) -> JoinHandle<GString> {
        trace!("Creating MessageDialog with title \"{}\"...", title);
        let dialog = adw::MessageDialog::builder()
            .transient_for(window)
            .modal(true)
            .heading(title)
            .build();

        let entry = Entry::builder()
            .placeholder_text(placeholder)
            .activates_default(true)
            .build();

        dialog.set_extra_child(Some(&entry));

        dialog.add_response("cancel", "Cancel");
        dialog.add_response("submit", "Create");
        dialog.set_default_response(Some("submit"));
        dialog.set_close_response("cancel");

        let dialog_rc = dialog.clone();

        dialog.present();
        glib::MainContext::default().spawn_local(async move {
            // TODO: error when typing in Entry:
            // g_variant_iter_loop: assertion 'g_variant_is_of_type (GVSI(iter)->value, G_VARIANT_TYPE_ARRAY)' failed
            // idk why this is happening bc im stupid so
            // but the dialog still works so its fine for now
            // def still needs fixing in the future though
            trace!("Listening to user input response...");
            let response = dialog_rc.choose_future().await;
            trace!("User reponse type \"{}\"", response);
            trace!("User entry content \"{}\"", entry.text());
            if matches!(response.as_str(), "cancel") || entry.text_length() == 0 {
                return GString::new();
            }

            entry.text()
        })
    }
}
