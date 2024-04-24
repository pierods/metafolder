use std::collections::{HashMap, HashSet};

use gtk::{Fixed};
use gtk::prelude::{Cast, FixedExt, IsA, WidgetExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use ignore::Error;

use crate::{files, gtk_wrappers};
use crate::files::{load_settings, MemoIcon};
use crate::gtk_wrappers::{get_application, get_desktop, get_widget_bounds, set_zoom_widgets};

#[derive(Default, Debug)]
pub struct MetaFolder {
    pub(crate) background_color: String,
    cell_size: i32,
    pub(crate) drilldown: bool,
    pub(crate) cell_map: HashMap<String, gtk::Box>,
    pub(crate) added_cells: HashSet<String>,
    pub(crate) current_path: String,
    pub(crate) zoom: bool,
    pub(crate) zoom_x: i32,
    pub(crate) zoom_y: i32,

}

impl MetaFolder {
    pub(crate) fn rename_cell(&mut self, old_name: &str, new_name: &str) -> Option<Error> {
        let cell = self.cell_map.remove(old_name).unwrap();
        cell.set_tooltip_text(Some(new_name));
        let image_widget = cell.first_child().unwrap();
        image_widget.set_tooltip_text(Some(new_name));

        let label_widget = cell.last_child().unwrap();
        label_widget.set_tooltip_text(Some(new_name));
        let label = label_widget.downcast::<gtk::Label>().unwrap();
        let mut label_text = label.label().as_str().to_string();
        label_text = label_text.replace(old_name, new_name);
        label.set_label(label_text.as_str());
        self.cell_map.insert(new_name.to_string(), cell);

        let mut memo_folder = load_settings(self.current_path.clone());
        let memo_icon = memo_folder.icons.remove(old_name);

        if memo_icon.is_none() {
            println!("Unexpected: cell {} not found", old_name)
        }
        memo_folder.icons.insert(new_name.to_string(), memo_icon.unwrap());
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    pub(crate) fn add_cell(&mut self, name: String, cell: gtk::Box) {
        self.cell_map.insert(name.clone(), cell);
        self.added_cells.insert(name);
    }

    pub(crate) fn is_cell_newly_added(&self, name: String) -> bool {
        return self.added_cells.contains(name.as_str());
    }

    pub(crate) fn clear_added_flag(&mut self, name: String) {
        self.added_cells.remove(name.as_str());
    }
    pub(crate) fn delete_cell(&mut self, name: String) -> (gtk::Box, Option<Error>) {
        let cell = self.cell_map.remove(name.as_str());
        let mut memo_folder = load_settings(self.current_path.clone());
        if memo_folder.icons.remove(name.as_str()).is_none() {
            println!("Unexpected: cell {} not found", name)
        }
        (cell.unwrap(), files::save_settings(self.current_path.clone(), memo_folder))
    }
    pub(crate) fn zoom_and_set_zoom_widgets(&mut self, zoom_x: i32, zoom_y: i32, w: &impl IsA<gtk::Widget>) {
        self.move_to_zoomed(zoom_x, zoom_y, w);
        set_zoom_widgets(w, true, zoom_x, zoom_y);
    }

    pub(crate) fn zoom_and_save_settings(&mut self, zoomx: i32, zoomy: i32, w: &impl IsA<gtk::Widget>) -> Option<Error> {
        //don't save settings on a false movement
        if self.zoom(zoomx, zoomy, w) {
            self.save_zoom_settings(true, zoomx, zoomy);
        }
        None
    }

    fn save_zoom_settings(&self, zoom: bool, zoomx: i32, zoomy: i32) -> Option<Error> {
        let mut memo_folder = load_settings(self.current_path.clone());
        memo_folder.zoom = zoom;
        memo_folder.zoom_x = zoomx;
        memo_folder.zoom_y = zoomy;
        files::save_settings(self.current_path.clone(), memo_folder)
    }

    pub(crate) fn move_to_zoomed(&self, zoomx: i32, zoomy: i32, w: &impl IsA<gtk::Widget>) {
        let desktop = get_desktop(w);

        for (_, gbox) in &self.cell_map {
            let current_pos = get_widget_bounds(&desktop, gbox);
            let new_x = current_pos.x() * (zoomx as f32 / 100f32);
            let new_y = current_pos.y() * (zoomy as f32 / 100f32);
            desktop.move_(gbox, new_x as f64, new_y as f64)
        }
    }
    pub(crate) fn zoom(&mut self, zoomx: i32, zoomy: i32, w: &impl IsA<gtk::Widget>) -> bool {
        if self.zoom_x == zoomx && self.zoom_y == zoomy {
            println!("equal");
            //don't save settings on a false movement
            return false;
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
        true
    }

    pub fn zoom_commit_and_save_settings(&mut self, w: &impl IsA<gtk::Widget>) -> Option<Error> {
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

        set_zoom_widgets(w, false, 100, 100);
        files::save_settings(self.current_path.clone(), memo_folder)
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
        set_zoom_widgets(w, false, 100, 100);
    }
    pub fn unzoom_and_save_settings(&mut self, w: &impl IsA<gtk::Widget>) -> Option<Error> {
        self.unzoom(w);
        self.save_zoom_settings(false, 0, 0)
    }
    pub(crate) fn scan_positions_and_save_settings(&self, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> Option<Error> {
        let mut memo_folder = load_settings(self.current_path.clone());
        let mut icons: HashMap<String, MemoIcon> = HashMap::new();
        let ds = get_application(desktop);
        for (path, gbox) in &self.cell_map {
            let memo_icon: MemoIcon;
            if path == icon_file_path {
                memo_icon = MemoIcon {
                    position_x: x as i32,
                    position_y: y as i32,
                };
            } else {
                // don't consider newly added cells as having been deliberately placed where they are by the user
                // and thereby don't save their position
                if ds.imp().metafolder.borrow().added_cells.contains(path) {
                    continue;
                }
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
