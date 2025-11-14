use simple_logger::SimpleLogger;

use crate::app::application::App;
use crate::app::window;

pub mod app;
pub mod helper;

fn main() {
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

    adw::init().expect("Failed to initialize libadwaita");

    let app = App::new();
    app.connect_activate(window::MainWindow::initialize);
    app.run_with_args(&unknown_args);
}
