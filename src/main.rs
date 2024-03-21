use crate::glib::clone;
use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid, pango, Align, gdk, DragSource, WidgetPaintable, EventSequenceState, Fixed};
use gtk::gdk::{ContentProvider, DragAction};
use gtk::gdk::ffi::gdk_content_provider_new_typed;
use gtk::glib::gobject_ffi::G_TYPE_CHAR;
use gtk::glib::Value;


const APP_ID: &str = "org.github.pierods.metafolder";
const DRAG_ACTION: DragAction = DragAction::MOVE;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("metafolder").build();
    window.set_default_size(1024, 768);
    //window.(0.0);
    window.connect_maximized_notify(|win: &ApplicationWindow| { println!("*****************************{}", win.width()) });
    window.maximize();
    window.present();

    let entries: HashMap<String, String>;

    let home = home_path();
    if try_desktop(home.as_str()) {
        entries = get_entries(home + "/Desktop");
    } else {
        entries = get_entries(home);
    }
    let desktop = gtk::Fixed::new();
    let cell_map = draw_icons_on_desktop(entries, &desktop, 1500);

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child(Option::Some(&desktop));


    window.set_child(Option::Some(&scrolled_window));

    let drop_target = gtk::DropTarget::new(glib::types::Type::OBJECT, DRAG_ACTION);
    drop_target.set_types(&[glib::types::Type::STRING]);
    drop_target.connect_drop(move |window, value, x, y| {
        let drop = value.get::<&str>();
        match drop {
            Ok(lab) => {
                println!("{:?}, {}, {}", lab, x, y);
                let cell = cell_map.get(lab).expect("Fatal: cannot find cell");
                desktop.move_(cell, x, y);
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

fn draw_icons_on_desktop(entries: HashMap<String, String>, desktop: &Fixed, width: i32) -> HashMap<String, gtk::Box> {
    let mut cell_map:HashMap<String, gtk::Box> = HashMap::new();


    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for (k, _v) in entries {
        let size = 60;
        let cell = make_cell(k.clone(), size);
        desktop.put(&cell, c as f64, r as f64);
        cell_map.insert(k, cell);
        c += size;
        if c > width {
            c = 0;
            r += 2* size;
        }
        //break;
    }
    cell_map
}

struct DesktopIcon {
    file_name: String,
    file_path: String,
    icon : gtk::Box,
}


fn make_cell(text: String, size: i32) -> gtk::Box {
    //let img = gtk::Image::from_file("asset.png");
    let img = gtk::Image::from_icon_name("folder");
    img.set_pixel_size(size);

    let g_text = glib::markup_escape_text(text.as_str());
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
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, _, _| {
            me.set_state(EventSequenceState::Claimed);
            Some(ContentProvider::for_value(&Value::from(text.clone())))
        })
    );
    let w_p = WidgetPaintable::new(Some(&desktop_icon));
    drag_source.set_icon(Some(&w_p), 0,0);
    desktop_icon.add_controller(drag_source);

    desktop_icon
}

fn try_desktop(home_path: &str) -> bool {
    let desktop_path = home_path.to_string() + "/Desktop";
    Path::new(desktop_path.as_str()).exists()
}

fn home_path() -> String {
    match home::home_dir() {
        Some(pb) => {
            match pb.to_str() {
                Some(s) => String::from(s),
                None => panic!("Impossible to get your home dir!"),
            }
        }
        None => panic!("Impossible to get your home dir!"),
    }
}

fn get_entries(p: String) -> HashMap<String, String> {
    let mut entries: HashMap<String, String> = HashMap::new();

    let paths = fs::read_dir(p).expect("Impossible to get your home dir!");

    for path in paths {
        let dir_entry = path.expect("Impossible to get your home dir!").path();
        let file_name_opt = dir_entry.file_name();
        match file_name_opt {
            Some(f) => {
                let file_name_string_opt = f.to_str();
                match file_name_string_opt {
                    Some(f) => if !f.starts_with(".") {
                        entries.insert(f.to_string(), f.to_string());
                    },
                    None => panic!("Impossible to get your home dir!"),
                }
            }
            None => panic!("Impossible to get your home dir!"),
        }
    }
    entries
}

