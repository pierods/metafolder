use std::collections::HashMap;
use std::io::{Read, Write};

use gtk::{ApplicationWindow, Fixed, glib};
use gtk::gdk;
use gtk::gdk::DragAction;
use gtk::glib::Type;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use ignore::Error;
use serde::{Deserialize, Serialize};

use crate::appwindow_with_datastore::AppWithDatastore;
use crate::files::{load_settings, MemoFolder, MemoIcon};
use crate::gtk_wrappers::get_desktop;
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
    let app = AppWithDatastore::default();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &AppWithDatastore) {
    let window = ApplicationWindow::builder().application(app).title("metafolder").build();
    window.set_titlebar(Some(&make_header_bar()));
    window.set_default_size(1024, 768);
    window.maximize();

    let provider = gtk::CssProvider::new();
    let bytes = glib::Bytes::from("window {background-color:rgba(80,80,80,255); border-radius: 7px;}".as_bytes());
    provider.load_from_bytes(&bytes);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.present();
    folder::draw_folder(files::initial_dir(), &window);
}

#[derive(Default, Debug)]
struct MetaFolder {
    background_color: String,
    cell_size: i32,
    drilldown: bool,
    cell_map: HashMap<String, gtk::Box>,
    current_path : String,
}


impl MetaFolder{
    fn update_current_path(&mut self, new_path: String) {
        self.current_path = new_path
    }
    fn update_cell_positions(&self, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> Option<Error> {
        let mut memo_folder = MemoFolder::default();
        let mut icons: HashMap<String, MemoIcon> = HashMap::new();

        for (path, gbox) in &self.cell_map {
            let memo_icon: MemoIcon;
            if path == icon_file_path {
                memo_icon = MemoIcon {
                    position_x: x as i32,
                    position_y: y as i32,
                };
            } else {
                //let bounds = gbox.allocation();
                let bounds = gtk_wrappers::get_widget_bounds(desktop, &gbox);
                memo_icon = MemoIcon {
                    position_x: bounds.x() as i32,
                    position_y: bounds.y() as i32,
                };
            }
            icons.insert(path.to_string(), memo_icon);
        }
        memo_folder.icons = icons;
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    fn update_background_color(&mut self, new_color: String) -> Option<Error> {
        self.background_color = new_color.clone();

        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.background_color = new_color;
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    fn build_new(&mut self, new_metafolder: &MetaFolder) {
        self.current_path = new_metafolder.current_path.clone();
        self.cell_size = new_metafolder.cell_size;
        self.background_color = new_metafolder.background_color.clone();
        self.drilldown = new_metafolder.drilldown;
        self.cell_map = new_metafolder.cell_map.clone();
    }

    fn get_cell(&self, csp : String) -> &gtk::Box {
        self.cell_map.get(&csp).expect("Fatal: cannot find cell")
    }

    fn get_current_path(&self) -> String {
        self.current_path.clone()
    }
}