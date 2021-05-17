use druid::{AppLauncher, WindowDesc};

mod build_widget;
mod model;

use model::*;
use build_widget::*;

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_app_widget)
        .title("Cipher")
        .resizable(false)
        .window_size((450.0, 450.0));


    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .launch(AppStauts::new())
        .expect("Failed to launch application");
}

