use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::rc::Rc;

use gtk::{ApplicationWindow, EventSequenceState, Fixed, glib, glib::Variant, PickFlags, WidgetPaintable};
use gtk::gdk::ContentProvider;
use gtk::glib::{clone, Value};
use gtk::glib::property::PropertyGet;
use gtk::graphene::Rect;
use gtk::prelude::{FixedExt, ObjectExt, ToVariant, WidgetExt};
use gtk::prelude::GestureExt;
use gtk::prelude::GtkWindowExt;

use crate::{cell, Desktop, DRAG_ACTION, DROP_TYPE, files, folder, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::cell::DNDInfo;

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

    let drop_target = gtk::DropTarget::new(DROP_TYPE, DRAG_ACTION);

    let d = desktop.clone();
    drop_target.connect_drop(move |drop_target, value, x, y| {
        let drop_result = extract_from_variant(value);
        match drop_result {
            Ok(csp) => {
                if check_for_underlying_icon(d.borrow().as_ref(), x, y, csp.w, csp.h) {
                    return false;
                }
                let c = desktop_props_rc.clone();
                let desktop_props = c.borrow();
                let cell = desktop_props.cell_map.get(csp.path.as_str()).expect("Fatal: cannot find cell");
                d.borrow().move_(cell, x, y);
                files::save_settings(desktop_props, csp.path.as_str(), x, y);
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


fn extract_from_variant(v: &Value) -> Result<DNDInfo, Box<dyn Error>> {
    let variant = v.get::<Variant>()?;
    let c_s_p_opt = variant.get::<DNDInfo>();
    match c_s_p_opt {
        None => { Result::Err("no dnd data")? }
        Some(csp) => {
            Result::Ok(csp)
        }
    }
}

fn check_for_underlying_icon(d: &Fixed, x: f64, y: f64, w: f64, h: f64) -> bool {
    let p1x = x;
    let p1y = y;
    let widget_opt = d.pick(x, y, PickFlags::DEFAULT);
    match widget_opt {
        None => {
            panic!();
        }
        Some(underlying_icon) => {
            let widget_name = underlying_icon.type_().to_string();
            println!("{}",widget_name) ;
                return false;
        }
    }
}

fn draw_icons(entries: HashSet<files::DirItem>, desktop: &Fixed, width: i32, size: i32, memo_desktop: files::MemoDesktop) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let memo_icons = memo_desktop.icons;
    for entry in entries {
        let path_name = entry.path_name.clone();
        let cell = cell::make_cell(entry, size);
        let drag_source = make_drag_source(path_name.clone(), &cell, &desktop);
        cell.add_controller(drag_source);
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

fn make_drag_source(path_name : String, desktop_icon: &gtk::Box, layout: &Fixed) -> gtk::DragSource {

    let drag_source = gtk::DragSource::new();
    drag_source.set_actions(DRAG_ACTION);
    let path_copy = String::from(path_name.as_str());
    let l_clone = layout.clone();
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, x, y| {
            me.set_state(EventSequenceState::Claimed);
            let mut dnd_info  = DNDInfo::default();
            dnd_info.path = path_copy.to_string();
            let transform = l_clone.child_transform(&desktop_icon).expect("Fatal: cannot get layout.child_transform");
            let bounds = desktop_icon.compute_bounds(&desktop_icon).expect("Fatal: cannot get cell.compute_bounds");
            let rect = Rect::new(bounds.x(), bounds.y(), bounds.width(), bounds.height());
            let actual_pos = transform.transform_bounds(&rect);
            dnd_info.pos_x = actual_pos.x() as f64;
            dnd_info.pos_y = actual_pos.y() as f64;
            dnd_info.w = bounds.width() as f64;
            dnd_info.h = bounds.height() as f64;
            println!("{:?}", dnd_info);
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