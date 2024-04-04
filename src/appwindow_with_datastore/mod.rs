mod imp;

use gtk::{gio, glib};
use gtk::glib::SignalHandlerId;
use gtk::prelude::ApplicationExt;
use crate::APP_ID;

glib::wrapper! {
    pub struct AppWithDatastore(ObjectSubclass<imp::AppWithDatastore>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

impl Default for AppWithDatastore {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .build()
    }
}
