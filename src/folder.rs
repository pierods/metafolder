use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gtk::{ApplicationWindow, EventSequenceState, Fixed, glib, WidgetPaintable};
use gtk::gdk::ContentProvider;
use gtk::glib::{clone, Value};
use gtk::glib::property::PropertyGet;
use gtk::prelude::{Cast, FixedExt, IconExt, ObjectExt, ToVariant, WidgetExt};
use gtk::prelude::GestureExt;
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{cell, MetaFolder, DRAG_ACTION, DROP_TYPE, files, folder, gtk_wrappers, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::cell::DNDInfo;
use crate::files::{MemoFolder, MemoIcon};

pub(crate) fn draw_folder(path: String, window: &ApplicationWindow) {
    let metafolder_rc: Rc<RefCell<MetaFolder>> = Rc::new(RefCell::new(MetaFolder::default()));
    let c = metafolder_rc.clone();
    let mut metafolder = c.borrow_mut();

    let entries = files::get_entries(path.clone());

    let desktop_rc = Rc::new(RefCell::new(gtk::Fixed::new()));
    let desktop = desktop_rc.clone();
    metafolder.cell_map = folder::draw_icons(path.clone(), entries, desktop.borrow().as_ref(), INITIAL_DESKTOP_WIDTH, ICON_SIZE, files::load_settings(path.clone()));

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::<&gtk::Fixed>::Some(desktop.borrow().as_ref()));
    window.set_child(Option::Some(&scrolled_window));

    let d = desktop.clone();
    let drop_target = gtk::DropTarget::new(DROP_TYPE, DRAG_ACTION);
    let path_rc = Rc::new(RefCell::new(String::from(&path)));
    drop_target.connect_drop(move |drop_target, dnd_msg, x, y| {
        let dnd_info_result = gtk_wrappers::extract_from_variant(dnd_msg);
        match dnd_info_result {
            Ok(csp) => {
                if gtk_wrappers::is_something_underneath(d.borrow().as_ref(), x, y, csp.w, csp.h) {
                    return false;
                }
                let c = metafolder_rc.clone();
                let desktop_props = c.borrow();
                let cell = desktop_props.cell_map.get(csp.path.as_str()).expect("Fatal: cannot find cell");
                let memo_desktop = make_settings(metafolder_rc.clone().borrow(), d.borrow().as_ref(), csp.path.as_str(), x, y);
                let data_store = gtk_wrappers::get_application(<gtk::Fixed as AsRef<gtk::Fixed>>::as_ref(&desktop.borrow()));
                data_store.imp().current_path.replace(String::from(path_rc.clone().borrow().to_string()));
                if let Some(err) = files::save_settings(path_rc.clone().borrow().to_string(), memo_desktop)  {
                    let alert =  gtk::AlertDialog::builder().modal(true).detail(err.to_string()).message("folder settings could not be saved").build();
                    let root = <Fixed as AsRef<Fixed>>::as_ref(&d.borrow()).root().unwrap();
                    let app_window: ApplicationWindow = root.downcast().unwrap();
                    alert.show(Some(&app_window));
                    return false
                }
                d.borrow().move_(cell, x, y);
                true
            }
            Err(err) => {
                println!("err={}", err);
                false
            }
        }
    });
    desktop_rc.clone().borrow().add_controller(drop_target);
    let data_store = gtk_wrappers::get_application(<gtk::Fixed as AsRef<gtk::Fixed>>::as_ref(&desktop_rc.clone().borrow()));
    data_store.imp().current_path.replace(String::from(path));
}

fn make_settings(desktop_props: Ref<MetaFolder>, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> MemoFolder {
    let mut memo_desktop = MemoFolder::default();
    let mut icons: HashMap<String, MemoIcon> = HashMap::new();

    memo_desktop.background_color = desktop_props.background_color.clone();
    for (path, gbox) in desktop_props.cell_map.clone() {
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
        icons.insert(path, memo_icon);
    }
    memo_desktop.icons = icons;
    memo_desktop
}

fn draw_icons(path: String, entries: HashSet<files::DirItem>, desktop: &Fixed, width: i32, size: i32, memo_desktop: files::MemoFolder) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let memo_icons = memo_desktop.icons;
    for entry in entries {
        let name = entry.name.clone();
        let cell = cell::make_cell(String::from(&path), entry, size);
        let drag_source = make_drag_source(name.clone(), &cell, &desktop);
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

fn make_drag_source(path_name: String, desktop_icon: &gtk::Box, layout: &Fixed) -> gtk::DragSource {
    let drag_source = gtk::DragSource::new();
    drag_source.set_actions(DRAG_ACTION);
    let path_copy = String::from(path_name.as_str());
    let l_clone = layout.clone();
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, x, y| {
            me.set_state(EventSequenceState::Claimed);
            let mut dnd_info  = DNDInfo::default();
            dnd_info.path = path_copy.to_string();
            let actual_bounds = gtk_wrappers::get_widget_bounds(l_clone.as_ref(), &desktop_icon);
            dnd_info.pos_x = actual_bounds.x() as f64;
            dnd_info.pos_y = actual_bounds.y() as f64;
            dnd_info.w = actual_bounds.width() as f64;
            dnd_info.h = actual_bounds.height() as f64;
            let w_p = WidgetPaintable::new(Some(&desktop_icon));
            //TODO hot_x, hot_y
            me.set_icon(Some(&w_p), 0, 0);
            // must coincide with DROP_TYPE
            Some(ContentProvider::for_value(&Value::from(dnd_info.to_variant())))
        })
    );
    let w_p = WidgetPaintable::new(Some(desktop_icon));
    //TODO hot_x, hot_y
    drag_source.set_icon(Some(&w_p), 0, 0);
    drag_source
}