
//# In God We T-Rust.
use super::sorting;
use std::path::PathBuf;
use utils::find_drives ;

use crate::gui::create_list;
use adw::prelude::*;
use adw::Application;
use gtk::ApplicationWindow;


const APP_ID: &str = "Sorting.Fat32";
const TITLE: &str = "Fat Sorting";

pub fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    
    if let Some(f) = find_drives() { 
        // show chooser dialog
    }
    else { 
        //  show no drive is connected
    }

    let files :Vec<PathBuf>= match sorting::sort_files(){
        Some(files) => files , 
        None => Vec::new()
    };

    let overlay_list = create_list(&files ) ; 
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(800)
        .default_height(600)
        .width_request(400)
        .height_request(400)                                                
        .child(&overlay_list)
        .build();

    window.present();

}