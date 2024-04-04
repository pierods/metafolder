use gtk::AccessibleRole::{Application, Label, Switch};
use gtk::gio::MenuModel;
use gtk::glib::Propagation;
use gtk::{ApplicationWindow, HeaderBar, Root};
use gtk::prelude::{ApplicationExt, ButtonExt, Cast, GtkWindowExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::appwindow_with_datastore::AppWithDatastore;

pub(crate) fn build_menu(mwnubar: Option<MenuModel>) {}

pub(crate) fn make_header_bar() -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {});

    bar.pack_start(&up);
    let drilldown_switch = gtk::Switch::new();

    bar.pack_start(&gtk::Label::builder().label("drilldown").build());
    bar.pack_start(&drilldown_switch);

    bar.pack_start(&gtk::Label::builder().label("memorize").build());
    let memorize_switch = gtk::Switch::new();
    bar.pack_start(&memorize_switch);

    drilldown_switch.connect_state_set(|sw, state| {
        let root = sw.root().unwrap();
        let app_window = root.downcast::<ApplicationWindow>().unwrap();
        let app = app_window.application().unwrap();
        let ds = app.downcast::<AppWithDatastore>().unwrap();
        ds.imp().drilldown.set(state);
        Propagation::Proceed
    });
    bar
}