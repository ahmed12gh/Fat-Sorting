//# In God We T-Rust.
use super::sorting;
use std::path::PathBuf;
use utils::devices::drives::Drive;
use utils::find_drives;

use crate::gui::ListOfFiles;
use crate::gui::create_header_bar;
use crate::sorting::list_drive_files;
use crate::sorting::sort_drive;

use std::{rc::Rc , cell::RefCell} ;

use adw::prelude::*;
use adw::Application;
use gtk::glib;
use gtk::glib::clone;
use gtk::ApplicationWindow;

const APP_ID: &str = "Sorting.Fat32";
const TITLE: &str = "Fat Sorting";
const HEIGHT: i32 = 600;
const WIDTH: i32 = 400;

pub fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let headerbar = create_header_bar();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .width_request(400)
        .height_request(400)
        .titlebar(&headerbar)
        .build();

    let sort_action = gtk::gio::SimpleAction::new("sort", None);
    let reset_action = gtk::gio::SimpleAction::new("reset", None);

    let mut current_drive: Drive = Drive::new("".to_string(), "", "");
    let mut orignal_list: Vec<PathBuf> = Vec::new() ;
    let list_files = Rc::new(RefCell::new(ListOfFiles::new())) ;

    let overlay = gtk::Overlay::new();

    if let Some(drives) = find_drives() {
        current_drive = drives[0].clone();
        // show chooser dialog
        let  files: Vec<PathBuf> = match sorting::list_drive_files() {
            Some(files) => files,
            None => Vec::new(),
        };
        //clone the orinal list
        // crate the list view and set it to overlay as a child
        orignal_list = files.clone();
        list_files.borrow_mut().set_files(files);
        overlay.set_child(Some(&list_files.borrow_mut().create_list()));

        window.set_child(Some(&overlay))
    } else {
        //  show no drive is connected
    }

    sort_action.connect_activate(clone!(@weak window , @strong list_files=> move |_, _|{
        // return sorted list 
        // give it to sorting sort drive 
        // clear the overlay 
        let returned_list = list_files.borrow_mut().return_sorted_list() ;
        dbg!(&returned_list);
        sorting::sort_drive(&returned_list, &current_drive);
        let new_list = list_drive_files().unwrap() ;
        list_files.borrow_mut().set_files(new_list);
        list_files.borrow().update_list() ;

    }));

    reset_action.connect_activate(clone!(@weak window , @strong list_files  => move |_, _|{
        // change the list of files of Listoffiles to original
        // update the list  
        list_files.borrow_mut().set_files(orignal_list.clone()) ;
        list_files.borrow().update_list() ;
    }));

    window.add_action(&sort_action);
    window.add_action(&reset_action);

    window.present();
}
