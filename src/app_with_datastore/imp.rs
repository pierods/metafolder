use std::cell::RefCell;

use gtk::{glib, Label, subclass::prelude::*};
use gtk::gio::FileMonitor;

use crate::metafolder::MetaFolder;

#[derive(Debug, Default)]
// By implementing Default we don't have to provide a `new` fn in our
// ObjectSubclass impl.
pub struct AppWithDatastore {
    pub(crate) metafolder: RefCell<MetaFolder>,
    pub(crate) path_label: RefCell<Option<Label>>,
    pub(crate) monitor: RefCell<Option<FileMonitor>>,
    pub(crate) drilldown_switch: RefCell<Option<gtk::Switch>>,
    pub(crate) bg_color_button: RefCell<Option<gtk::ColorDialogButton>>,
    pub(crate) zoom_button: RefCell<Option<gtk::Button>>,
    pub(crate) zoom_x_scale: RefCell<Option<gtk::Scale>>,
    pub(crate) zoom_y_scale: RefCell<Option<gtk::Scale>>,
    pub(crate) font_color_button: RefCell<Option<gtk::ColorDialogButton>>,
    pub(crate) text_size_scale: RefCell<Option<gtk::Scale>>,
    pub(crate) font_bold_switch: RefCell<Option<gtk::Switch>>,
    pub(crate) cell_size_scale: RefCell<Option<gtk::Scale>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppWithDatastore {
    const NAME: &'static str = "AppWithDatastore";
    type Type = super::AppWithDatastore;
    type ParentType = gtk::Application;
}

impl ObjectImpl for AppWithDatastore {}

impl ApplicationImpl for AppWithDatastore {}

impl GtkApplicationImpl for AppWithDatastore {}
