use std::error::Error;

use gtk::{ApplicationWindow, Fixed, PickFlags};
use gtk::glib::{Value, Variant};
use gtk::graphene::Rect;
use gtk::prelude::{Cast, FixedExt, GtkWindowExt, IsA, ObjectExt, WidgetExt};

use crate::appwindow_with_datastore::AppWithDatastore;
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
            //println!("{:?}", actual_bounds);
            actual_bounds
        }
        None => {
            let bounds = w.compute_bounds(w).expect("Fatal: cannot get cell.compute_bounds");
            println!("Unexpected: cannot get Fixed.child_transform(icon) - container : {:?}, icon: {:?}, bounds: {:?}", container, w, bounds);
            bounds
        }
    }
}

pub fn get_application(sw : & impl IsA<gtk::Widget>) -> AppWithDatastore {

    let root = sw.root().unwrap();
    let app_window = root.downcast::<ApplicationWindow>().unwrap();
    let app = app_window.application().unwrap();
    let ds = app.downcast::<AppWithDatastore>().unwrap();
    ds
}