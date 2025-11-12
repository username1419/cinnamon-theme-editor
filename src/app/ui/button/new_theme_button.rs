use adw::SplitButton;
use gtk::{Box, Image, Label, gio::Menu, prelude::BoxExt};

pub fn setup_new_theme_button() -> SplitButton {
    let placeholder =
        Image::from_resource("/com/usr1419/cinnamon-desktop-editor/resources/icons/200x200.svg");

    let menu = Menu::new();

    menu.append(Some("Open existing theme"), Some("app.open-existing"));

    let label = Box::new(gtk::Orientation::Horizontal, 2);
    label.append(&placeholder);
    label.append(
        &Label::builder()
            .label("New")
            .valign(gtk::Align::Center)
            .build(),
    );

    SplitButton::builder()
        .vexpand(true)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .name("button-new-theme")
        .css_classes(["flat", "split-button", "suggested-action"])
        .child(&label)
        .direction(gtk::ArrowType::Down)
        .menu_model(&menu)
        .action_name("app.create-new")
        .build()
}
