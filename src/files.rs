use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use gtk::gio;
use gtk::gio::{Cancellable, FileInfo, FileType};
use gtk::prelude::FileExt;
use ignore::Error;
use serde::{Deserialize, Serialize};

pub(crate) fn try_file(path: &str) -> bool {
    Path::new(path).exists()
}

pub(crate) fn home_path() -> String {
    match home::home_dir() {
        Some(pb) => {
            match pb.to_str() {
                Some(s) => String::from(s),
                None => panic!("Impossible to get your home dir!"),
            }
        }
        None => panic!("Impossible to get your home dir!"),
    }
}

pub(crate) fn get_entries(p: String) -> HashSet<DirItem> {
    let mut entries: HashSet<DirItem> = HashSet::new();

    let paths = fs::read_dir(p).expect("Impossible to get your home dir!");

    for path in paths {
        let dir_entry = path.expect("Impossible to get your home dir!").path();
        let file_name_opt = dir_entry.file_name();
        match file_name_opt {
            Some(f) => {
                let file_name_string_opt = f.to_str();
                match file_name_string_opt {
                    Some(_) => {
                        let dir_item_opt = get_file_info(dir_entry.as_path().to_str().expect("Fatal: get complete path").to_string());
                        match dir_item_opt {
                            Some(dir_item) => {
                                entries.insert(dir_item);
                            }
                            None => continue
                        }
                    }
                    None => panic!("Impossible to get your home dir!"),
                }
            }
            None => panic!("Impossible to get your home dir!"),
        }
    }
    entries
}

#[derive(Eq, Hash, PartialEq, Default)]
pub struct DirItem {
    pub(crate) name: String,
    pub(crate) path_name: String,
    pub(crate) is_dir: bool,
    pub(crate) mime_type: String,
    pub(crate) icon: Option<gio::Icon>,
}


fn get_file_info(path_name: String) -> Option<DirItem> {
    let mut dir_item: DirItem = DirItem::default();

    dir_item.path_name = path_name.clone();

    let g_file = gio::File::for_path(path_name.clone());

    let g_file_info_result = g_file.query_info("*", gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS, Cancellable::NONE);
    let g_file_info: FileInfo;
    match g_file_info_result {
        Ok(file_info) => {
            g_file_info = file_info;
        }
        Err(error) => {
            println!("{}", error);
            return Option::None;
        }
    }

    if g_file_info.is_hidden() {
        return Option::None;
    };
    dir_item.name = g_file_info.name().to_str().expect("Fatal: gio cannot get path").to_string();
    if g_file_info.file_type() == FileType::Directory {
        dir_item.is_dir = true;
    }
    match g_file_info.icon() {
        Some(g_icon) => { dir_item.icon = Option::Some(g_icon) }
        None => {
            println!("cannot find icon for {}", path_name);
            dir_item.icon = Option::None
        }
    }
    match g_file_info.content_type() {
        Some(gmime) => { dir_item.mime_type = gmime.as_str().to_string() }
        None => {
            dir_item.mime_type = String::from("");
            println!("cannot find mime type for  {}", path_name)
        }
    }
    Some(dir_item)
}

#[derive(Eq, Hash, PartialEq, Default, Serialize, Deserialize, Debug)]
pub struct MemoIcon {
    pub(crate) position_x: i32,
    pub(crate) position_y: i32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct MemoDesktop {
    pub(crate) path_name: String,
    pub(crate) background_color: String,
    cell_size: i32,
    drilldown: bool,
    pub(crate) icons: HashMap<String, MemoIcon>,
}


pub(crate) fn save_settings(memo_desktop: MemoDesktop) -> Option<Error> {
    let serialized = serde_json::to_string_pretty(&memo_desktop).unwrap();
    let mut settings_path = memo_desktop.path_name.clone();
    settings_path.push_str("/.metafolder");
     match std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(settings_path) {
         Ok(mut f) => {
             f.write_all(serialized.as_bytes()).unwrap();
             f.flush().unwrap();
             return None;
         }
         Err(error) => {
             return Some(Error::from(error));
         }
     }
}

pub(crate) fn load_settings(mut path: String) -> MemoDesktop {
    path.push_str("/.metafolder");
    if !try_file(path.as_str()) {
        return MemoDesktop::default();
    }
    let mut f: fs::File;
    let f_result = std::fs::OpenOptions::new().read(true).open(path);
    match f_result {
        Ok(file) => { f = file }
        Err(e) => {
            println!("{}", e);
            return MemoDesktop::default();
        }
    }
    let mut serialized = String::new();
    f.read_to_string(&mut serialized).unwrap();

    let memo_desktop: MemoDesktop = serde_json::from_str(serialized.as_str()).unwrap();

    memo_desktop
}

pub fn initial_dir() -> String {
    let mut path_name = home_path();
    if try_file((path_name.clone() + "/Desktop").as_str()) {
        path_name += "/Desktop";
    }
    path_name
}

pub fn up(path: &String) -> Option<String> {
    if path == "/" {
        return None;
    }
    let std_path = Path::new(&path);
    let mut ancestors = std_path.ancestors();
    ancestors.next();
    match ancestors.next() {
        None => { return None; }
        Some(ancestor) => {
            match ancestor.to_str() {
                None => { return None; }
                Some(parent) => { return Some(String::from(parent)); }
            }
        }
    }
}