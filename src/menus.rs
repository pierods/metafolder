use gtk::glib;
use crate::glib::clone;
use gtk::{ApplicationWindow, ColorDialog, ColorDialogButton, HeaderBar};
use gtk::gdk::RGBA;
use gtk::glib::Propagation;

use gtk::prelude::{ButtonExt, Cast, PopoverExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{DEFAULT_BG_COLOR, files, gtk_wrappers, zoom};
use crate::folder::draw_folder;
use crate::gtk_wrappers::alert;

pub(crate) fn make_header_bar(app_window: &ApplicationWindow) -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {
        let ds = gtk_wrappers::get_application(b);
        let current_path = ds.imp().desktop.borrow().get_current_path();
        match files::up(&current_path) {
            None => { alert(b, "nowhere to go".to_string(), "".to_string()); }
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
            None => { Propagation::Proceed }
            Some(err) => {
                alert(sw, "folder settings could not be saved".to_string(), err.to_string());
                Propagation::Stop
            }
        };
        x
    });
    bar.pack_start(&drilldown_switch);

    bar.pack_start(&gtk::Label::builder().label("memorize").build());
    let memorize_switch = gtk::Switch::builder().state(true).active(true).build();
    bar.pack_start(&memorize_switch);

    let zoom_button = gtk::Button::builder().icon_name("folder").build();

    bar.pack_start(&zoom_button);
    let popover = zoom::make_zoom();
    popover.connect_closed(clone!(@weak zoom_button => move |_| {
        zoom_button.set_child(Some(&gtk::Image::builder().icon_name("folder").build()));
    }));

    zoom_button.connect_clicked(move |b| {
        b.set_child(Some(&popover));
        popover.set_visible(true);
    });

    let background_dialog = ColorDialog::builder().modal(true).title("Pick a background color").with_alpha(true).build();
    let background_color_button = ColorDialogButton::builder().rgba(&RGBA::parse(DEFAULT_BG_COLOR).unwrap()).dialog(&background_dialog).build();
    background_color_button.connect_rgba_notify(|cdb| {
        let ds = gtk_wrappers::get_application(cdb);
        match ds.imp().desktop.borrow_mut().update_background_color(cdb.rgba().to_string()) {
            None => { gtk_wrappers::set_window_background(cdb.rgba().to_string()); }
            Some(err) => {
                alert(cdb, "folder settings could not be saved".to_string(), err.to_string());
            }
        };
    });

    bar.pack_start(&background_color_button);

    let text_color_dialog = ColorDialog::builder().modal(true).title("Pick a text color").with_alpha(true).build();
    let text_color = ColorDialogButton::builder().rgba(&RGBA::parse(DEFAULT_BG_COLOR).unwrap()).dialog(&text_color_dialog).build();
    bar.pack_start(&text_color);

    let ds = gtk_wrappers::get_application(app_window);
    ds.imp().drilldown.replace(Some(drilldown_switch));
    ds.imp().bg_color.replace(Some(background_color_button));
    bar
}