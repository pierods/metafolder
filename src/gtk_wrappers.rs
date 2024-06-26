use std::error::Error;
use gtk::prelude::{RangeExt};
use gtk::{ApplicationWindow, Fixed, gdk, glib, PickFlags};
use gtk::gdk::RGBA;
use gtk::glib::{Value, Variant};
use gtk::graphene::Rect;
use gtk::prelude::{Cast, FixedExt, GtkWindowExt, IsA, ObjectExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::app_with_datastore::AppWithDatastore;
use crate::cell::DNDInfo;
use crate::{CELL_SIZES, DEFAULT_BG_COLOR, FONT_SIZES};

pub fn is_something_underneath(name: String, d: &Fixed, x: f64, y: f64, w: f64, h: f64) -> bool {
    struct Point {
        x: f64,
        y: f64,
    }
    let points: [Point; 5] = [Point { x, y }, Point { x: x + w, y }, Point { x, y: y + h }, Point { x: x + w, y: y + h }, Point{x: x + w/2f64, y: y+ h/2f64}];
    for p in points {
        match d.pick(p.x, p.y, PickFlags::DEFAULT) {
            None => {
                println!("Unexpected: desktop.pick == None")
            }
            Some(underlying_icon) => {
                let widget_type = underlying_icon.type_().to_string();
                if widget_type != "GtkFixed" {
                    let img_name = underlying_icon.tooltip_text().unwrap();
                    if name == img_name {
                        return false;
                    }
                    return true;
                }
            }
        }
    }
    false
}

pub fn extract_from_variant(v: &Value) -> Result<DNDInfo, Box<dyn Error>> {
    let variant = v.get::<Variant>()?;
    let c_s_p_opt = variant.get::<DNDInfo>();
    match c_s_p_opt {
        None => { Result::Err("no dnd data")? }
        Some(csp) => {
            Result::Ok(csp)
        }
    }
}

pub fn get_widget_bounds(container: &Fixed, w: &gtk::Box) -> Rect {
    let transform_opt = container.child_transform(w);
    match transform_opt {
        Some(transform) => {
            let bounds = w.compute_bounds(w).expect("Fatal: cannot get cell.compute_bounds");
            let rect = Rect::new(bounds.x(), bounds.y(), bounds.width(), bounds.height());
            let actual_bounds = transform.transform_bounds(&rect);
            actual_bounds
        }
        None => {
            let bounds = w.compute_bounds(w).expect("Fatal: cannot get cell.compute_bounds");
            println!("Unexpected: cannot get Fixed.child_transform(icon) - container : {:?}, icon: {:?}, bounds: {:?}", container, w, bounds);
            bounds
        }
    }
}

pub fn get_application(w: &impl IsA<gtk::Widget>) -> AppWithDatastore {
    let root = w.root().unwrap();
    let app_window = root.downcast::<ApplicationWindow>().unwrap();
    let app = app_window.application().unwrap();
    let ds = app.downcast::<AppWithDatastore>().unwrap();
    ds
}

pub fn get_desktop(sw: &impl IsA<gtk::Widget>) -> Fixed {
    let root = sw.root().unwrap();
    let app_window = root.downcast::<gtk::ApplicationWindow>().unwrap();
    let scrolled_window = app_window.child().unwrap();
    let viewport = scrolled_window.first_child().unwrap();
    let fixed_widget = viewport.first_child().unwrap();
    let fixed = fixed_widget.downcast::<gtk::Fixed>().unwrap();
    fixed
}

pub fn set_cell_size_scale(w: AppWithDatastore, cell_size: i32) {
    //let app = get_application(w);
    let cell_size_scale_binding = w.imp().cell_size_scale.borrow();
    let cell_size_scale_opt = cell_size_scale_binding.as_ref();
    let cell_size_scale = cell_size_scale_opt.unwrap();
    let index = array_find(cell_size, CELL_SIZES);
    if index.is_none() {
        return;
    }
    cell_size_scale.set_value(index.unwrap() as f64);
}

fn array_find<T: Eq>(t :T, array: &[T]) -> Option<usize>{
        for i in 0..array.len() {
            if array[i] == t {
                return Some(i)
            }
        }
    None
}

pub fn set_font_size_scale(w: &impl IsA<gtk::Widget>, font_size: String) {
    let app = get_application(w);
    let font_size_scale_binding = app.imp().text_size_scale.borrow();
    let text_scale_opt = font_size_scale_binding.as_ref();
    let text_size_scale = text_scale_opt.unwrap();
    let index = array_find(font_size.as_str(), FONT_SIZES);
    if index.is_none() {
        return;
    }
    text_size_scale.set_value(index.unwrap() as f64);
}

pub fn set_font_bold_switch(w: AppWithDatastore, state: bool) {
    let binding = w.imp().font_bold_switch.borrow();
    let fb = binding.as_ref();
    let fbs = fb.unwrap();
    fbs.set_state(state);
    fbs.set_active(state);
}
pub fn set_font_color_button(w: &impl IsA<gtk::Widget>, font_color: String) {
    let app = get_application(w);
    let binding = app.imp().font_color_button.borrow();
    let fc = binding.as_ref();
    fc.unwrap().set_rgba(&RGBA::parse(font_color).unwrap());
}

pub fn set_window_background(rgba: String) {
    let color = String::from("window {background-color:").to_owned() + rgba.as_str() + "; border-radius: 7px;}";
    let provider = gtk::CssProvider::new();
    let bytes = glib::Bytes::from(color.as_bytes());
    provider.load_from_bytes(&bytes);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn alert(w: &impl IsA<gtk::Widget>, msg: String, err: String) {
    let alert = gtk::AlertDialog::builder().modal(true).detail(err).message(msg).build();
    let root = w.root().unwrap();
    let app_window: ApplicationWindow = root.downcast().unwrap();
    alert.show(Some(&app_window));
}

pub fn set_drilldown_switch(w: &impl IsA<gtk::Widget>, state: bool) {
    let app = get_application(w);
    let binding = app.imp().drilldown_switch.borrow();
    let dd = binding.as_ref();
    dd.unwrap().set_state(state);
    dd.unwrap().set_active(state);
}

pub fn set_bgcolor_button_color(w: &impl IsA<gtk::Widget>, mut color: String) {
    if color == "" {
        color = DEFAULT_BG_COLOR.to_string();
    }
    let app = get_application(w);
    let binding = app.imp().bg_color_button.borrow();
    let bg = binding.as_ref();
    bg.unwrap().set_rgba(&RGBA::parse(color).unwrap());
}

pub fn set_zoom_widgets(w: &impl IsA<gtk::Widget>, zoom: bool, zoom_x: i32, zoom_y: i32) {
    let app = get_application(w);
    let binding_zoom_button = app.imp().zoom_button.borrow();
    let zoom_button_opt = binding_zoom_button.as_ref();
    let zoom_button = zoom_button_opt.unwrap();
    if zoom {
        zoom_button.set_css_classes(&["folder-zoomed"]);
    } else {
        zoom_button.set_css_classes(&["folder-unzoomed"]);
    }
    let binding_zoom_x_scale = app.imp().zoom_x_scale.borrow();
    let zoom_x_scale_opt = binding_zoom_x_scale.as_ref();
    let zoom_x_scale = zoom_x_scale_opt.unwrap();
    zoom_x_scale.set_value(zoom_x as f64);

    let binding_zoom_y_scale = app.imp().zoom_y_scale.borrow();
    let zoom_y_scale_opt = binding_zoom_y_scale.as_ref();
    let zoom_y_scale = zoom_y_scale_opt.unwrap();
    zoom_y_scale.set_value(zoom_y as f64);
}

pub fn set_title_path(w: &impl IsA<gtk::Widget>, path: String) {
    let app = get_application(w);
    let binding = app.imp().path_label.borrow();
    let bg = binding.as_ref();
    bg.unwrap().set_label(&path);
}