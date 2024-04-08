use std::error::Error;

use gtk::{ApplicationWindow, Fixed, gdk, glib, PickFlags};
use gtk::glib::{Value, Variant};
use gtk::graphene::Rect;
use gtk::prelude::{Cast, FixedExt, GtkWindowExt, IsA, ObjectExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::app_with_datastore::AppWithDatastore;
use crate::cell::DNDInfo;

pub fn is_something_underneath(d: &Fixed, x: f64, y: f64, w: f64, h: f64) -> bool {
    struct Point {
        x: f64,
        y: f64,
    }
    let points: [Point; 4] = [Point { x, y }, Point { x: x + w, y }, Point { x, y: y + h }, Point { x: x + w, y: y + h }];
    for p in points {
        let widget_opt = d.pick(p.x, p.y, PickFlags::DEFAULT);
        match widget_opt {
            None => {
                panic!();
            }
            Some(underlying_icon) => {
                let widget_type = underlying_icon.type_().to_string();
                if widget_type != "GtkFixed" {
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

// pub fn get_desktop(sw : & impl IsA<gtk::Widget>) -> Fixed {
//
//     let root = sw.root().unwrap();
//     let app_window= root.downcast::<gtk::ApplicationWindow>().unwrap();
//     let scrolled_window = app_window.child().unwrap();
//     let viewport = scrolled_window.first_child().unwrap();
//     let fixed_widget = viewport.first_child().unwrap();
//     let fixed = fixed_widget.downcast::<gtk::Fixed>().unwrap();
//     fixed
// }

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

pub fn set_drilldown_switch_value(w: &impl IsA<gtk::Widget>, state: bool) {
    let  app = get_application(w);
    let binding = app.imp().drilldown.borrow();
    let dd = binding.as_ref();
    dd.unwrap().set_state(state);
    dd.unwrap().set_active(state);
}