use crate::glib::clone;
use gtk::{Align, GestureClick, WidgetPaintable, glib, pango, EventSequenceState, gdk::ContentProvider, glib::Value, prelude::GestureExt, graphene::Point};
use std::process::Command;
use gtk::prelude::{BoxExt, ToVariant, WidgetExt};
use crate::{DRAG_ACTION, files};

#[derive(Default, Debug, PartialEq, glib::Variant)]
pub(crate) struct DNDInfo {
    pub(crate) path: String,
    pub(crate) w : f64,
    pub(crate) h : f64,
    pub(crate) pos_x: f64,
    pub(crate) pos_y: f64,
}

pub fn make_cell(dir_item: files::DirItem, size: i32) -> gtk::Box {
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
    desktop_icon.add_controller(make_clicked_controller(String::from(path_name)));

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

fn make_clicked_controller(path_name: String) -> GestureClick {
    let gesture_click = GestureClick::new();
    gesture_click.connect_pressed(move |_, clicks, _, _| {
        if clicks == 2 {
            match Command::new("xdg-open").args([path_name.clone()]).output() {
                Ok(_) => {}
                Err(error) => { println!("{}", error) }
            }
        }
    });
    gesture_click
}
