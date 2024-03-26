use gtk::{Align, EventSequenceState, glib, pango, WidgetPaintable};
use gtk::gdk::ContentProvider;
use gtk::glib::Value;
use gtk::prelude::{BoxExt, WidgetExt};
use gtk::prelude::GestureExt;

use crate::{DRAG_ACTION, files};
use crate::glib::clone;

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
    drag_source.connect_prepare(
        clone!(@weak  desktop_icon => @default-return None, move |me, _, _| {
            me.set_state(EventSequenceState::Claimed);
            Some(ContentProvider::for_value(&Value::from(path_name.clone())))
        })
    );
    let w_p = WidgetPaintable::new(Some(&desktop_icon));
    //TODO hot_x, hot_y
    drag_source.set_icon(Some(&w_p), 0, 0);
    desktop_icon.add_controller(drag_source);

    desktop_icon
}

fn generate_icon(dir_item: files::DirItem, size: i32) -> gtk::Image {
    let img: gtk::Image;
    //println!("{}", dir_item.mime_type);
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