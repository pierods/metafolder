use gtk::gio::MenuModel;
use gtk::glib::Propagation;
use gtk::HeaderBar;
use gtk::prelude::{ButtonExt, Cast, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{files, gtk_wrappers};
use crate::folder::draw_folder;

pub(crate) fn build_menu(menubar: Option<MenuModel>) {}

pub(crate) fn make_header_bar() -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {
        let ds = gtk_wrappers::get_application(b);
        let current_path = <std::cell::RefCell<std::string::String> as Clone>::clone(&ds.imp().current_path).into_inner();
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
    let drilldown_switch = gtk::Switch::new();

    bar.pack_start(&gtk::Label::builder().label("drilldown").build());
    bar.pack_start(&drilldown_switch);

    bar.pack_start(&gtk::Label::builder().label("memorize").build());
    let memorize_switch = gtk::Switch::new();
    bar.pack_start(&memorize_switch);

    drilldown_switch.connect_state_set(|sw, state| {
        let ds = gtk_wrappers::get_application(sw);
        ds.imp().drilldown.set(state);
        Propagation::Proceed
    });
    bar
}