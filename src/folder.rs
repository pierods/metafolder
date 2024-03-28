use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gtk::{ApplicationWindow, Fixed, glib, EventSequenceState};
use gtk::prelude::{FixedExt, WidgetExt};
use gtk::prelude::GtkWindowExt;

use crate::{cell, Desktop, DRAG_ACTION, files, folder, ICON_SIZE, INITIAL_DESKTOP_WIDTH};

pub(crate) fn draw_folder(window: &ApplicationWindow) {
    let desktop_props_rc: Rc<RefCell<Desktop>> = Rc::new(RefCell::new(Desktop::default()));
    let c = desktop_props_rc.clone();
    let mut desktop_props = c.borrow_mut();

    desktop_props.path_name = files::home_path();
    if files::try_file((desktop_props.path_name.clone() + "/Desktop").as_str()) {
        desktop_props.path_name += "/Desktop";
    }
    let entries = files::get_entries(desktop_props.path_name.clone());

    let desktop_rc = Rc::new(RefCell::new(gtk::Fixed::new()));
    let desktop = desktop_rc.clone();
    desktop_props.cell_map = folder::draw_icons(entries, desktop.borrow().as_ref(), INITIAL_DESKTOP_WIDTH, ICON_SIZE, files::load_settings(desktop_props.path_name.clone()));

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::<&gtk::Fixed>::Some(desktop.borrow().as_ref()));
    window.set_child(Option::Some(&scrolled_window));

    let drop_target = gtk::DropTarget::new(glib::types::Type::OBJECT, DRAG_ACTION);
    drop_target.set_types(&[glib::types::Type::STRING]);

    let d = desktop.clone();
    drop_target.connect_drop(move |window, value, x, y| {
        let drop = value.get::<&str>();
        match drop {
            Ok(icon_file_path) => {
                let c = desktop_props_rc.clone();
                let desktop_props = c.borrow();
                let cell = desktop_props.cell_map.get(icon_file_path).expect("Fatal: cannot find cell");
                d.borrow().move_(cell, x, y);
                files::save_settings(desktop_props, icon_file_path, x, y);
                true
            }
            Err(err) => {
                println!("err={}", err);
                false
            }
        }
    });
    desktop.borrow().add_controller(drop_target);
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
        c += size + size / 3;
        if c > width {
            c = 0;
            r += 2 * size;
        }
        //break;
    }
    cell_map
}