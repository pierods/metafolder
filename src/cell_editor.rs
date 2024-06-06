use gtk::{Label, Orientation, Popover, PositionType, Scale, Switch};
use gtk::prelude::{BoxExt, PopoverExt, RangeExt, ScaleExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{CELL_SIZES, FONT_SIZES, gtk_wrappers};
use gtk::prelude::FixedExt;
use gtk::glib::Propagation;
use crate::gtk_wrappers::alert;

pub(crate) fn make_cell_formatter() -> (Popover, Scale, Switch, Scale) {
    let grid = gtk::Fixed::builder().build();
    let popover = Popover::builder().build();
    popover.set_size_request(300, 50);
    popover.set_child(Some(&grid));

    let text_size_adjustment = gtk::Adjustment::new(
        2.0,   // The value where the handle will be at the initial state
        0.0,   // Lower bound
        6.0, // Upper bound
        1.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        1.0,   // Page increment
        0.0,   // Page size
    );
    let text_size_scale = Scale::new(gtk::Orientation::Horizontal, Some(&text_size_adjustment));
    text_size_scale.add_mark(0f64, PositionType::Top, None);
    text_size_scale.add_mark(1f64, PositionType::Top, None);
    text_size_scale.add_mark(2f64, PositionType::Top, None);
    text_size_scale.add_mark(3f64, PositionType::Top, None);
    text_size_scale.add_mark(4f64, PositionType::Top, None);
    text_size_scale.add_mark(5f64, PositionType::Top, None);
    text_size_scale.add_mark(6f64, PositionType::Top, None);
    text_size_scale.set_round_digits(0);
    text_size_scale.set_width_request(300);
    text_size_scale.set_height_request(50);
    text_size_scale.set_tooltip_text(Some("adjust text size"));

    text_size_scale.connect_change_value(move |scale, _, val| {
        let ds = gtk_wrappers::get_application(scale);
        ds.imp().metafolder.borrow().change_font_size(FONT_SIZES[val as usize].to_string(), true);
        Propagation::Proceed
    });
    let bold_container = gtk::Box::builder().orientation(Orientation::Horizontal).build();

    let bold_label = Label::builder().label("bold    ").build();
    bold_container.append(&bold_label);
    let bold_switch = Switch::builder().state(true).active(true).build();
    bold_switch.connect_state_set(move |sw, state| {
        if sw.state() == state {
            // prevent redoing of action from programmatically being set
            return Propagation::Stop;
        }
        let ds = gtk_wrappers::get_application(sw);
        if let Some(err) = ds.imp().metafolder.borrow().change_bold(state, true) {
            alert(sw, "folder settings could not be saved".to_string(), err.to_string());
            return Propagation::Stop;
        }
        return Propagation::Proceed;
    });
    bold_container.append(&bold_switch);

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
    cell_size_scale.set_tooltip_text(Some("adjust cell size"));

    cell_size_scale.connect_change_value(move |scale, _, val| {
        let ds = gtk_wrappers::get_application(scale);
        let size = CELL_SIZES[val as usize];
        ds.imp().metafolder.borrow().change_cell_size(size, true);
        Propagation::Proceed
    });
    grid.put(&text_size_scale, 0f64, 0f64);
    grid.put(&bold_container, 0f64, 50f64);
    grid.put(&cell_size_scale, 0f64, 100f64);
    (popover, text_size_scale, bold_switch, cell_size_scale)
}