use adw::prelude::{MessageDialogExt, MessageDialogExtManual};
use gtk::{
    Entry, Window, glib,
    prelude::{EditableExt, EntryExt, GtkWindowExt},
};

pub struct TextEntryDialog {}
impl TextEntryDialog {
    /// Creates a new `adw::MessageDialog` with a `GtkEntry` child, a canel button, and a submit
    /// button for the provided `adw::Window`.
    /// Clicking the submit button will call `on_submit()` if GtkEntry::text_length() is not 0.
    /// `on_submit` will be called asynchonously.
    pub fn new<F>(window: &Window, on_submit: F)
    where
        F: Fn(&str) + 'static,
    {
        let dialog = adw::MessageDialog::builder()
            .transient_for(window)
            .modal(true)
            .heading("Enter theme name")
            .build();

        let entry = Entry::builder()
            .placeholder_text("Theme name")
            .activates_default(true)
            .build();

        dialog.set_extra_child(Some(&entry));

        dialog.add_response("cancel", "Cancel");
        dialog.add_response("submit", "Create");
        dialog.set_default_response(Some("submit"));
        dialog.set_close_response("cancel");

        let dialog_rc = dialog.clone();
        glib::MainContext::default().spawn_local(async move {
            let response = dialog_rc.choose_future().await;
            if !matches!(response.as_str(), "ok") || entry.text_length() == 0 {
                return;
            }

            on_submit(entry.text().as_str());
        });

        dialog.present();
    }
}
