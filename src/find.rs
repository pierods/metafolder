use gtk::{EventControllerKey, Label, SearchEntry};
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::{EditableExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::gtk_wrappers;

pub(crate) fn make_find() -> (SearchEntry, Label) {
    let find_box = SearchEntry::builder().placeholder_text("search").build();
    find_box.set_tooltip_text(Some("find a cell by name - enter/escape"));
    find_box.set_width_request(300);

    let find_results = Label::new(None);
    find_box.connect_activate(clone!(@weak find_results => move |f_b| {
        let ds = gtk_wrappers::get_application(f_b);
        let matches = ds.imp().metafolder.borrow_mut().find_cell(f_b.text().to_string());
        match matches {
            0 => {find_results.set_label("no matches");}
            1 => {find_results.set_label((matches.to_string().as_str().to_owned() + " match").as_str());}
            _ => {find_results.set_label((matches.to_string().as_str().to_owned() + " matches").as_str());}
        }
    }));
    find_box.connect_search_changed(clone!(@weak find_results => move |f_b| {
        if f_b.text() == "" {
            let ds = gtk_wrappers::get_application(f_b);
            ds.imp().metafolder.borrow_mut().clear_found_cells();
            find_results.set_label("")
        }
    }));
    find_box.connect_stop_search(clone!(@weak find_results => move |f_b| {
        let ds = gtk_wrappers::get_application(f_b);
        ds.imp().metafolder.borrow_mut().clear_found_cells();
        find_results.set_label("")
    }));

    let key_capture = EventControllerKey::new();
    //disable ctrl-g default behavior
    key_capture.connect_key_pressed(|_, _, _, _| {
        glib::Propagation::Proceed
    });
    find_box.add_controller(key_capture);

    (find_box, find_results)
}