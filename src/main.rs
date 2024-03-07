use std::collections::HashMap;
use std::path::{Path};
use std::fs;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid, pango, Overlay};

const APP_ID: &str = "org.github.pierods.metafolder";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("metafolder")
        //.child(&scrolled_window)
        .build();

    window.connect_maximized_notify(|win: &ApplicationWindow|{ println!("*****************************{}", win.width())});

    // Present window
    window.present();
    window.maximize();

    let entries : HashMap<String, String>;

    let home = home_path();
    if try_desktop(home.as_str()) {
        entries = get_entries(home + "/Desktop");
    } else {
        entries = get_entries(home);
    }
    let grid = gtk::Grid::new();
    draw_icons_as_grid(entries, &grid, 1500);

    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_child( Option::Some(&grid));


    window.set_child(Option::Some(&scrolled_window));

}

fn draw_icons_as_grid(entries: HashMap<String, String>, grid: &Grid, width: i32) {

    let mut r: i32 = 0;
    let mut c: i32 = 0;

    for (k, v_) in entries {
        let size = 100;
        grid.attach(&make_cell(k, size), c, r, size, size);
        c += size;
        if c > width {
            c = 0;
            r += size;
        }
        //break;
    }
}

fn make_cell(text: String, size : i32) -> Overlay {
    let img = gtk::Image::from_file("/home/piero/temp/gtk4-rs/examples/clipboard/asset.png");
    //let img = gtk::Image::from_icon_name("folder");
    img.set_halign(gtk::Align::Center);
    img.set_pixel_size(size);

    let pango_string = String::from("<span font_size=\"small\">") + text.as_str() + "</span>";
    let txt = gtk::Label::new(Option::Some(pango_string.as_str()));
    txt.set_use_markup(true);
    txt.set_ellipsize(pango::EllipsizeMode::End);
    txt.set_wrap(true);
    txt.set_wrap_mode(pango::WrapMode::WordChar);
    txt.set_lines(2);

    txt.set_justify(gtk::Justification::Center);


    txt.set_halign(gtk::Align::Center);
    txt.set_valign(gtk::Align::End);


    let desktop_icon = gtk::Overlay::new();
    desktop_icon.set_child(Option::Some(&img));
    desktop_icon.add_overlay(&txt);

    desktop_icon

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

    let paths = fs::read_dir(p).expect("Impossible to get your home dir!");

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

