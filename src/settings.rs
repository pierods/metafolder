use gtk::Fixed;
use ignore::Error;
use std::collections::HashMap;
use crate::files::{load_settings, MemoFolder, MemoIcon};
use crate::{files, gtk_wrappers};

#[derive(Default, Debug)]
pub struct MetaFolder {
    pub(crate) background_color: String,
    cell_size: i32,
    pub(crate) drilldown: bool,
    pub(crate) cell_map: HashMap<String, gtk::Box>,
    pub(crate) current_path : String,
}


impl MetaFolder{
    pub(crate) fn update_cell_positions(&self, desktop: &Fixed, icon_file_path: &str, x: f64, y: f64) -> Option<Error> {
        let mut memo_folder = MemoFolder::default();
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

    pub(crate) fn get_cell(&self, csp : String) -> &gtk::Box {
        self.cell_map.get(&csp).expect("Fatal: cannot find cell")
    }

    pub(crate) fn get_current_path(&self) -> String {
        self.current_path.clone()
    }
}
