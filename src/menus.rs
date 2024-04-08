use gtk::{ApplicationWindow, ColorDialog, ColorDialogButton, GestureClick, HeaderBar, Popover, PositionType};
use gtk::gdk::RGBA;
use gtk::glib::{Propagation};
use glib::clone;
use gtk::prelude::{AdjustmentExt, ButtonExt, Cast, PopoverExt, RangeExt, ScaleExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::{files, gtk_wrappers};
use crate::folder::draw_folder;
use crate::gtk_wrappers::{alert, set_drilldown_switch_value};
use gtk::glib;


//pub(crate) fn build_menu(_menubar: Option<MenuModel>) {}

pub(crate) fn make_header_bar(app_window: &ApplicationWindow) -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {
        let ds = gtk_wrappers::get_application(b);
        let current_path = ds.imp().desktop.borrow().get_current_path();
        match files::up(&current_path) {
            None => {alert(b, "nowhere to go".to_string(), "".to_string());}
            Some(up) => {
                let root = b.root().unwrap();
                let app_window = root.downcast::<gtk::ApplicationWindow>().unwrap();
                draw_folder(up, &app_window)
            }
        }
    });
    bar.pack_start(&up);

    bar.pack_start(&gtk::Label::builder().label("drilldown").build());
    let drilldown_switch = gtk::Switch::builder().state(true).active(true).build();
    drilldown_switch.connect_state_set(|sw, state| {
        let ds = gtk_wrappers::get_application(sw);
        let x = match ds.imp().desktop.borrow_mut().set_drilldown(state) {
            None => {
                set_drilldown_switch_value(sw, state);
                Propagation::Proceed}
            Some(err) => {
                alert(sw, "folder settings could not be saved".to_string(), err.to_string());
                Propagation::Stop
            }
        }; x

    });
    bar.pack_start(&drilldown_switch);

    bar.pack_start(&gtk::Label::builder().label("memorize").build());
    let memorize_switch = gtk::Switch::builder().state(true).active(true).build();
    bar.pack_start(&memorize_switch);


    let horizontal_adjustment = gtk::Adjustment::new(
        100.0,   // The value where the handle will be at the initial state
        50.0,   // Lower bound
        200.0, // Upper bound
        10.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        20.0,   // Page increment
        0.0,   // Page size
    );
    let zoom = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&horizontal_adjustment));
    zoom.add_mark(100f64, PositionType::Top, None);
    zoom.set_round_digits(0);
    zoom.set_width_request(300);
    let gesture_click = GestureClick::new();
    gesture_click.connect_unpaired_release(glib::clone!(@weak  zoom => @default-return (), move |_, _, _, _, _| {
        println!("{:?}", zoom.value());
    }));
    zoom.add_controller(gesture_click);
    let popover = Popover::builder().build();
    popover.set_child(Some(&zoom));


    let zoom_toggle = gtk::ToggleButton::builder().label("zoom").child(&popover).build();
    bar.pack_start(&zoom_toggle);

    let commit = gtk::Button::builder().label("commit").build();
    bar.pack_start(&commit);

    let background_dialog = ColorDialog::builder().modal(true).title("Pick a background color").with_alpha(true).build();
    let background_color = ColorDialogButton::builder().rgba(&RGBA::new(150f32, 150f32, 150f32, 255f32)).dialog(&background_dialog).build();
    background_color.connect_rgba_notify(|cdb| {
        let ds = gtk_wrappers::get_application(cdb);
        match ds.imp().desktop.borrow_mut().update_background_color(cdb.rgba().to_string()) {
            None => { gtk_wrappers::set_window_background(cdb.rgba().to_string()); }
            Some(err) => {
                alert(cdb, "folder settings could not be saved".to_string(), err.to_string());
            }
        };
    });
    bar.pack_start(&background_color);

    let text_color_dialog = ColorDialog::builder().modal(true).title("Pick a text color").with_alpha(true).build();
    let text_color = ColorDialogButton::builder().rgba(&RGBA::new(255f32, 255f32, 255f32, 255f32)).dialog(&text_color_dialog).build();
    bar.pack_start(&text_color);

    let ds = gtk_wrappers::get_application(app_window);
    ds.imp().drilldown.replace(Some(drilldown_switch));
    println!("ciao");
    bar
}