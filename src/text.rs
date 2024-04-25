use gtk::{Popover, PositionType, Scale};
use gtk::prelude::{FixedExt, PopoverExt, RangeExt, ScaleExt, WidgetExt};

pub(crate) fn make_text_formatter() -> Popover {
    let text_size_adjustment = gtk::Adjustment::new(
        3.0,   // The value where the handle will be at the initial state
        1.0,   // Lower bound
        8.0, // Upper bound
        1.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        1.0,   // Page increment
        0.0,   // Page size
    );
    let text_scale = Scale::new(gtk::Orientation::Horizontal, Some(&text_size_adjustment));
    text_scale.add_mark(1f64, PositionType::Top, None);
    text_scale.set_round_digits(0);
    text_scale.set_width_request(300);
    text_scale.set_height_request(50);

    let grid = gtk::Fixed::builder().build();
    grid.put(&text_scale, 0f64, 0f64);

    let popover = Popover::builder().build();
    popover.set_size_request(300, 50);
    popover.set_child(Some(&grid));

    popover
}