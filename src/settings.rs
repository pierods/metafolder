use std::collections::HashMap;

use gtk::Fixed;
use gtk::prelude::{FixedExt, IsA};
use ignore::Error;

use crate::{files, gtk_wrappers};
use crate::files::{load_settings, MemoIcon};
use crate::gtk_wrappers::{get_desktop, get_widget_bounds, set_zoom_widgets};

#[derive(Default, Debug)]
pub struct MetaFolder {
    pub(crate) background_color: String,
    cell_size: i32,
    pub(crate) drilldown: bool,
    pub(crate) cell_map: HashMap<String, gtk::Box>,
    pub(crate) current_path: String,
    pub(crate) zoom: bool,
    pub(crate) zoom_x: i32,
    pub(crate) zoom_y: i32,
}


impl MetaFolder {
    pub(crate) fn zoom_and_set_zoom_widgets(&mut self, zoom_x: i32, zoom_y: i32, w: &impl IsA<gtk::Widget>) {
        self.zoom(zoom_x, zoom_y, w);
        set_zoom_widgets(w, true, zoom_x, zoom_y);
    }
    pub(crate) fn zoom_and_save_settings(&mut self, zoomx: i32, zoomy: i32, w: &impl IsA<gtk::Widget>) {
        self.zoom(zoomx, zoomy, w);
        self.save_zoom_setttings(true, zoomx, zoomy);
    }

    fn save_zoom_setttings(&self, zoom: bool, zoomx: i32, zoomy: i32) {
        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.zoom = zoom;
        memo_folder.zoom_x = zoomx;
        memo_folder.zoom_y = zoomy;
        files::save_settings(self.current_path.clone(), memo_folder);
    }

    pub(crate) fn zoom(&mut self, zoomx: i32, zoomy: i32, w: &impl IsA<gtk::Widget>) {
        if self.zoom_x == zoomx && self.zoom_y == zoomy {
            println!("equal");
            return;
        }
        self.zoom = true;

        if self.zoom_x == 0 {
            self.zoom_x = 100;
            self.zoom_y = 100;
        }
        let zxf = (zoomx as f32) / 100f32;
        let zyf = (zoomy as f32) / 100f32;
        let desktop = get_desktop(w);

        for (_, gbox) in &self.cell_map {
            let current_pos = get_widget_bounds(&desktop, gbox);
            let unzoomed_x = (current_pos.x() / self.zoom_x as f32) * 100f32;
            let unzoomed_y = (current_pos.y() / self.zoom_y as f32) * 100f32;
            let new_x = unzoomed_x * zxf;
            let new_y = unzoomed_y * zyf;
            desktop.move_(gbox, new_x as f64, new_y as f64)
        }

        self.zoom_x = zoomx;
        self.zoom_y = zoomy;
    }

    pub fn zoom_commit_and_save_settings(&mut self, w: &impl IsA<gtk::Widget>) {
        self.zoom = false;
        self.zoom_x = 0;
        self.zoom_y = 0;

        let mut icons: HashMap<String, MemoIcon> = HashMap::new();
        let desktop = get_desktop(w);
        for (path, gbox) in &self.cell_map {
            let bounds = gtk_wrappers::get_widget_bounds(&desktop, &gbox);
            let memo_icon = MemoIcon {
                position_x: bounds.x() as i32,
                position_y: bounds.y() as i32,
            };
            icons.insert(path.to_string(), memo_icon);
        }

        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.zoom = self.zoom;
        memo_folder.zoom_x = self.zoom_x;
        memo_folder.zoom_y = self.zoom_y;
        memo_folder.icons = icons;

        files::save_settings(self.current_path.clone(), memo_folder);
        set_zoom_widgets(w, false, 0, 0);
    }

    pub fn unzoom(&mut self, w: &impl IsA<gtk::Widget>) {
        self.zoom = false;

        if self.zoom_x == 0 {
            return;
        }
        let desktop = get_desktop(w);

        for (_, gbox) in &self.cell_map {
            let current_pos = get_widget_bounds(&desktop, gbox);
            let unzoomed_x = (current_pos.x() / self.zoom_x as f32) * 100f32;
            let unzoomed_y = (current_pos.y() / self.zoom_y as f32) * 100f32;
            desktop.move_(gbox, unzoomed_x as f64, unzoomed_y as f64)
        }
        self.zoom_x = 0;
        self.zoom_y = 0;
    }
    pub fn unzoom_and_save_settings(&mut self, w: &impl IsA<gtk::Widget>) {
        self.unzoom(w);
        self.save_zoom_setttings(false, 0, 0);
    }
    pub(crate) fn arrange_cells_and_save_settings(&self, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> Option<Error> {
        let mut memo_folder = load_settings(self.current_path.clone());
        let mut icons: HashMap<String, MemoIcon> = HashMap::new();

        for (path, gbox) in &self.cell_map {
            let memo_icon: MemoIcon;
            if path == icon_file_path {
                memo_icon = MemoIcon {
                    position_x: x as i32,
                    position_y: y as i32,
                };
            } else {
                let bounds = gtk_wrappers::get_widget_bounds(desktop, &gbox);
                memo_icon = MemoIcon {
                    position_x: bounds.x() as i32,
                    position_y: bounds.y() as i32,
                };
            }
            icons.insert(path.to_string(), memo_icon);
        }
        memo_folder.icons = icons;
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    pub(crate) fn update_background_color(&mut self, new_color: String) -> Option<Error> {
        self.background_color = new_color.clone();

        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.background_color = new_color;
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    pub(crate) fn build_new(&mut self, new_metafolder: &MetaFolder) {
        self.current_path = new_metafolder.current_path.clone();
        self.cell_size = new_metafolder.cell_size;
        self.background_color = new_metafolder.background_color.clone();
        self.drilldown = new_metafolder.drilldown;
        self.cell_map = new_metafolder.cell_map.clone();
        self.zoom = new_metafolder.zoom;
        self.zoom_x = new_metafolder.zoom_x;
        self.zoom_y = new_metafolder.zoom_y
    }

    pub(crate) fn get_cell(&self, csp: String) -> &gtk::Box {
        self.cell_map.get(&csp).expect("Fatal: cannot find cell")
    }

    pub(crate) fn get_current_path(&self) -> String {
        self.current_path.clone()
    }

    pub(crate) fn set_drilldown(&mut self, status: bool) -> Option<Error> {
        self.drilldown = status;

        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.drilldown = self.drilldown;
        files::save_settings(self.current_path.clone(), memo_folder)
    }
}
