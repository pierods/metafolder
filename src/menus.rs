use gtk::{Align, Button, Label, MenuButton, Orientation, Switch};
use gtk::{ApplicationWindow, ColorDialog, ColorDialogButton, HeaderBar};
use gtk::gdk::RGBA;
use gtk::glib::Propagation;

use crate::glib::clone;
use gtk::glib;

use gtk::prelude::{BoxExt, ButtonExt, Cast, PopoverExt, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{DEFAULT_BG_COLOR, files, gtk_wrappers, zoom};
use crate::folder::draw_folder;
use crate::gtk_wrappers::{alert};
use crate::cell_editor::make_cell_formatter;
use crate::find::make_find;
use crate::preset::make_presets;

pub(crate) fn make_header_bar(app_window: &ApplicationWindow) -> HeaderBar {
    let bar = HeaderBar::new();

    let up = gtk::Button::builder().label("up").build();
    up.connect_clicked(|b| {
        up_button_action(b);
    });
    up.set_tooltip_text(Some("go up to parent folder"));
    bar.pack_start(&up);

    let drilldown_label = &gtk::Label::builder().label("drilldown").build();
    drilldown_label.set_tooltip_text(Some("switch between drilldown or open folder for double click"));
    bar.pack_start(drilldown_label);
    let drilldown_switch = gtk::Switch::builder().state(true).active(true).build();
    drilldown_switch.set_tooltip_text(Some("switch between drilldown or open folder for double click"));
    drilldown_switch.connect_state_set(|sw, state| {
        drilldown_action(sw, state)
    });
    bar.pack_start(&drilldown_switch);

    let (zoom_popover, zoom_x_scale, zoom_y_scale) = zoom::make_zoom();
    let zoom_button = gtk::MenuButton::builder().icon_name("folder").popover(&zoom_popover).build();
    zoom_button.set_tooltip_text(Some("zoom folder in or out"));
    bar.pack_start(&zoom_button);
    zoom_popover.connect_closed(clone!(@weak zoom_button => move |_| {
        let ds = gtk_wrappers::get_application(&zoom_button);
        if ds.imp().metafolder.borrow().zoom {
            zoom_button.set_css_classes(&["folder-zoomed"]);
        }
        else {
        zoom_button.set_css_classes(&["folder-unzoomed"]);
        }
    }));

    let background_dialog = ColorDialog::builder().modal(true).title("Pick a background color").with_alpha(true).build();
    let background_color_button = ColorDialogButton::builder().rgba(&RGBA::parse(DEFAULT_BG_COLOR).unwrap()).dialog(&background_dialog).build();
    background_color_button.set_tooltip_text(Some("Pick a background color"));
    background_color_button.connect_rgba_notify(|cdb| {
        background_color_action(cdb);
    });
    bar.pack_start(&background_color_button);

    let text_color_dialog = ColorDialog::builder().modal(true).title("Pick a text color").with_alpha(true).build();
    let text_color_button = ColorDialogButton::builder().rgba(&RGBA::parse(DEFAULT_BG_COLOR).unwrap()).dialog(&text_color_dialog).build();
    text_color_button.set_tooltip_text(Some("Pick a text color"));
    text_color_button.connect_rgba_notify(|cdb| {
        text_color_action(cdb);
    });
    bar.pack_start(&text_color_button);

    let (cell_size_popover, text_scale, bold_switch, cell_size_scale) = make_cell_formatter();
    let cell_size_button = MenuButton::builder().label("a").popover(&cell_size_popover).build();
    cell_size_button.set_tooltip_text(Some("Edit cells"));
    bar.pack_start(&cell_size_button);

    // let preset_button = MenuButton::builder().icon_name("document-save").popover(&make_presets()).build();
    // bar.pack_start(&preset_button);
    //
     let (search_box, search_results) = make_find();
    bar.pack_end(&search_box);
    bar.pack_end(&search_results);

    let app_name_pango = String::from("<span font_weight =\"bold\">metafolder</span>");
    let app_name_label = Label::builder().use_markup(true).label(app_name_pango.as_str()).build();
    let path_label = Label::new(Some(""));
    let title_widget = gtk::Box::builder().orientation(Orientation::Vertical).valign(Align::Center).build();
    title_widget.append(&app_name_label);
    title_widget.append(&path_label);
    bar.set_title_widget(Some(&title_widget));

    let ds = gtk_wrappers::get_application(app_window);
    ds.imp().path_label.replace(Some(path_label));
    ds.imp().drilldown_switch.replace(Some(drilldown_switch));
    ds.imp().bg_color_button.replace(Some(background_color_button));
    ds.imp().zoom_button.replace(Some(zoom_button));
    ds.imp().zoom_x_scale.replace(Some(zoom_x_scale));
    ds.imp().zoom_y_scale.replace(Some(zoom_y_scale));
    ds.imp().font_color_button.replace(Some(text_color_button));
    ds.imp().text_size_scale.replace(Some(text_scale));
    ds.imp().font_bold_switch.replace(Some(bold_switch));
    ds.imp().cell_size_scale.replace(Some(cell_size_scale));

    bar
}

fn text_color_action(cdb: &ColorDialogButton) {
    let ds = gtk_wrappers::get_application(cdb);
    let mf = ds.imp().metafolder.borrow();
    if let Some(err) = mf.change_font_color(cdb.rgba().to_str().to_string(), true) {
        alert(cdb, "folder settings could not be saved".to_string(), err.to_string());
    }
}

fn background_color_action(cdb: &ColorDialogButton) {
    let ds = gtk_wrappers::get_application(cdb);
    match ds.imp().metafolder.borrow_mut().update_background_color(cdb.rgba().to_string()) {
        None => { gtk_wrappers::set_window_background(cdb.rgba().to_string()); }
        Some(err) => {
            alert(cdb, "folder settings could not be saved".to_string(), err.to_string());
        }
    };
}

fn drilldown_action(sw: &Switch, state: bool) -> Propagation {
    let ds = gtk_wrappers::get_application(sw);
    let x = match ds.imp().metafolder.borrow_mut().set_drilldown(state) {
        None => { Propagation::Proceed }
        Some(err) => {
            alert(sw, "folder settings could not be saved".to_string(), err.to_string());
            Propagation::Stop
        }
    };
    x
}

fn up_button_action(b: &Button) {
    let ds = gtk_wrappers::get_application(b);
    let current_path = ds.imp().metafolder.borrow().get_current_path();
    match files::up(&current_path) {
        None => { alert(b, "nowhere to go".to_string(), "".to_string()); }
        Some(up) => {
            let root = b.root().unwrap();
            let app_window = root.downcast::<gtk::ApplicationWindow>().unwrap();
            draw_folder(up, &app_window)
        }
    }
}