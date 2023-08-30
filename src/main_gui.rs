//# In God We T-Rust.
use super::sorting ;

use std::cell::Cell;
use std::rc::Rc;

use adw::{prelude::*, ActionRow};

use adw::Application;

use adw::glib::StrV;
use gtk::glib::clone;
use gtk::ApplicationWindow;
use gtk::{gdk, glib, StateFlags};


const APP_ID: &str = "Sorting.Fat32";
const TITLE: &str = "Fat Sorting";

pub fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();

}

fn build_ui(app: &Application) {

    let scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .overlay_scrolling(true)
        .vscrollbar_policy(gtk::PolicyType::Always)
        .build();

    let sides = 40;
    let up_down = 30;

    //some thing beautiful
    let clamp = adw::Clamp::builder()
        .margin_bottom(up_down)
        .margin_top(up_down)
        .margin_start(sides)
        .margin_end(sides)
        .maximum_size(800)
        .can_focus(true)
        .build();

    clamp.set_width_request(300);
    scroll.set_child(Some(&clamp));

    // box to hold list items
    let gtkbox = gtk::ListBox::builder()
        .valign(gtk::Align::Start)
        .css_classes(StrV::from(["boxed-list"]))
        .selection_mode(gtk::SelectionMode::None)
        .build();


    let overlay = gtk::Overlay::builder().focus_on_click(true).build();
    overlay.set_child(Some(&scroll));

    let mut row: ActionRow = ActionRow::new();
    let files = sorting::sort_files();

    for (i , file) in files.iter().enumerate() {
        row = list_row(&format!("{i}.{}" , file.file_name().unwrap().to_str().unwrap()), &gtkbox );
    }
    let drop_target = gtk::DropTarget::new(row.type_(), gdk::DragAction::MOVE);

    drop_target.connect_drop(clone!(@strong gtkbox => move |_,value,_x ,y|{
        println!("droped at {y}");
        let target_row = match gtkbox.row_at_y(y as i32){
            Some(row) => row ,
            None => {println!("exited from the worng place"); return false}
        };
        let target_index = target_row.index() ;

        let value = match value.get::<adw::ActionRow>() {
            Ok(v) => v ,
            Err(e) => {println!("exited from the right place{e:?}");return false}
        };

        println!("{:?}", value.last_child().unwrap());

        gtkbox.remove(&value);
        gtkbox.insert(&value, target_index);
        target_row.set_state_flags(StateFlags::NORMAL, true);


        true
    }));

    gtkbox.add_controller(drop_target);
    clamp.set_child(Some(&gtkbox));


    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(800)
        .default_height(600)
        .width_request(400)
        .height_request(400)
        .child(&overlay)
        .build();

    window.present();

    let drag_scroll_ctl = gtk::DropControllerMotion::builder()
        .propagation_phase(gtk::PropagationPhase::Bubble)
        .build();
  
    drag_scroll_ctl.connect_motion(clone!(@weak overlay , @strong scroll => move |_,_,y|{

        let y_end =scroll.height() as f64;
        const HOT_AREA : f64 = 100.0 ;
        let accel ;
        let f = 8.0;

        if y <= HOT_AREA { 
            accel = - (HOT_AREA - y) /f;
            let vadj = scroll.vadjustment();
            vadj.set_value(vadj.value() + accel);
         }
        else if y >= y_end - HOT_AREA { 
            accel = -(y-y_end )/ f ;
            let vadj = scroll.vadjustment();
            vadj.set_value(vadj.value() + accel);
        }
    }));

    overlay.add_controller(drag_scroll_ctl);



    utils::list_drives(); 
    
  
}

fn list_row(label: &str, list: &gtk::ListBox) -> ActionRow {
    let row = adw::ActionRow::builder().title(label).build();
    let icon = gtk::Image::builder()
        .icon_name("format-justify-center")
        .build();
    row.add_prefix(&icon);

    list.append(&row);

    let drag_x = Rc::new(Cell::new(0.0));
    let drag_y = Rc::new(Cell::new(0.0));

    let drop_controller = gtk::DropControllerMotion::builder().build();
   
    let drag_source = gtk::DragSource::builder()
        .actions(gdk::DragAction::MOVE)
        .build();

    drag_source.connect_prepare(
        clone!(@strong drag_x , @strong drag_y , @weak row  => @default-return None,move  |_ ,x,y|{
            drag_x.set(x);
            drag_y.set(y);
            println!("prepeaing to drag");

            Some(gdk::ContentProvider::for_value(&row.to_value()))

        }),
    );

    drag_source.connect_drag_begin(clone!(@strong row => move|_ , drag| {
        println!("draging a row");
        let allocation = row.allocation();
        let drag_wiget = gtk::ListBox::builder().css_classes(StrV::from(["boxed-list"])).build();
        let drag_row = adw::ActionRow::builder().title(row.title()).width_request(allocation.width()).height_request(allocation.height()).build();

        drag_row.add_prefix(&icon);
        drag_wiget.append(&drag_row);
        drag_wiget.drag_highlight_row(&drag_row);
        // drag.set_hotspot(drag_x.get() as i32, drag_y.get() as i32);
        drag.set_hotspot(0,0);
    }));

    drop_controller.connect_enter(clone!(@strong row , @strong list  => move|_,_,_y|{
        list.drag_highlight_row(&row);
       //println!("highlighting a row at {y}");
    }));

    drop_controller.connect_leave(clone!(@strong row , @strong list  => move|_|{
        list.drag_unhighlight_row();
    }));


 
    row.add_controller(drag_source);
    row.add_controller(drop_controller);


    row
}
