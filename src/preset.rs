use gtk::{Button, EditableLabel, Popover};
use rand::Rng;
use gtk::Orientation::Vertical;
use gtk::prelude::{BoxExt, ButtonExt, PopoverExt};

pub(crate) fn make_presets() -> Popover {
    let container = gtk::Box::builder().orientation(Vertical).build();
    let popover = Popover::builder().build();
    popover.set_child(Some(&container));

    let add_button = Button::builder().label("+").build();
    container.append(&add_button);

    add_button.connect_clicked(move |_b| {
        let preset_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();

        let mut rng = rand::thread_rng();
        let preset = EditableLabel::builder().text("preset-".to_owned() + rng.gen_range(1000..9999).to_string().as_str()).build();
        preset_box.append(&preset);
        let preset_button = Button::builder().label("apply").build();
        preset_box.append(&preset_button);
        container.append(&preset_box);
    });

    popover
}