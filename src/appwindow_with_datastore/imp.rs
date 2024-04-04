use std::cell::{Cell, RefCell};
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
// By implementing Default we don't have to provide a `new` fn in our
// ObjectSubclass impl.
pub struct AppWithDatastore {
    pub(crate) current_path: RefCell<String>,
    pub(crate) drilldown : Cell<bool>,
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