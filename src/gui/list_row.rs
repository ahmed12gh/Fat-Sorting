
use adw::prelude::* ; 
use adw::ActionRow ; 
use gtk::{glib ,gdk};
use glib::clone ;


/// create an adwaita action row that supports drag and drop its `titile` is file name 
/// return a actionrow that should be appended to the list 
pub fn create_list_row(label: &str, list: &gtk::ListBox) -> ActionRow {
    let label = label.replace("&", "&amp;");
    let row = adw::ActionRow::builder().title(label).use_markup(true).build();
    let icon = gtk::Image::builder()
        .icon_name("list-drag-handle-symbolic")
        .build();
    row.add_prefix(&icon);

  
    let drop_controller = gtk::DropControllerMotion::builder().build();

    let drag_source = gtk::DragSource::builder()
        .actions(gdk::DragAction::MOVE)
        .build();

    drag_source.connect_prepare(
        clone!( @weak row  => @default-return None ,move  |_ ,_,_|{
            Some(gdk::ContentProvider::for_value(&row.to_value()))
        }),
    );
    
    drop_controller.connect_enter(clone!(@strong row , @strong list  => move|_,_,_|{
        list.drag_highlight_row(&row);
    }));

    drop_controller.connect_leave(clone!(@strong row , @strong list  => move|_|{
        list.drag_unhighlight_row();        
    }));

    row.add_controller(drag_source);
    row.add_controller(drop_controller);

    row
}


    // let drag_x = Rc::new(Cell::new(0.0)); // for drag icon not working at the momnet 
    // let drag_y = Rc::new(Cell::new(0.0));


 // drag_source.connect_drag_begin(clone!(@strong row => move|_ , drag| {
    //     println!("draging a row");
    //     let allocation = row.allocation();
    //     let drag_wiget = gtk::ListBox::builder().css_classes(glib::StrV::from(["boxed-list"])).build();
    //     let drag_row = adw::ActionRow::builder().title(row.title()).width_request(allocation.width()).height_request(allocation.height()).build();

    //     drag_row.add_prefix(&icon);
    //     drag_wiget.append(&drag_row);
    //     drag_wiget.drag_highlight_row(&drag_row);
    //     drag.set_hotspot(0,0);
    // }));