use std::process::Command;

use gtk::{Align, ApplicationWindow, Fixed, GestureClick, glib, pango, WidgetPaintable};
use gtk::EventSequenceState;
use gtk::gdk::ContentProvider;
use gtk::glib::Value;
use gtk::prelude::{BoxExt, Cast, WidgetExt};
use gtk::prelude::GestureExt;
use gtk::prelude::ToVariant;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{DRAG_ACTION, files, folder, gtk_wrappers};
use crate::glib::clone;
use crate::gtk_wrappers::get_application;

#[derive(Default, Debug, PartialEq, glib::Variant)]
pub(crate) struct DNDInfo {
    pub(crate) name: String,
    pub(crate) w: f64,
    pub(crate) h: f64,
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
    pub(crate) grabbed_x: f64,
    pub(crate) grabbed_y: f64,
}

pub fn make_cell(path: String, dir_item: files::DirItem, size: i32) -> gtk::Box {
    let name = dir_item.name.clone();
    let img = generate_icon(path, &dir_item, size);
    img.set_tooltip_text(Some(dir_item.name.as_str()));
    img.set_halign(Align::Center);
    // img.set_valign(Align::Start);

    let g_text = glib::markup_escape_text(name.as_str());
    let pango_string = String::from("<span font_size=\"small\" font_weight=\"bold\"  color=\"white\">") + g_text.as_str() + "</span>";
    let label = gtk::Label::new(Option::Some(pango_string.as_str()));
    label.set_use_markup(true);
    label.set_ellipsize(pango::EllipsizeMode::End);
    label.set_wrap(true);
    label.set_wrap_mode(pango::WrapMode::WordChar);
    label.set_lines(2);
    label.set_justify(gtk::Justification::Center);
    label.set_halign(Align::Center);
    label.set_tooltip_text(Some(dir_item.name.as_str()));
    // txt.set_valign(Align::End);

    let desktop_icon = gtk::Box::new(gtk::Orientation::Vertical, 10);
    desktop_icon.set_tooltip_text(Some(dir_item.name.as_str()));
    desktop_icon.set_homogeneous(false);
    desktop_icon.set_spacing(3);
    desktop_icon.append(&img);
    desktop_icon.append(&label);
    let gesture_click = GestureClick::new();
    gesture_click.connect_pressed(clone!(@weak  desktop_icon => @default-return (), move |_, clicks, _, _| {
        if clicks == 2 {
            let data_store = get_application(&desktop_icon);
            let current_path = data_store.imp().desktop.borrow().get_current_path();
            if dir_item.mime_type == "inode/directory" {
                let app = get_application(&desktop_icon);
                let drilldown = app.imp().drilldown.borrow().as_ref().unwrap().state();
                if  drilldown{
                    let root = desktop_icon.root().unwrap();
                    let app_window_result = root.downcast::<ApplicationWindow>();
                    match app_window_result {
                        Ok(app_win) => {
                            folder::draw_folder(current_path + name.as_str() + "/", &app_win);
                            return
                        }
                        Err(r) => {
                            println!("{:?} is not an application window", r);
                            return
                        }
                    }
                }
            }
            match Command::new("xdg-open").args([current_path.clone() + name.as_str()]).output() {
                Ok(_) => {}
                Err(error) => { println!("error opening file {} : {}", current_path + name.as_str(), error) }
            }
        }
    }));
    desktop_icon.add_controller(gesture_click);
    desktop_icon
}

fn generate_icon(path: String, dir_item: &files::DirItem, size: i32) -> gtk::Image {
    let img: gtk::Image;

    if let Some(gicon) = &dir_item.icon {
        if dir_item.mime_type.starts_with("image") {
            img = gtk::Image::from_file(path.to_owned() + dir_item.name.as_str());
        } else {
            match dir_item.mime_type.as_str() {
                "application/pdf" => {
                    img = gtk::Image::from_gicon(gicon)
                }
                _ => img = gtk::Image::from_gicon(gicon)
            }
        }
    } else {
        img = gtk::Image::from_icon_name("x-office-document");
    }
    img.set_pixel_size(size);
    img
}

pub(crate) fn make_drag_source(name: String, desktop_icon: &gtk::Box, layout: &Fixed) -> gtk::DragSource {
    let drag_source = gtk::DragSource::new();
    drag_source.set_actions(DRAG_ACTION);
    let name_copy = String::from(name.as_str());
    let l_clone = layout.clone();
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, x, y| {
            me.set_state(EventSequenceState::Claimed);
            let mut dnd_info  = DNDInfo::default();
            dnd_info.name = name_copy.to_string();
            let actual_bounds = gtk_wrappers::get_widget_bounds(l_clone.as_ref(), &desktop_icon);
            dnd_info.pos_x = actual_bounds.x() as f64;
            dnd_info.pos_y = actual_bounds.y() as f64;
            dnd_info.w = actual_bounds.width() as f64;
            dnd_info.h = actual_bounds.height() as f64;
            dnd_info.grabbed_x = x;
            dnd_info.grabbed_y = y;
            let w_p = WidgetPaintable::new(Some(&desktop_icon));
            me.set_icon(Some(&w_p), x as i32, y as i32);
            // must coincide with DROP_TYPE
            Some(ContentProvider::for_value(&Value::from(dnd_info.to_variant())))
        })
    );
    drag_source
}