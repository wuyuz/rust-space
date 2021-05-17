use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{ UnitPoint, WidgetExt};

use crate::model::*;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| {
        if data.name.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {}!", data.name)
        }
    })
    .with_text_size(32.0);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .align_vertical(UnitPoint::CENTER)
}


pub fn build_app_widget() -> impl Widget<AppStauts> {
    // let textbox = TextBox::new()
    //     .with_placeholder("search passwords")
    //     .with_text_size(18.0)
    //     .lens(Search::text);

    let textbox2 = TextBox::new()
        .with_placeholder("search passwords")
        .with_text_size(18.0)
        .lens(AppStauts::email);

    Flex::column()
        .with_child(textbox2)
        .align_vertical(UnitPoint::CENTER)

}