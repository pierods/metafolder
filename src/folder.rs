use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::rc::Rc;

use gtk::{Align, ApplicationWindow, EventSequenceState, Fixed, GestureClick, glib, pango, WidgetPaintable};
use gtk::gdk::ContentProvider;
use gtk::glib::Value;
use gtk::prelude::{BoxExt, FixedExt, WidgetExt};
use gtk::prelude::GestureExt;
use gtk::prelude::GtkWindowExt;

use crate::{Desktop, DRAG_ACTION, files, folder, ICON_SIZE, INITIAL_DESKTOP_WIDTH};
use crate::glib::clone;

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

pub(crate) fn draw_icons(entries: HashSet<files::DirItem>, desktop: &Fixed, width: i32, size: i32, memo_desktop: files::MemoDesktop) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let memo_icons = memo_desktop.icons;
    for entry in entries {
        let path_name = entry.path_name.clone();
        let cell = folder::make_cell(entry, size);
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

pub fn make_cell(dir_item: files::DirItem, size: i32) -> gtk::Box {
    let path_name = dir_item.path_name.clone();
    let name = dir_item.name.clone();
    let img = generate_icon(dir_item, size);

    //let double_click_controller = gtk::
    let g_text = glib::markup_escape_text(name.as_str());
    let pango_string = String::from("<span font_size=\"small\">") + g_text.as_str() + "</span>";
    let label = gtk::Label::new(Option::Some(pango_string.as_str()));
    label.set_use_markup(true);
    label.set_ellipsize(pango::EllipsizeMode::End);
    label.set_wrap(true);
    label.set_wrap_mode(pango::WrapMode::WordChar);
    label.set_lines(2);
    label.set_justify(gtk::Justification::Center);

    label.set_halign(Align::Center);
    // txt.set_valign(Align::End);
    img.set_halign(Align::Center);
    // img.set_valign(Align::Start);

    let desktop_icon = gtk::Box::new(gtk::Orientation::Vertical, 10);
    desktop_icon.set_homogeneous(false);
    desktop_icon.set_spacing(3);
    desktop_icon.append(&img);
    desktop_icon.append(&label);

    let drag_source = gtk::DragSource::new();
    drag_source.set_actions(DRAG_ACTION);
    let path_copy = String::from(path_name.as_str());
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, x, y| {
            me.set_state(EventSequenceState::Claimed);
            Some(ContentProvider::for_value(&Value::from(&path_copy)))
        })
    );
    let w_p = WidgetPaintable::new(Some(&desktop_icon));
    //TODO hot_x, hot_y
    drag_source.set_icon(Some(&w_p), 0, 0);
    desktop_icon.add_controller(drag_source);
    desktop_icon.add_controller(clicked(String::from(path_name)));

    desktop_icon
}

fn generate_icon(dir_item: files::DirItem, size: i32) -> gtk::Image {
    let img: gtk::Image;
    //println!("{}", dir_item.mime_type);

    if let Some(gicon) = dir_item.icon {
        if dir_item.mime_type.starts_with("image") {
            img = gtk::Image::from_file(dir_item.path_name.clone());
        } else {
            match dir_item.mime_type.as_str() {
                "application/pdf" => {
                    img = gtk::Image::from_gicon(&gicon)
                }
                _ => img = gtk::Image::from_gicon(&gicon)
            }
        }
    } else {
        img = gtk::Image::from_icon_name("x-office-document");
    }
    img.set_pixel_size(size);
    img
}

fn clicked(path_name: String) -> GestureClick {
    let gesture_click = GestureClick::new();
       gesture_click.connect_pressed(move |_, clicks, _, _| {
        if clicks == 2 {
            match Command::new("xdg-open").args([path_name.clone()]).output() {
                Ok(_) => {}
                Err(error) => {println!("{}", error)}
            }
        }
    });
    gesture_click
}