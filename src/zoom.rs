use gtk::{Popover, PositionType, Scale};
use gtk::prelude::*;

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

    let folder = gtk::Image::builder().icon_name("folder").build();
    grid.put(&folder, 0f64, 0f64);
    popover
}