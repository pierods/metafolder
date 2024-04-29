use gtk::{Label, Orientation, Popover, PositionType, Scale, Switch};
use gtk::prelude::{BoxExt, PopoverExt, RangeExt, ScaleExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{FONT_SIZES, gtk_wrappers};

use crate::glib::clone;
use gtk::glib::Propagation;
use crate::gtk_wrappers::alert;

pub(crate) fn make_cell_formatter() -> (Popover, Scale, Switch, Scale) {

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


    //let container = gtk::Fixed::builder().build();
    let container = gtk::Box::builder().orientation(Orientation::Vertical).build();
    container.append(&text_scale);

    let popover = Popover::builder().build();
    popover.set_size_request(300, 50);
    popover.set_child(Some(&container));

    text_scale.connect_value_changed(clone!(@strong popover => move |s| {
        let ds = gtk_wrappers::get_application(&popover);
        ds.imp().metafolder.borrow().change_font_size(FONT_SIZES[s.value() as usize].to_string(), true);
    }));

    let bold_container = gtk::Box::builder().orientation(Orientation::Horizontal).build();

    let bold_label = Label::builder().label("bold    ").build();
    bold_container.append(&bold_label);
    let bold_switch = Switch::builder().state(true).active(true).build();
    bold_switch.connect_state_set(move |sw, state| {
        let ds = gtk_wrappers::get_application(sw);
        if let Some(err) = ds.imp().metafolder.borrow().change_bold(state, true) {
            alert(sw, "folder settings could not be saved".to_string(), err.to_string());
            return Propagation::Stop
        }
        return Propagation::Proceed;
    });
    bold_container.append(&bold_switch);
    container.append(&bold_container);

    let cell_size_adjustment = gtk::Adjustment::new(
        1.0,   // The value where the handle will be at the initial state
        0.0,   // Lower bound
        2.0, // Upper bound
        1.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        1.0,   // Page increment
        0.0,   // Page size
    );
    let cell_size_scale = Scale::new(gtk::Orientation::Horizontal, Some(&cell_size_adjustment));
    cell_size_scale.add_mark(0f64, PositionType::Top, None);
    cell_size_scale.add_mark(1f64, PositionType::Top, None);
    cell_size_scale.add_mark(2f64, PositionType::Top, None);
    cell_size_scale.set_round_digits(0);
    cell_size_scale.set_width_request(300);
    cell_size_scale.set_height_request(50);

    cell_size_scale.connect_value_changed(|s| {
        let ds = gtk_wrappers::get_application(s);
        ds.imp().metafolder.borrow().change_cell_size(s.value() as i32, true);
    });
    container.append(&cell_size_scale);
    (popover, text_scale, bold_switch, cell_size_scale)
}