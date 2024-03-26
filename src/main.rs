use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::rc::Rc;

use gtk::{Application, ApplicationWindow, Fixed, glib};
use gtk::gdk::DragAction;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};

mod cell;
mod files;

const APP_ID: &str = "org.github.pierods.metafolder";
const DRAG_ACTION: DragAction = DragAction::MOVE;
const ICON_SIZE: i32 = 60;

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
    window.present();

    let desktop_props_rc: Rc<RefCell<Desktop>> = Rc::new(RefCell::new(Desktop::default()));
    let c = desktop_props_rc.clone();
    let mut desktop_props = c.borrow_mut();

    desktop_props.path_name = files::home_path();
    if files::try_file((desktop_props.path_name.clone() + "/Desktop").as_str()) {
        desktop_props.path_name += "/Desktop";
    }
    let entries = files::get_entries(desktop_props.path_name.clone());

    let desktop = gtk::Fixed::new();
    desktop_props.cell_map = draw_icons(entries, &desktop, 1500, ICON_SIZE, files::load_settings(desktop_props.path_name.clone()));

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::Some(&desktop));
    window.set_child(Option::Some(&scrolled_window));

    let drop_target = gtk::DropTarget::new(glib::types::Type::OBJECT, DRAG_ACTION);
    drop_target.set_types(&[glib::types::Type::STRING]);

    drop_target.connect_drop(move |window, value, x, y| {
        let drop = value.get::<&str>();
        match drop {
            Ok(lab) => {
                let c = desktop_props_rc.clone();
                let desktop_props = c.borrow();
                let cell = desktop_props.cell_map.get(lab).expect("Fatal: cannot find cell");
                desktop.move_(cell, x, y);
                files::save_settings(desktop_props);
                true
            }
            Err(err) => {
                println!("err={}", err);
                false
            }
        }
    });
    window.add_controller(drop_target);
}

fn draw_icons(entries: HashSet<files::DirItem>, desktop: &Fixed, width: i32, size: i32, memo_desktop: files::MemoDesktop) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let memo_icons = memo_desktop.icons;
    for entry in entries {
        let path_name = entry.path_name.clone();
        let cell = cell::make_cell(entry, size);
        if !memo_icons.contains_key(path_name.as_str()) {
            desktop.put(&cell, c as f64, r as f64);
        } else {
            desktop.put(&cell, memo_icons.get(path_name.as_str()).unwrap().position_x as f64, memo_icons.get(path_name.as_str()).unwrap().position_y as f64);
        }

        cell_map.insert(path_name, cell);
        c += size;
        if c > width {
            c = 0;
            r += 2 * size;
        }
        //break;
    }
    cell_map
}


