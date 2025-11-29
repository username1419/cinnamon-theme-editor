use gtk::Button;

pub fn setup_apply_button() -> Button {
    Button::builder()
        .label("Apply")
        .name("ApplyButton")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Fill)
        .css_classes(["suggested-action", "apply-button"])
        .visible(false)
        .build()
}
