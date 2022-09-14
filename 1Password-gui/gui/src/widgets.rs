use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{send, WidgetPlus, Widgets};
use crate::model::AppModel;
use crate::data::AppMsg;

// pub struct AppWidgets {
//     window: gtk::ApplicationWindow,
//     vbox: gtk::Box,
//     inc_button: gtk::Button,
//     dec_button: gtk::Button,
//     label: gtk::Label,
// }

// impl Widgets<AppModel, ()> for AppWidgets {
//     type Root = gtk::ApplicationWindow;

//     /// Initialize the UI.
//     fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
//         let window = gtk::ApplicationWindow::builder()
//             .title("1Password")
//             .default_width(300)
//             .default_height(100)
//             .build();
//         let vbox = gtk::Box::builder()
//             .orientation(gtk::Orientation::Vertical)
//             .spacing(5)
//             .build();
//         vbox.set_margin_all(5);

//         let inc_button = gtk::Button::with_label("Increment");
//         let dec_button = gtk::Button::with_label("Decrement");

//         let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
//         label.set_margin_all(5);

//         // Connect the widgets
//         window.set_child(Some(&vbox));
//         vbox.append(&inc_button);
//         vbox.append(&dec_button);
//         vbox.append(&label);

//         // Connect events
//         let btn_sender = sender.clone();
//         inc_button.connect_clicked(move |_| {
//             send!(btn_sender, AppMsg::Increment);
//         });

//         dec_button.connect_clicked(move |_| {
//             send!(sender, AppMsg::Decrement);
//         });

//         Self {
//             window,
//             vbox,
//             inc_button,
//             dec_button,
//             label,
//         }
//     }

//     /// Return the root widget.
//     fn root_widget(&self) -> Self::Root {
//         self.window.clone()
//     }

//     /// Update the view to represent the updated model.
//     fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
//         self.label.set_label(&format!("Counter: {}", model.counter));
//     }
// }


#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Increment);
                    },
                },
                append = &gtk::Button {
                    set_label: "Decrement",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Decrement);
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Counter: {}", model.counter) },
                }
            },
        }
    }
}
