use std::collections::HashMap;
use std::io::{Read, Write};

use gtk::{Application, ApplicationWindow, glib};
use gtk::gdk;
use gtk::gdk::DragAction;
use gtk::glib::Type;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};

mod folder;
mod files;
mod cell;

const APP_ID: &str = "org.github.pierods.metafolder";
const DRAG_ACTION: DragAction = DragAction::MOVE;
const ICON_SIZE: i32 = 60;
const INITIAL_DESKTOP_WIDTH: i32 = 1024;
const DROP_TYPE : Type = Type::VARIANT;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

#[derive(Default)]
struct Desktop {
    path_name: String,
    background_color: String,
    cell_map: HashMap<String, gtk::Box>,
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("metafolder").build();
    window.set_default_size(1024, 768);
    window.connect_maximized_notify(|win: &ApplicationWindow| { println!("*****************************{}", win.width()) });
    window.maximize();

    let provider = gtk::CssProvider::new();
    let bytes = glib::Bytes::from("window {background-color:rgba(80,80,80,80);}".as_bytes());
    provider.load_from_bytes(&bytes);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.present();
    folder::draw_folder(&window);
}

