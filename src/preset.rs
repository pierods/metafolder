use gtk::{Align, Button, Entry, Label, Popover};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::prelude::{BoxExt, ButtonExt, EditableExt, PopoverExt};

pub(crate) fn make_presets() -> Popover {
    let container = gtk::Box::builder().orientation(Vertical).build();
    let popover = Popover::builder().build();
    popover.set_child(Some(&container));

    let add_box = gtk::Box::builder().orientation(Horizontal).build();
    add_box.set_spacing(10);
    let add_entry = Entry::builder().max_length(30).build();
    add_box.append(&add_entry);
    let add_button = Button::builder().label("+").build();
    add_box.append(&add_button);

    container.append(&add_box);

    add_button.connect_clicked(move |_b| {
        if add_entry.text() == "" {
            return;
        }
        let preset_box = gtk::Box::builder().orientation(Horizontal).hexpand(true).build();
        preset_box.set_spacing(10);
        let preset = Label::builder().label(add_entry.text()).build();
        preset_box.append(&preset);
        let delete_button = Button::builder().label("delete").halign(Align::End).build();
        preset_box.append(&delete_button);
        let preset_button = Button::builder().label("apply").halign(Align::End).build();
        preset_box.append(&preset_button);

        container.append(&preset_box);
        add_entry.set_text("");
    });

    popover
}