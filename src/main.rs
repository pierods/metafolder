use std::collections::HashMap;
use std::io::{Read, Write};

use gtk::{Application, ApplicationWindow, glib};
use gtk::gdk;
use gtk::gdk::DragAction;
use gtk::glib::Type;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use crate::appwindow_with_datastore::AppWithDatastore;

use crate::menus::make_header_bar;

mod folder;
mod files;
mod cell;
mod menus;
mod appwindow_with_datastore;
mod gtk_wrappers;

const APP_ID: &str = "org.github.pierods.metafolder";
const DRAG_ACTION: DragAction = DragAction::MOVE;
const ICON_SIZE: i32 = 60;
const INITIAL_DESKTOP_WIDTH: i32 = 1024;
const DROP_TYPE : Type = Type::VARIANT;

fn main() -> glib::ExitCode {
    //let app = Application::builder().application_id(APP_ID).build();
    let app = AppWithDatastore::default();
    //let to_app : gtk::Application = app.into();
    // to_app.connect_activate(build_ui);
    // to_app.run()
    app.connect_activate(build_ui);
    app.run()
}

#[derive(Default)]
struct Desktop {
    path_name: String,
    background_color: String,
    cell_size: i32,
    cell_map: HashMap<String, gtk::Box>,
}

fn build_ui(app: &AppWithDatastore) {
    let window = ApplicationWindow::builder().application(app).title("metafolder").build();
    window.set_titlebar(Some(&make_header_bar()));
    window.set_default_size(1024, 768);
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
    folder::draw_folder(files::initial_dir(), &window);
}