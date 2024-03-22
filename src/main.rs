use std::cell::{Ref, RefCell, RefMut};
use crate::glib::clone;
use std::collections::{HashMap, HashSet};
use std::path::{Path};
use std::fs;
use gio_sys::G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid, pango, Align, gdk, DragSource, WidgetPaintable, EventSequenceState, Fixed, gio};
use gtk::gdk::{ContentProvider, DragAction};
use gtk::gdk::ffi::gdk_content_provider_new_typed;
use gtk::gio::{Cancellable, FileInfo, FileQueryInfoFlags, FileType, Icon};
use gtk::glib::gobject_ffi::G_TYPE_CHAR;
use gtk::glib::{GStr, Value};
use home::env::home_dir_with_env;
use std::rc::{Rc};
use serde::{Serialize, Deserialize};


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

    let entries: HashSet<DirItem>;
    let desktop_props_rc: Rc<RefCell<Desktop>> = Rc::new(RefCell::new(Desktop::default()));
    let c = desktop_props_rc.clone();
    let mut desktop_props = c.borrow_mut();

    desktop_props.path_name = home_path();
    if try_desktop(desktop_props.path_name.as_str()) {
        desktop_props.path_name += "/Desktop";
    }
    entries = get_entries(desktop_props.path_name.clone());

    let desktop = gtk::Fixed::new();
    desktop_props.cell_map = draw_icons_on_desktop(entries, &desktop, 1500, ICON_SIZE);

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
                save_settings(desktop_props);
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

fn draw_icons_on_desktop(entries: HashSet<DirItem>, desktop: &Fixed, width: i32, size: i32) -> HashMap<String, gtk::Box> {
    let mut cell_map: HashMap<String, gtk::Box> = HashMap::new();

    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for entry in entries {
        let path_name = entry.path_name.clone();
        let cell = make_cell(entry, size);
        desktop.put(&cell, c as f64, r as f64);
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

struct DesktopIcon {
    file_name: String,
    file_namepath: String,
    icon: gtk::Box,
    position_x: f64,
    position_y: f64,
}


fn make_cell(dir_item: DirItem, size: i32) -> gtk::Box {
    let path_name = dir_item.path_name.clone();
    let name = dir_item.name.clone();
    let img = generate_icon(dir_item, size);

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
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, _, _| {
            me.set_state(EventSequenceState::Claimed);
            Some(ContentProvider::for_value(&Value::from(path_name.clone())))
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
                    Some(f) => {
                        let dir_item_opt = get_file_info(dir_entry.as_path().to_str().expect("Fatal: get complete path").to_string());
                        match dir_item_opt {
                            Some(mut dir_item) => {
                                entries.insert(dir_item);
                            }
                            None => continue
                        }
                    }
                    None => panic!("Impossible to get your home dir!"),
                }
            }
            None => panic!("Impossible to get your home dir!"),
        }
    }
    entries
}

#[derive(Eq, Hash, PartialEq, Default)]
struct DirItem {
    name: String,
    path_name: String,
    is_dir: bool,
    mime_type: String,
    icon: Option<gio::Icon>,
}


fn get_file_info(path_name: String) -> Option<DirItem> {
    let mut dir_item: DirItem = DirItem::default();

    dir_item.path_name = path_name.clone();

    let g_file = gio::File::for_path(path_name.clone());

    let g_file_info_result = g_file.query_info("*", gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS, Cancellable::NONE);
    let g_file_info: FileInfo;
    match g_file_info_result {
        Ok(file_info) => {
            g_file_info = file_info;
        }
        Err(error) => {
            println!("{}", error);
            return Option::None;
        }
    }

    if g_file_info.is_hidden() {
        return Option::None;
    };
    dir_item.name = g_file_info.name().to_str().expect("Fatal: gio cannot get path").to_string();
    if g_file_info.file_type() == FileType::Directory {
        dir_item.is_dir = true;
    }
    match g_file_info.icon() {
        Some(g_icon) => { dir_item.icon = Option::Some(g_icon) }
        None => {
            println!("cannot find icon for {}", path_name);
            dir_item.icon = Option::None
        }
    }
    match g_file_info.content_type() {
        Some(gmime) => { dir_item.mime_type = gmime.as_str().to_string() }
        None => {
            dir_item.mime_type = String::from("");
            println!("cannot find mime type for  {}", path_name)
        }
    }
    Some(dir_item)
}


fn generate_icon(dir_item: DirItem, size: i32) -> gtk::Image {
    let img: gtk::Image;
    println!("{}", dir_item.mime_type);
    if dir_item.is_dir {
        img = gtk::Image::from_icon_name("folder");
    } else {
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
    }

    img.set_pixel_size(size);
    img
}

#[derive(Eq, Hash, PartialEq, Default, Serialize, Deserialize, Debug)]
struct MemoIcon {
    file_name: String,
    position_x: i32,
    position_y: i32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct MemoDesktop {
    path_name: String,
    background_color: String,
    icons: HashSet<MemoIcon>,
}


fn save_settings(desktop_props: Ref<Desktop>) {
    let mut memo_desktop = MemoDesktop::default();
    let mut icons :HashSet<MemoIcon> = HashSet::new();

    memo_desktop.path_name = desktop_props.path_name.clone();
    memo_desktop.background_color = desktop_props.background_color.clone();
    for (path, gbox) in desktop_props.cell_map.clone() {
        let allocation = gbox.allocation();
        let memo_icon =  MemoIcon{
            file_name: path,
            position_x: allocation.x(),
            position_y: allocation.y(),
        };
        icons.insert(memo_icon);
    }
    memo_desktop.icons =icons;

    let serialized = serde_json::to_string(&memo_desktop).unwrap();
    println!("{}", serialized)
}