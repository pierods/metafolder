use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gtk::{ApplicationWindow, Fixed};
use gtk::prelude::{Cast, FixedExt, WidgetExt};
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{cell, DRAG_ACTION, DROP_TYPE, files, gtk_wrappers, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::gtk_wrappers::{set_bgcolor_button_color, set_drilldown_switch_value, set_window_background};
use crate::settings::MetaFolder;

pub(crate) fn draw_folder(path: String, window: &ApplicationWindow) {
    let entries = files::get_entries(path.clone());

    let desktop_rc = Rc::new(RefCell::new(gtk::Fixed::new()));
    let data_store = gtk_wrappers::get_application(window);

    let desktop = desktop_rc.clone();
    let memo_folder = files::load_settings(path.clone());
    set_window_background(memo_folder.background_color.clone());

    let drilldown = memo_folder.drilldown;
    let bg_color = memo_folder.background_color.clone();
    let zoom = memo_folder.zoom;
    let zoom_x = memo_folder.zoom_x;
    let zoom_y = memo_folder.zoom_y;
    let mut metafolder = MetaFolder::default();
    metafolder.current_path = path.clone();
    metafolder.background_color = memo_folder.background_color.clone();
    metafolder.drilldown = memo_folder.drilldown;
    metafolder.zoom = memo_folder.zoom;
    metafolder.zoom_x = memo_folder.zoom_x;
    metafolder.zoom_y = memo_folder.zoom_y;
    metafolder.cell_map = draw_icons(path.clone(), entries, desktop.borrow().as_ref(), INITIAL_DESKTOP_WIDTH, ICON_SIZE, memo_folder);

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::<&gtk::Fixed>::Some(desktop.borrow().as_ref()));
    window.set_child(Option::Some(&scrolled_window));

    let desktop_clone = desktop.clone();
    let drop_target = gtk::DropTarget::new(DROP_TYPE, DRAG_ACTION);
    let metafolder_rc: Rc<RefCell<MetaFolder>> = Rc::new(RefCell::new(metafolder));

    drop_target.connect_drop(move |_drop_target, dnd_msg, x, y| {
        match gtk_wrappers::extract_from_variant(dnd_msg) {
            Ok(dnd_info) => {
                let mf = data_store.imp().desktop.borrow();
                let cell = mf.get_cell(dnd_info.name.clone());
                if gtk_wrappers::is_something_underneath(dnd_info.name.clone(), desktop_clone.borrow().as_ref(), x-dnd_info.grabbed_x, y-dnd_info.grabbed_y, dnd_info.w, dnd_info.h) {
                    return false;
                }
                if let Some(err) = data_store.imp().desktop.borrow().arrange_cells_and_save_settings(desktop_clone.borrow().as_ref(), dnd_info.name.as_str(), x, y) {
                    let alert = gtk::AlertDialog::builder().modal(true).detail(err.to_string()).message("folder settings could not be saved").build();
                    let root = <Fixed as AsRef<Fixed>>::as_ref(&desktop_clone.borrow()).root().unwrap();
                    let app_window: ApplicationWindow = root.downcast().unwrap();
                    alert.show(Some(&app_window));
                    return false;
                }
                desktop_clone.borrow().move_(cell, x - dnd_info.grabbed_x, y - dnd_info.grabbed_y);
                true
            }
            Err(err) => {
                println!("error on drop_target.drop: {}", err);
                false
            }
        }
    });
    desktop_rc.clone().borrow().add_controller(drop_target);

    let data_store = gtk_wrappers::get_application(window);
    data_store.imp().desktop.borrow_mut().build_new(&metafolder_rc.clone().borrow());
    // must do it after drawing desktop because it will trigger a save settings and go out of sync b/c done before data_store.desktop is set
    //  (therefore going to the wrong path)

    set_drilldown_switch_value(window, drilldown);
    set_bgcolor_button_color(window, bg_color);
    if zoom {
        let ds = gtk_wrappers::get_application(window);
        ds.imp().desktop.borrow_mut().zoom_and_set_zoom_widgets(zoom_x, zoom_y, window);
    }
}

fn draw_icons(path: String, entries: HashSet<files::DirItem>, desktop: &Fixed, width: i32, size: i32, memo_desktop: files::MemoFolder) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let memo_icons = memo_desktop.icons;
    for entry in entries {
        let name = entry.name.clone();
        let cell = cell::make_cell(String::from(&path), entry, size);
        let drag_source = cell::make_drag_source(name.clone(), &cell, &desktop);
        cell.add_controller(drag_source);
        if !memo_icons.contains_key(name.as_str()) {
            desktop.put(&cell, c as f64, r as f64);
        } else {
            desktop.put(&cell, memo_icons.get(name.as_str()).unwrap().position_x as f64, memo_icons.get(name.as_str()).unwrap().position_y as f64);
        }

        cell_map.insert(name, cell);
        c += size + size / 3;
        if c > width {
            c = 0;
            r += 2 * size;
        }
        //break;
    }
    cell_map
}