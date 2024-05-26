use gtk::{EventControllerKey, gdk, Label, SearchEntry};
use gtk::gdk::ModifierType;
use gtk::glib;
use gtk::prelude::{EditableExt, WidgetExt};

use crate::glib::clone;

pub(crate) fn make_find() -> (SearchEntry, Label) {
    let find_box = SearchEntry::builder().placeholder_text("search cells").build();
    find_box.set_tooltip_text(Some("find a cell by name - ctrl-f/escape"));
    find_box.set_width_request(300);

    let find_results = Label::new(None);
    find_box.connect_search_changed(|a| {
        if a.text() == "" {
            println!("cleared")
        }
    });
    find_box.connect_search_started(|a| {
        println!("sst {}", a.text())
    }
    );
    find_box.connect_stop_search(|a| {
        a.set_text("");
    });

    let key_capture = EventControllerKey::new();

    key_capture.connect_key_pressed(clone!(@weak find_box, @weak find_results => @default-return glib::Propagation::Proceed, move |_, key, _, modifier_type| {
        match modifier_type {
            ModifierType::CONTROL_MASK => {
                match key {
                    gdk::Key::f => {
                        find_results.set_text(find_box.text().as_str());
                        println!("{}", find_box.text());
                    }
                    _ => return glib::Propagation::Proceed
                }
            }
            _ => return glib::Propagation::Proceed
        };
        glib::Propagation::Proceed
    }));
    find_box.add_controller(key_capture);

    (find_box, find_results)
}