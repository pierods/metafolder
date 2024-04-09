use std::collections::HashMap;

use gtk::Fixed;
use gtk::prelude::{FixedExt, IsA};
use ignore::Error;

use crate::{files, gtk_wrappers};
use crate::files::{load_settings, MemoIcon};
use crate::gtk_wrappers::{get_desktop, get_widget_bounds};

#[derive(Default, Debug)]
pub struct MetaFolder {
    pub(crate) background_color: String,
    cell_size: i32,
    pub(crate) drilldown: bool,
    pub(crate) cell_map: HashMap<String, gtk::Box>,
    pub(crate) current_path: String,
    pub(crate) zoom: bool,
    pub(crate) zoomx: i32,
    pub(crate) zoomy: i32,
}


impl MetaFolder {
    pub(crate) fn zoom(& mut self, zoomx : i32, zoomy : i32, w : & impl IsA<gtk::Widget>) {
        if self.zoomx == zoomx && self.zoomy == zoomy {
            println!("equal");
            return;
        }
        self.zoom = true;

        if self.zoomx == 0 {
            self.zoomx = 100;
            self.zoomy = 100;
        }
        let zxf = (zoomx as f32)/100f32;
        let zyf = (zoomy as f32)/100f32;
        let desktop = get_desktop(w);

        for (_, gbox) in &self.cell_map {
            let current_pos = get_widget_bounds(&desktop, gbox);
            let unzoomed_x = (current_pos.x() / self.zoomx as f32) * 100f32;
            let unzoomed_y = (current_pos.y() / self.zoomy as f32) * 100f32;
            let new_x = unzoomed_x * zxf;
            let new_y = unzoomed_y * zyf;
            desktop.move_(gbox, new_x as f64, new_y as f64)
        }

        self.zoomx = zoomx;
        self.zoomy = zoomy;
    }

    pub fn commit_zoom(&mut self, w : & impl IsA<gtk::Widget>) {
        self.zoom = false;
        self.zoomx = 0;
        self.zoomy = 0;
    }
    pub fn reset_zoom(&mut self, w : & impl IsA<gtk::Widget>) {
        self.zoom = false;

        if self.zoomx == 0 {
            return;
        }
        let desktop = get_desktop(w);

        for (_, gbox) in &self.cell_map {
            let current_pos = get_widget_bounds(&desktop, gbox);
            let unzoomed_x = (current_pos.x() / self.zoomx as f32) * 100f32;
            let unzoomed_y = (current_pos.y() / self.zoomy as f32) * 100f32;
            desktop.move_(gbox, unzoomed_x as f64, unzoomed_y as f64)
        }
        self.zoomx = 0;
        self.zoomy = 0;
    }

    pub(crate) fn arrange_cells(&self, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> Option<Error> {
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
                //let bounds = gbox.allocation();
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
