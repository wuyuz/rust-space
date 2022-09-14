use relm4::{RelmApp};

mod model;
mod utils;
mod widgets;
mod data;

fn main() {
    let model = model::AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}