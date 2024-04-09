use gtk::{GestureClick, Popover, PositionType, Scale};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::glib::clone;
use crate::gtk_wrappers;

pub(crate) fn make_zoom() -> Popover {
    let horizontal_adjustment = gtk::Adjustment::new(
        100.0,   // The value where the handle will be at the initial state
        50.0,   // Lower bound
        200.0, // Upper bound
        10.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        20.0,   // Page increment
        0.0,   // Page size
    );
    let zoomx = Scale::new(gtk::Orientation::Horizontal, Some(&horizontal_adjustment));
    zoomx.add_mark(100f64, PositionType::Top, None);
    zoomx.set_round_digits(0);
    zoomx.set_width_request(300);
    zoomx.set_height_request(50);

    let vertical_adjustment = gtk::Adjustment::new(
        100.0,   // The value where the handle will be at the initial state
        50.0,   // Lower bound
        200.0, // Upper bound
        10.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        20.0,   // Page increment
        0.0,   // Page size
    );
    let zoomy = gtk::Scale::new(gtk::Orientation::Vertical, Some(&vertical_adjustment));

    zoomy.add_mark(100f64, PositionType::Top, None);
    zoomy.set_round_digits(0);
    zoomy.set_width_request(50);
    zoomy.set_height_request(300);


    let grid = gtk::Fixed::builder().build();

    grid.put(&zoomy, 275f64, 0f64);
    grid.put(&zoomx, 0f64, 275f64);

    let popover = Popover::builder().build();
    popover.set_position(PositionType::Bottom);
    popover.set_size_request(300, 300);
    popover.set_child(Some(&grid));

    let commit = gtk::Button::builder().label("commit").build();
    commit.connect_clicked(clone!(@weak popover, @weak zoomx, @weak zoomy =>  move|_b| {
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().commit_zoom(&popover);
        zoomx.set_value(100f64);
        zoomy.set_value(100f64);
    }));
    grid.put(&commit, 0f64, 0f64);

    let zero = gtk::Button::builder().label("zero").build();
    zero.connect_clicked(clone!(@weak popover, @weak zoomx, @weak zoomy => move |_b| {
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().reset_zoom(&popover);
        zoomx.set_value(100f64);
        zoomy.set_value(100f64);
    }));

    grid.put(&zero, 100f64, 0f64);

    let gesture_click_y = GestureClick::new();
    gesture_click_y.connect_unpaired_release(clone!(@weak zoomx, @weak zoomy, @weak popover => move |_click, _, _, _, _|{
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().zoom(zoomx.value() as i32, zoomy.value() as i32, &popover);
    }));
    gesture_click_y.connect_stopped(clone!(@weak zoomx, @weak zoomy, @weak popover => move|_click| {
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().zoom(zoomx.value() as i32, zoomy.value() as i32, &popover);
    }));
    zoomy.add_controller(gesture_click_y);

    let gesture_click_x = GestureClick::new();
    gesture_click_x.connect_unpaired_release(clone!(@weak zoomx, @weak zoomy, @weak popover => move |_click, _, _, _, _|{
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().zoom(zoomx.value() as i32, zoomy.value() as i32, &popover);
    }));
    gesture_click_x.connect_stopped(clone!(@weak zoomx, @weak zoomy, @weak popover => move|_click| {
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().desktop.borrow_mut().zoom(zoomx.value() as i32, zoomy.value() as i32, &popover);
    }));
    zoomx.add_controller(gesture_click_x);

    popover
}

