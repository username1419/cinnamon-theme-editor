use gtk::Button;

pub fn setup_apply_button() -> Button {
    // TODO: disable when theme not loaded
    Button::builder()
        .label("Apply")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Fill)
        .css_classes(["suggested-action", "apply-button"])
        .build()
}
