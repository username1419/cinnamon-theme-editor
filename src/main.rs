use core::panic;

use simple_logger::SimpleLogger;

use crate::app::application::App;
use crate::app::window;

pub mod app;
pub mod helper;

fn main() {
    if cfg!(windows) {
        panic!("Unsupported on Windows");
    }

    match std::env::var("DESKTOP_SESSION") {
        Ok(value) => {
            if value != "cinnamon" {
                panic!(
                    "This program is only compatible with the Cinnamon Desktop Environment. Current desktop environment: {}.",
                    value
                );
            }
        }
        Err(error) => panic!("{}", error),
    }

    let args = std::env::args().skip(1);
    let mut unknown_args = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--trace" => {
                println!("Starting log at trace level");
                SimpleLogger::new().init().unwrap();
            }
            _ => {
                unknown_args.push(arg);
            }
        }
    }

    // enable inspector if compiled in debug mode
    if cfg!(debug_assertions) {
        unsafe {
            std::env::set_var("GTK_DEBUG", "interactive");
        }
    }

    adw::init().expect("Failed to initialize libadwaita");

    let app = App::new();
    app.connect_activate(window::MainWindow::initialize);
    app.run_with_args(&unknown_args);
}
