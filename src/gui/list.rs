use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib::clone;
use gtk::{gdk, StateFlags};

use crate::gui::create_list_row;


const HOT_AREA: f64 = 100.0;
const F: f64 = 10.0;

pub struct  ListOfFiles  { 
   files : Vec<PathBuf> ,
   list_box : gtk::ListBox  
}


impl ListOfFiles {
    pub fn new() -> Self{
        ListOfFiles { files : Vec::new() ,list_box : gtk::ListBox::new()  }
    } 

    pub fn set_files(&mut self , files : Vec<PathBuf>){ 
        self.files = files ;
    }

    pub fn create_list(&mut self) -> gtk::ScrolledWindow { 
        create_list(&self.files.clone() , self)
    } 


    pub fn update_list(&self) { 
        while let Some(child) = self.list_box.last_child()  {
            self.list_box.remove(&child) ;
        }
        for (_i,file) in self.files.iter().enumerate() {
            let row = create_list_row(
                &format!("{}", file.file_name().unwrap().to_str().unwrap()),
                &self.list_box,
            );
            self.list_box.append(&row)
        }
    }
    

    pub fn return_sorted_list(& mut self) -> Vec<PathBuf>{
        let mut sorted_files: Vec<PathBuf> = Vec::new();
        for i in 0..self.files.len() { 
            //get list row at index 
            let row   = self.list_box.row_at_index(i as i32).unwrap() ;
            let title : String = row.property("title");
            dbg!(&title);

            sorted_files.push(get_pathbuf_by_filename(&title, self).unwrap());

        }
        self.files = sorted_files.clone() ;
        sorted_files
    }
}

fn create_list(files : &Vec<PathBuf> , listoffiles : & mut ListOfFiles ) -> gtk::ScrolledWindow{ 

    let scroll = create_scroll() ; // enable scrolling to to the list 
    let clamp = create_clamp() ; // clamp the area to specfic width 
    let gtkbox = create_gtk_list() ; 

    
    for (_i, file) in files.iter().enumerate() {
        let row = create_list_row(
            &format!("{}", file.file_name().unwrap().to_str().unwrap()),
            &gtkbox,
        );
   
        gtkbox.append(&row)
    }

    let drop_target = gtk::DropTarget::new(adw::ActionRow::static_type() , gdk::DragAction::MOVE);
    drop_target.connect_drop(clone!(@strong gtkbox => move |_,value,_x ,y|{
        let target_row = match gtkbox.row_at_y(y as i32){
            Some(row) => row ,
            None => {return false}
        };
        let target_index = target_row.index() ;        
        let value = match value.get::<adw::ActionRow>() {
            Ok(v) => v ,
            Err(_) => {return false}
        };
        gtkbox.remove(&value);
        gtkbox.insert(&value, target_index);
        target_row.set_state_flags(StateFlags::NORMAL, true);

        true
    }));

    gtkbox.add_controller(drop_target);

    scroll.set_child(Some(&clamp));
    clamp.set_child(Some(&gtkbox));

    let drag_scroll_ctl = gtk::DropControllerMotion::builder()
        .propagation_phase(gtk::PropagationPhase::Bubble)
        .build();

    
    // enbale drag scroll at the top and bottem of the window
    let accel = Rc::new(Cell::new(0.0f64));
    drag_scroll_ctl.connect_motion(
        clone!( @strong scroll  , @strong accel=> move |_,_,y|{
            let y_end =scroll.height() as f64;
            if y <= HOT_AREA {
                accel.set(- (HOT_AREA - y) /F);
             }
            else if y >= y_end - HOT_AREA {
                accel.set((HOT_AREA+ y-y_end )/ F) ;
            }
            scroll_drag(&scroll, accel.get() )
        }),
    );
    scroll.add_controller(drag_scroll_ctl);

    listoffiles.list_box = gtkbox ;

    return scroll; 

}


fn scroll_drag(scroll: &gtk::ScrolledWindow, accel: f64 ) {
    const FACTOR: u8 =3 ;  
    let part_of_accel = accel / FACTOR as f64 ;

    let vadj = scroll.vadjustment();
    for _ in 0..FACTOR { 
        vadj.set_value(vadj.value() + part_of_accel);   
    }
}

fn get_pathbuf_by_filename(filename : &str , listoffiles : &ListOfFiles) -> Option<PathBuf>{ 
    let filename = filename.replace("&amp;", "&");
    for file in &listoffiles.files { 
        if file.file_name().unwrap().to_str().unwrap() == filename { 
            return Some(file.clone());
        }
    }
    None

}

fn create_scroll() -> gtk::ScrolledWindow {

    let scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .overlay_scrolling(true)
        .vscrollbar_policy(gtk::PolicyType::Always)
        .build();

    scroll
}


fn create_clamp() -> adw::Clamp{
    const  SIDES: i32 = 40;
    const UPDOWN: i32 = 30;

    let clamp = adw::Clamp::builder()
        .margin_bottom(UPDOWN)
        .margin_top(UPDOWN)
        .margin_start(SIDES)
        .margin_end(SIDES)
        .maximum_size(800)
        .width_request(300)
        .can_focus(true)
        .build();

    clamp
}

fn create_gtk_list() ->gtk::ListBox{
    let listbox = gtk::ListBox::builder()
    .valign(gtk::Align::Start)
    .selection_mode(gtk::SelectionMode::None) // possible to chnage use keyboard shortcuts 
    .build();
    listbox.add_css_class("boxed-list");
     
    listbox
}