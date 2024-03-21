use crate::glib::clone;
use std::collections::{HashMap, HashSet};
use std::path::{Path};
use std::fs;
use gio_sys::G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid, pango, Align, gdk, DragSource, WidgetPaintable, EventSequenceState, Fixed, gio};
use gtk::gdk::{ContentProvider, DragAction};
use gtk::gdk::ffi::gdk_content_provider_new_typed;
use gtk::gio::{Cancellable, FileQueryInfoFlags, Icon};
use gtk::glib::gobject_ffi::G_TYPE_CHAR;
use gtk::glib::{GStr, Value};


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

    let entries: HashSet<DirItem>;

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

fn draw_icons_on_desktop(entries: HashSet<DirItem>, desktop: &Fixed, width: i32) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();


    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for entry in entries {
        let size = 60;
        let cell = make_cell(entry.path_name.clone(), entry.is_dir, size);
        desktop.put(&cell, c as f64, r as f64);
        cell_map.insert(entry.path_name, cell);
        c += size;
        if c > width {
            c = 0;
            r += 2 * size;
        }
        //break;
    }
    cell_map
}

struct DesktopIcon {
    file_namepath: String,
    icon: gtk::Box,
    position_x: f64,
    position_y: f64,
}


fn make_cell(text: String, is_dir: bool, size: i32) -> gtk::Box {
    //let img = gtk::Image::from_file("asset.png");
    let img: gtk::Image;
    if is_dir {
        img = gtk::Image::from_icon_name("folder");
    } else {
        img = gtk::Image::from_icon_name("x-office-document");
    }

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
    drag_source.set_icon(Some(&w_p), 0, 0);
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

#[derive(Eq, Hash, PartialEq)]
struct DirItem {
    path_name: String,
    is_dir: bool,
    mime_type: Option<String>,
    icon: Option<gio::Icon>,
}

fn get_entries(p: String) -> HashSet<DirItem> {
    let mut entries: HashSet<DirItem> = HashSet::new();

    let paths = fs::read_dir(p).expect("Impossible to get your home dir!");

    for path in paths {
        let dir_entry = path.expect("Impossible to get your home dir!").path();
        let file_name_opt = dir_entry.file_name();
        match file_name_opt {
            Some(f) => {
                let file_name_string_opt = f.to_str();
                match file_name_string_opt {
                    Some(f) => if !f.starts_with(".") {
                        let (mime_opt, icon_opt) = get_file_info(dir_entry.as_path().to_str().expect("Fatal: get complete path").to_string());
                        if dir_entry.is_dir() {
                            entries.insert(DirItem { path_name: f.to_string(), is_dir: true, mime_type: Option::None, icon: Option::None });
                        } else {
                            entries.insert(DirItem { path_name: f.to_string(), is_dir: false, mime_type: mime_opt, icon: icon_opt });
                        }
                    },
                    None => panic!("Impossible to get your home dir!"),
                }
            }
            None => panic!("Impossible to get your home dir!"),
        }
    }
    entries
}


fn get_file_info(path_name: String) -> (Option<String>, Option<Icon>) {
    let g_file = gio::File::for_path(path_name.clone());
    let g_file_info_result = g_file.query_info("*", gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS,  Cancellable::NONE);
    let mime_opt: Option<glib::GString>;
    let g_icon_opt: Option<gio::Icon>;
    match g_file_info_result {
        Ok(file_info) => {
            println!("{}", file_info.size());
            g_icon_opt = file_info.icon();
            mime_opt = file_info.content_type();
        }
        Err(error) => {
            println!("{}", error);
            return (Option::None, Option::None);
        }
    }
    let icon: Option<Icon>;
    match g_icon_opt {
        Some(g_icon) => { icon = Option::Some(g_icon) }
        None => {
            println!("cannot find icon for {}", path_name);
            icon = Option::None
        }
    }
    let mime: Option<String>;
    match mime_opt {
        Some(gmime) => { mime = Some(gmime.as_str().to_string()) }
        None => {
            mime = Option::None;
            println!("cannot find mime type for  {}", path_name)
        }
    }
    (mime, icon)
}