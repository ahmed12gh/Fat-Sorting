//# In God We T-Rust.
use super::sorting;
use std::path::PathBuf;
use utils::devices::drives::Drive;
use utils::find_drives;

use crate::gui::*;
use crate::sorting::list_drive_files;

use std::{cell::RefCell, rc::Rc};

use adw::prelude::*;
use adw::Application;
use gtk::glib;
use gtk::glib::clone;
use gtk::ApplicationWindow;

const APP_ID: &str = "io.github.agent57.UsbSorter";
const TITLE: &str = "USB Sorter";
const HEIGHT: i32 = 600;
const WIDTH: i32 = 400;

pub fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // create header bar from headerbar.rs in gui folder
    // create the application window with default width and height and a headerbar
    let headerbar = create_header_bar();
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .width_request(400)
        .height_request(400)
        .icon_name("io.github.agent57.UsbSorter")
        .titlebar(&headerbar)
        .build();

    // functionality for both the reset and sort button in the headerbar 
    let sort_action = gtk::gio::SimpleAction::new("sort", None);
    let reset_action = gtk::gio::SimpleAction::new("reset", None);

    // expose the current drive and other varables to other functions in the code 
    let mut current_drive: Drive = Drive::new("".to_string(), "", "");
    let list_files = Rc::new(RefCell::new(ListOfFiles::new()));
    // save a copy of the orign order of the drive for reset
    let mut orignal_list: Vec<PathBuf> = Vec::new(); 
     
    // possible to go 
    let overlay = gtk::Overlay::new();

    if let Some(drives) = find_drives() {
        //TODO: show chooser dialog
        current_drive = drives[0].clone();
        let files: Vec<PathBuf> = match sorting::list_drive_files() {
            Some(files) => files,
            None => Vec::new(),
        };
        //clone the orinal list
        // crate the list view and set it to overlay as a child
        orignal_list = files.clone();
        list_files.borrow_mut().set_files(files);
        // show the list as gtk::listbox under an overlay
        overlay.set_child(Some(&list_files.borrow_mut().create_list()));
     } else {
        //  show no drive is connected page.rs
        // loop find_drives until drive is found every 1 sec
       let status_page = no_device_connected() ;
        overlay.set_child(Some(&status_page))
    }
    window.set_child(Some(&overlay));

    sort_action.connect_activate(clone!(@weak window , @weak list_files=> move |_, _|{
        // check if drive is selected ; Null drive name ""
        // return sorted list
        // give it to sorting sort drive
        // update the overlay
        if current_drive.name != ""{ 
            let returned_list = list_files.borrow_mut().return_sorted_list() ;
            sorting::sort_drive(&returned_list, &current_drive);
            let new_list = list_drive_files().unwrap() ;
            list_files.borrow_mut().set_files(new_list);
            list_files.borrow().update_list() ;
        }

    }));

    reset_action.connect_activate(clone!(@weak window  => move |_, _|{
        // change the list of files of Listoffiles to original
        // update the list
        list_files.borrow_mut().set_files(orignal_list.clone()) ;
        list_files.borrow().update_list() ;
    }));

    window.add_action(&sort_action);
    window.add_action(&reset_action);

    window.present();
}
