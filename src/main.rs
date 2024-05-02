use gtk::{ApplicationWindow, glib};
use gtk::gdk;
use gtk::gdk::DragAction;
use gtk::glib::Type;
use gtk::prelude::*;

use crate::app_with_datastore::AppWithDatastore;
use crate::menus::make_header_bar;

mod folder;
mod files;
mod cell;
mod menus;
mod app_with_datastore;
mod gtk_wrappers;
mod metafolder;
mod zoom;
mod cell_editor;

const APP_ID: &str = "metafolder";
const DRAG_ACTION: DragAction = DragAction::MOVE;
const ICON_SIZE: i32 = 60;
const INITIAL_DESKTOP_WIDTH: i32 = 1024;
const DROP_TYPE: Type = Type::VARIANT;
const DEFAULT_BG_COLOR: &str = "rgba(170, 170, 170, 1)";
const CLASSES: &str = " .folder_zoomed {background-image: none; background-color: rgba(245, 241, 39, 0.8);} .icon_added {background-color: rgba(214, 39, 39, 0.35);}";
static CELL_SIZES: &'static [i32] = &[40, 60, 80];
static FONT_SIZES: &'static [&str] = &["xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large"];

fn main() -> glib::ExitCode {
    glib::set_application_name("metafolder");
    let app = AppWithDatastore::default();
    app.set_application_id(Some("metafolder"));
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &AppWithDatastore) {
    let window = ApplicationWindow::builder().application(app).title("metafolder").build();
    window.set_titlebar(Some(&make_header_bar(&window)));
    window.set_default_size(1024, 768);
    window.maximize();

    let provider = gtk::CssProvider::new();
    let bytes = glib::Bytes::from(String::from(("window {background-color:").to_owned() + DEFAULT_BG_COLOR + "; border-radius: 7px;} box {border-radius: 7px;}" + CLASSES).as_bytes());
    provider.load_from_bytes(&bytes);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.present();
    folder::draw_folder(files::initial_dir(), &window);
}