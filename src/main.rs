use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    let entries : HashMap<String, String>;

    let home = home_path();
    if try_desktop(home.as_str()) {
        entries = get_entries(home + "/Desktop");
    } else {
        entries = get_entries(home);
    }
    let gtk_box = gtk::Grid::new();

    let mut r :i32 = 0;
    let mut c :i32 = 0;

    draw_icons_as_grid(entries, &gtk_box);

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child( Option::Some(&gtk_box));

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("metafolder")
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
    window.maximize();

}

fn draw_icons_as_grid(entries: HashMap<String, String>, gtk_box: &Grid) {
    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for (k, v) in entries {
        let size = 100;
        let img = gtk::Image::from_file("/home/piero/temp/gtk4-rs/examples/clipboard/asset.png");

        let txt = gtk::Label::new(Option::Some(k.as_str()));
        txt.ellipsize();
        txt.set_justify(gtk::Justification::Center);
        txt.set_halign(gtk::Align::Center);
        txt.set_valign(gtk::Align::End);

        let desktop_icon = gtk::Overlay::new();
        desktop_icon.set_child(Option::Some(&img));
        desktop_icon.add_overlay(&txt);
        img.set_pixel_size(size);
        gtk_box.attach(&desktop_icon, c, r, size, size);
        r += size;
        if r == 1000 {
            r = 0;
            c += size;
        }
        break;
    }
}

fn try_desktop(home_path : &str) -> bool {
    let desktop_path = home_path.to_string() + "/Desktop";
    Path::new(desktop_path.as_str()).exists()
}

fn home_path() -> String {
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

fn get_entries(p: String) -> HashMap<String, String>{
    let mut entries :  HashMap<String, String> =HashMap::new();

    let paths = fs::read_dir(p).unwrap();

    for path in paths {
        let dir_entry = path.unwrap().path();
        let file_name_opt = dir_entry.file_name();
        match file_name_opt {
            Some(f) => {
                let file_name_string_opt = f.to_str();
                match file_name_string_opt {
                    Some(f) =>  if !f.starts_with(".") {
                        entries.insert(f.to_string(), f.to_string());
                    },
                    None => panic!("Impossible to get your home dir!"),
                }
            }
            None => panic!("Impossible to get your home dir!"),
        }
    }
    entries
}

