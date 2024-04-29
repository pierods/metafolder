use crate::glib::clone;
use crate::glib;
use crate::gtk_wrappers::{is_something_underneath, set_title_path};
use std::collections::{HashMap, HashSet};

use gtk::{ApplicationWindow, Fixed, gio};
use gtk::gio::{Cancellable, File, FileMonitorEvent, FileMonitorFlags};
use gtk::glib::Value;
use gtk::prelude::{Cast, FileExt, FileMonitorExt, FixedExt, IsA, WidgetExt};
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{cell, DRAG_ACTION, DROP_TYPE, files, gtk_wrappers, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::files::MemoFolder;
use crate::gtk_wrappers::{set_bgcolor_button_color, set_drilldown_switch_value, set_window_background, set_zoom_widgets};
use crate::metafolder::MetaFolder;

pub(crate) fn draw_folder(path: String, window: &ApplicationWindow) {
    let entries = files::get_entries(path.clone());

    let desktop = gtk::Fixed::new();
    let memo_folder = files::load_settings(path.clone());
    set_window_background(memo_folder.background_color.clone());

    let mut metafolder = MetaFolder::new();
    metafolder.current_path = path.clone();
    metafolder.background_color = memo_folder.background_color.clone();
    metafolder.drilldown = memo_folder.drilldown;
    metafolder.zoom = memo_folder.zoom;
    metafolder.zoom_x = memo_folder.zoom_x;
    metafolder.zoom_y = memo_folder.zoom_y;
    let (cell_map, new_entries) = draw_icons(path.clone(), entries, &desktop, INITIAL_DESKTOP_WIDTH, ICON_SIZE, &memo_folder);

    metafolder.cell_map = cell_map;
    metafolder.added_cells = new_entries;
    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::<&gtk::Fixed>::Some(&desktop));
    window.set_child(Option::Some(&scrolled_window));

    let drop_target = gtk::DropTarget::new(DROP_TYPE, DRAG_ACTION);
    drop_target.connect_drop(clone!(@weak desktop => @default-return false, move |_drop_target, dnd_msg, x, y| {
        drop_action(dnd_msg, &desktop, x, y)
    }));
    desktop.add_controller(drop_target);

    let data_store = gtk_wrappers::get_application(window);
    data_store.imp().metafolder.replace(metafolder);
    // must do it after drawing desktop because it will trigger a save settings and go out of sync b/c done before data_store.desktop is set
    //  (therefore going to the wrong path)

    apply_stored_settings(window, &memo_folder);
    set_title_path(window, path.clone());

    let watched = gio::File::for_path(path);
    let monitor = watched.monitor_directory(FileMonitorFlags::WATCH_MOVES, None::<&Cancellable>).expect("Fatal: cannot monitor directory");
    monitor.connect_changed(clone!(@weak window => move |_, f, other, event |{
        monitor_folder(f, other, event, &desktop);
    }));
    let ds = gtk_wrappers::get_application(window);
    ds.imp().monitor.replace(Some(monitor));
}

fn apply_stored_settings(w: &impl IsA<gtk::Widget>, memo_folder: &MemoFolder) {
    let ds = gtk_wrappers::get_application(w);
    set_drilldown_switch_value(w, memo_folder.drilldown);
    set_bgcolor_button_color(w, memo_folder.background_color.clone());
    if memo_folder.zoom {
        ds.imp().metafolder.borrow_mut().zoom_and_set_zoom_widgets(memo_folder.zoom_x, memo_folder.zoom_y, w);
    } else {
        set_zoom_widgets(w, false, 100, 100);
    }
    //TODO move scales and switches
    if memo_folder.cell_size != 0 {
        ds.imp().metafolder.borrow().change_cell_size(memo_folder.cell_size, false);

    }
    if memo_folder.font_color != "" {
        ds.imp().metafolder.borrow().change_font_color(memo_folder.font_color.clone(), false);
    }
    if memo_folder.font_size != "" {
        ds.imp().metafolder.borrow().change_font_size(memo_folder.font_size.clone(), false);
    }
    if !memo_folder.font_bold {
        ds.imp().metafolder.borrow().change_bold(memo_folder.font_bold, false);
    }
}

fn drop_action(dnd_msg: &Value, desktop: &Fixed, x: f64, y: f64) -> bool {
    let data_store = gtk_wrappers::get_application(desktop);

    match gtk_wrappers::extract_from_variant(dnd_msg) {
        Ok(dnd_info) => {
            let name = dnd_info.name;
            if gtk_wrappers::is_something_underneath(name.clone(), &desktop, x - dnd_info.grabbed_x, y - dnd_info.grabbed_y, dnd_info.w, dnd_info.h) {
                return false;
            }
            let mut mf = data_store.imp().metafolder.borrow_mut();
            if mf.is_cell_newly_added(name.clone()) {
                mf.clear_added_flag(name.clone());
            }
            drop(mf);
            let mf = data_store.imp().metafolder.borrow();
            if let Some(err) = mf.scan_positions_and_save_settings(&desktop, name.as_str(), x - dnd_info.grabbed_x, y - dnd_info.grabbed_y) {
                let alert = gtk::AlertDialog::builder().modal(true).detail(err.to_string()).message("folder settings could not be saved").build();
                let root = <Fixed as AsRef<Fixed>>::as_ref(&desktop).root().unwrap();
                let app_window: ApplicationWindow = root.downcast().unwrap();
                alert.show(Some(&app_window));
                return false;
            }
            let cell = mf.get_cell(name.clone());
            desktop.move_(cell, x - dnd_info.grabbed_x, y - dnd_info.grabbed_y);
            cell.remove_css_class("icon_added");
            true
        }
        Err(err) => {
            println!("error on drop_target.drop: {}", err);
            false
        }
    }
}

fn monitor_folder(f: &File, other: Option<&File>, event: FileMonitorEvent, d: &Fixed) {
    if f.basename().unwrap().to_str().unwrap() == ".metafolder" {
        return;
    }
    match event {
        FileMonitorEvent::Deleted | FileMonitorEvent::MovedOut => {
            let ds = gtk_wrappers::get_application(d);
            let name = f.basename().expect("Fatal: no basename");
            let (icon, _err) = ds.imp().metafolder.borrow_mut().delete_cell(name.to_str().unwrap().to_string());
            d.remove(&icon);
        }
        FileMonitorEvent::Created | FileMonitorEvent::MovedIn => {
            let full_path_unwrap = f.path().unwrap();
            let full_path = full_path_unwrap.to_str().unwrap();
            let file_info = files::get_file_info(full_path.to_string());

            let cell = cell::make_cell(full_path.to_string(), &file_info.unwrap(), ICON_SIZE);
            let drag_source = cell::make_drag_source(f.basename().unwrap().to_str().unwrap().to_string(), &cell, d);
            cell.add_controller(drag_source);
            cell.set_css_classes(&["icon_added"]);
            drop_icon_on_free_space(d, &cell, ICON_SIZE, INITIAL_DESKTOP_WIDTH);
            let ds = gtk_wrappers::get_application(d);
            ds.imp().metafolder.borrow_mut().add_cell(f.basename().unwrap().to_str().unwrap().to_string(), cell);
        }
        FileMonitorEvent::Renamed => {
            let old_name_binding = f.basename().unwrap();
            let old_name = old_name_binding.to_str().unwrap();
            let new_name_binding = other.unwrap().basename().unwrap();
            let new_name = new_name_binding.to_str().unwrap();
            let ds = gtk_wrappers::get_application(d);
            ds.imp().metafolder.borrow_mut().rename_cell(old_name, new_name);
        }
        _ => { println!("Unhandled file event on {}", f.basename().unwrap().to_str().unwrap()); }
    }
}

fn draw_icons(path: String, entries: HashSet<files::DirItem>, desktop: &Fixed, desktop_width: i32, icon_size: i32, memo_desktop: &files::MemoFolder) -> (HashMap<String, gtk::Box>, HashSet<String>) {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();
    let mut new_entries: HashSet<String> = HashSet::new();
    let memo_icons = &memo_desktop.icons;
    let mut max_y = 0;
    for entry in entries {
        let name = entry.name.clone();
        let cell = cell::make_cell(String::from(&path), &entry, icon_size);
        let drag_source = cell::make_drag_source(name.clone(), &cell, &desktop);
        cell.add_controller(drag_source);
        if !memo_icons.contains_key(name.as_str()) {
            new_entries.insert(name.clone());
        } else {
            desktop.put(&cell, memo_icons.get(name.as_str()).unwrap().position_x as f64, memo_icons.get(name.as_str()).unwrap().position_y as f64);
            if memo_icons.get(name.as_str()).unwrap().position_y > max_y {
                max_y = memo_icons.get(name.as_str()).unwrap().position_y;
            }
        }
        cell_map.insert(name, cell);
    }

    let mut r: i32 = 0;
    let mut c: i32 = max_y;
    for (name, cell) in &cell_map {
        c += icon_size + icon_size / 3;
        if c > desktop_width {
            c = 0;
            r += 2 * icon_size;
        }
        if new_entries.contains(name.as_str()) {
            cell.set_css_classes(&["icon_added"]);
            desktop.put(cell, c as f64, r as f64);
        }
    }

    (cell_map, new_entries)
}

fn drop_icon_on_free_space(d: &Fixed, icon: &gtk::Box, icon_size: i32, desktop_width: i32) {
    let mut r: i32 = 0;
    let mut c: i32 = 0;

    while is_something_underneath("".to_string(), d, c as f64, r as f64, icon_size as f64, icon_size as f64) {
        c += icon_size + icon_size / 3;
        if c > desktop_width {
            c = 0;
            r += 2 * icon_size;
        }
    }
    d.put(icon, c as f64, r as f64);
}