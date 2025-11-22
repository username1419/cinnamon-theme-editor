use adw::SplitButton;
use gtk::{
    Box, Image,
    gio::Menu,
    prelude::{BoxExt, WidgetExt},
};
use log::trace;

pub fn setup_new_theme_button() -> SplitButton {
    let placeholder =
        Image::from_resource("/com/usr1419/cinnamon-desktop-editor/resources/icons/200x200.svg");
    placeholder.set_icon_size(gtk::IconSize::Large);
    trace!(
        "Loaded new theme creation icon, size {:?}",
        placeholder.icon_size()
    );

    let menu = Menu::new();

    menu.append(Some("Open existing theme"), Some("app.open-existing"));

    let label = Box::builder().hexpand(true).vexpand(true).build();
    label.append(&placeholder);

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
