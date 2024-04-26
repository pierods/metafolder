use gtk::{Popover, PositionType, Scale};
use gtk::prelude::{FixedExt, PopoverExt, RangeExt, ScaleExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::gtk_wrappers;

use crate::glib::clone;
use gtk::glib;

pub(crate) fn make_text_formatter() -> Popover {
    let sizes: [&str; 7] = ["xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large"];

    let text_size_adjustment = gtk::Adjustment::new(
        2.0,   // The value where the handle will be at the initial state
        0.0,   // Lower bound
        6.0, // Upper bound
        1.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        1.0,   // Page increment
        0.0,   // Page size
    );
    let text_scale = Scale::new(gtk::Orientation::Horizontal, Some(&text_size_adjustment));
    text_scale.add_mark(1f64, PositionType::Top, None);
    text_scale.add_mark(2f64, PositionType::Top, None);
    text_scale.add_mark(3f64, PositionType::Top, None);
    text_scale.add_mark(4f64, PositionType::Top, None);
    text_scale.add_mark(5f64, PositionType::Top, None);
    text_scale.add_mark(6f64, PositionType::Top, None);
    text_scale.add_mark(7f64, PositionType::Top, None);
    text_scale.add_mark(8f64, PositionType::Top, None);
    text_scale.set_round_digits(0);
    text_scale.set_width_request(300);
    text_scale.set_height_request(50);


    let grid = gtk::Fixed::builder().build();
    grid.put(&text_scale, 0f64, 0f64);

    let popover = Popover::builder().build();
    popover.set_size_request(300, 50);
    popover.set_child(Some(&grid));

    text_scale.connect_value_changed(clone!(@strong popover => move |s| {
        let ds = gtk_wrappers::get_application(&popover);
        print!("s.value={}, {} ", s.value() as usize, sizes[s.value() as usize]);
        ds.imp().metafolder.borrow().change_font_size(sizes[s.value() as usize]);
    }));

    popover
}