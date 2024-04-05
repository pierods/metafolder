use std::fmt::Display;
use gtk::{ColorDialog, ColorDialogButton, gdk, glib, HeaderBar};
use gtk::gdk::RGBA;
use gtk::gio::MenuModel;
use gtk::glib::Propagation;
use gtk::prelude::{ButtonExt, Cast, ToValue, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{files, gtk_wrappers};
use crate::folder::draw_folder;

pub(crate) fn build_menu(menubar: Option<MenuModel>) {}

pub(crate) fn make_header_bar() -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {
        let ds = gtk_wrappers::get_application(b);
        let current_path = <std::cell::RefCell<String> as Clone>::clone(&ds.imp().current_path).into_inner();
        match files::up(&current_path) {
            None => {}
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
        let mut metafolder = ds.imp().desktop.take();
        metafolder.drilldown = state;
        ds.imp().desktop.replace(metafolder);
        Propagation::Proceed
    });
    bar.pack_start(&drilldown_switch);

    bar.pack_start(&gtk::Label::builder().label("memorize").build());
    let memorize_switch = gtk::Switch::builder().state(true).active(true).build();
    bar.pack_start(&memorize_switch);

    bar.pack_start(&gtk::Label::builder().label("zoom").build());

    let horizontal_adjustment = gtk::Adjustment::new(
        100.0,   // The value where the handle will be at the initial state
        0.0,   // Lower bound
        200.0, // Upper bound
        10.0,   // Step increment, keep it 0 if you don't want it to be operated by arrow keys
        0.0,   // Page increment
        0.0,   // Page size
    );
    let zoom = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&horizontal_adjustment));
    zoom.set_width_request(300);
    bar.pack_start(&zoom);

    let commit = gtk::Button::builder().label("commit").build();
    bar.pack_start(&commit);

    let background_dialog = ColorDialog::builder().modal(true).title("Pick a background color").with_alpha(true).build();
    let background_color = ColorDialogButton::builder().rgba(&RGBA::new(80f32, 80f32, 80f32, 255f32)).dialog(&background_dialog).build();
    background_color.connect_rgba_notify(|cdb| {
        let color = String::from("window {background-color:").to_owned() + cdb.rgba().to_string().as_str() + "; border-radius: 7px;}";
        let provider = gtk::CssProvider::new();
        let bytes = glib::Bytes::from(color.as_bytes());
        provider.load_from_bytes(&bytes);
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let ds = gtk_wrappers::get_application(cdb);
        let mut  metafolder = ds.imp().desktop.take();
        metafolder.background_color = color;
        ds.imp().desktop.replace(metafolder);
    });
    bar.pack_start(&background_color);

    let text_color_dialog = ColorDialog::builder().modal(true).title("Pick a text color").with_alpha(true).build();
    let text_color = ColorDialogButton::builder().rgba(&RGBA::new(255f32, 255f32, 255f32, 255f32)).dialog(&text_color_dialog).build();
    bar.pack_start(&text_color);

    bar
}