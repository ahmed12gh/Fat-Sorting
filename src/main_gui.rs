use adw::{prelude::*, Application};
use gtk::{ApplicationWindow, Button };
use gtk::glib::{clone} ;
use std::{self, cell::Cell ,rc::Rc }  ;
use gtk::gdk ;

const APP_ID: &str = "agent47.AC.Sorty";
const TITLE: &str = "Sorty";

pub fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {

    let mut list_vec : Vec<adw::ActionRow>  = Vec::new() ;
    for i in 0..10 {

        let row = adw::ActionRow::builder().build() ;
        row.set_property("title" , format!("row {i}")) ;
        list_vec.push(row);
    }

    let list = gtk::ListBox::builder().build() ;
    for row in &list_vec {
        list.append(row);

    }


    let drag_target = gtk::DropTarget::new(list.type_() , gdk::DragAction::MOVE) ;
    list.add_controller(drag_target) ;

    for row in &list_vec {
        let mut drag_x = 0.0 ; 
        let mut drag_y= 0.0 ; 

        let drop_controller = gtk::DropControllerMotion::new() ;
        let drag_source = gtk::DragSource::builder().actions(gdk::DragAction::MOVE).build();

        row.add_controller(drop_controller);
        

        row.add_controller(drag_source) ;
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .child(&list)
        .build();

    window.present();
}
