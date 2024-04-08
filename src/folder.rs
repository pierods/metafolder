use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gtk::{ApplicationWindow, EventSequenceState, Fixed, glib, WidgetPaintable};
use gtk::gdk::ContentProvider;
use gtk::glib::{clone, Value};
use gtk::prelude::{Cast, FixedExt, ToVariant, WidgetExt};
use gtk::prelude::GestureExt;
use gtk::prelude::GtkWindowExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{cell, DRAG_ACTION, DROP_TYPE, files, gtk_wrappers, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::cell::DNDInfo;
use crate::gtk_wrappers::{set_drilldown_switch_value, set_window_background};
use crate::settings::MetaFolder;

pub(crate) fn draw_folder(path: String, window: &ApplicationWindow) {
    let entries = files::get_entries(path.clone());

    let desktop_rc = Rc::new(RefCell::new(gtk::Fixed::new()));
    let data_store = gtk_wrappers::get_application(window);

    let desktop = desktop_rc.clone();
    let memo_folder = files::load_settings(path.clone());
    set_window_background(memo_folder.background_color.clone());

    let drilldown = memo_folder.drilldown;
    let mut metafolder = MetaFolder::default();
    metafolder.current_path = path.clone();
    metafolder.background_color = memo_folder.background_color.clone();
    metafolder.drilldown = memo_folder.drilldown;
    metafolder.cell_map = draw_icons(path.clone(), entries, desktop.borrow().as_ref(), INITIAL_DESKTOP_WIDTH, ICON_SIZE, memo_folder);

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::<&gtk::Fixed>::Some(desktop.borrow().as_ref()));
    window.set_child(Option::Some(&scrolled_window));

    let desktop_clone = desktop.clone();
    let drop_target = gtk::DropTarget::new(DROP_TYPE, DRAG_ACTION);
    let metafolder_rc: Rc<RefCell<MetaFolder>> = Rc::new(RefCell::new(metafolder));

    drop_target.connect_drop(move |_drop_target, dnd_msg, x, y| {
        let dnd_info_result = gtk_wrappers::extract_from_variant(dnd_msg);
        match dnd_info_result {
            Ok(csp) => {
                if gtk_wrappers::is_something_underneath(desktop_clone.borrow().as_ref(), x, y, csp.w, csp.h) {
                    return false;
                }
                let mf = data_store.imp().desktop.borrow();
                let cell = mf.get_cell(csp.path.clone());
                if let Some(err) = data_store.imp().desktop.borrow().update_cell_positions(desktop_clone.borrow().as_ref(), csp.path.as_str(), x, y) {
                    let alert = gtk::AlertDialog::builder().modal(true).detail(err.to_string()).message("folder settings could not be saved").build();
                    let root = <Fixed as AsRef<Fixed>>::as_ref(&desktop_clone.borrow()).root().unwrap();
                    let app_window: ApplicationWindow = root.downcast().unwrap();
                    alert.show(Some(&app_window));
                    return false;
                }
                desktop_clone.borrow().move_(cell, x, y);
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
        clone!(@weak  desktop_icon => @default-return None, move |me, _x, _y| {
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