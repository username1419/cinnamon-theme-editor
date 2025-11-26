use adw::gtk::Label;
use adw::gtk::prelude::GtkWindowExt;
use adw::{ApplicationWindow, HeaderBar, NavigationPage, NavigationSplitView, ToolbarView, Window};
use gtk::Box;
use gtk::prelude::WidgetExt;
use log::trace;

use super::actions::setup_actions;
use super::ui::button::apply_button::setup_apply_button;
use super::ui::button::new_theme_button::setup_new_theme_button;
use super::ui::footerbar::FooterBar;
use super::ui::sidebar::components_sidebar::ComponentSideBar;

#[derive(Debug)]
/// The preset layout of the main window.
pub struct MainWindow {}

impl MainWindow {
    /// Initializes the given Application based on a preset layout
    pub fn initialize(app: &adw::Application) {
        let sidebar = NavigationPage::builder()
            .child(&Self::setup_sidebar())
            .title("Sidebar")
            .name("SidebarPage")
            .build();
        trace!("Created sidebar widget");

        let main_content = NavigationSplitView::builder()
            .name("NavSplitView")
            .sidebar(&sidebar)
            .content(
                &NavigationPage::builder()
                    .child(&Box::builder().build())
                    .title("MainContent")
                    .build(),
            )
            .build();
        trace!("Created split view container");

        let tool_bar = ToolbarView::builder()
            .name("ToolbarView")
            .top_bar_style(adw::ToolbarStyle::Flat)
            .halign(gtk::Align::Fill)
            .hexpand(true)
            .bottom_bar_style(adw::ToolbarStyle::Flat)
            .content(&main_content)
            .build();
        trace!("Created toolbar container");
        let header = Self::setup_top_toolbar();
        tool_bar.add_top_bar(&header);
        trace!("Created header bar");
        let footer = Self::setup_bottom_toolbar();
        tool_bar.add_bottom_bar(&footer);
        trace!("Created footer bar");

        let app_window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .default_height(450)
            .title("Cinnamon Dekstop Editor")
            .content(&tool_bar)
            .build();

        setup_actions(app.to_owned(), app_window.clone());

        app_window.present();
    }

    /// Setup the sidebar
    pub fn setup_sidebar() -> ComponentSideBar {
        let sidebar = ComponentSideBar::new();
        sidebar.add_css_class("sidebar");

        sidebar
    }

    pub fn setup_top_toolbar() -> HeaderBar {
        let header = HeaderBar::builder()
            .title_widget(
                &Label::builder()
                    .label("Cinnamon Desktop Editor")
                    .halign(gtk::Align::Center)
                    .valign(gtk::Align::Center)
                    .build(),
            )
            .show_title(true)
            .build();

        header.pack_start(&setup_new_theme_button());

        header
    }

    pub fn setup_bottom_toolbar() -> FooterBar {
        let mut footer = FooterBar::new();
        footer.pack_end(&setup_apply_button());

        footer
    }
}
