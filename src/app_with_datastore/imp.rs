use std::cell::RefCell;

use gtk::{glib, Label, subclass::prelude::*};

use crate::settings::MetaFolder;

#[derive(Debug, Default)]
// By implementing Default we don't have to provide a `new` fn in our
// ObjectSubclass impl.
pub struct AppWithDatastore {
    pub(crate) desktop: RefCell<MetaFolder>,
    pub(crate) path: RefCell<Option<Label>>,
    pub(crate) drilldown: RefCell<Option<gtk::Switch>>,
    pub(crate) bg_color: RefCell<Option<gtk::ColorDialogButton>>,
    pub(crate) zoom_button: RefCell<Option<gtk::Button>>,
    pub(crate) zoom_x: RefCell<Option<gtk::Scale>>,
    pub(crate) zoom_y: RefCell<Option<gtk::Scale>>,
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
