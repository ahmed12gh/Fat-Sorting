use adw::prelude::*;
use adw::ActionRow;
use glib::clone;
use gtk::{gdk, glib};
use std::{cell::Cell, rc::Rc};

// I am Done Here.

/// create an adwaita action row that supports drag and drop its `titile` is file name
/// return a actionrow that should be appended to the list
pub fn create_list_row(label: &str, list: &gtk::ListBox) -> ActionRow {
    let label = label.replace("&", "&amp;");
    let row = adw::ActionRow::builder()
        .title(label)
        .use_markup(true)
        .build();
    let icon = gtk::Image::builder()
        .icon_name("list-drag-handle-symbolic")
        .build();
    row.add_prefix(&icon);

    let drop_controller = gtk::DropControllerMotion::builder().build();

    let drag_source = gtk::DragSource::builder()
        .actions(gdk::DragAction::MOVE)
        .build();

    // capture the postion in which the row is draged from
    let drag_x = Rc::new(Cell::new(0.0)); // for drag icon not working at the momnet
    let drag_y = Rc::new(Cell::new(0.0));

    drag_source.connect_prepare(
        clone!( @weak row  ,@weak drag_x , @weak drag_y  => @default-return None ,move  |_ ,x,y|{
            drag_x.set(x);
            drag_y.set(y);
            Some(gdk::ContentProvider::for_value(&row.to_value()))
        }),
    );
    // create the wiget that is similar to the row being draged as a way of providing some feedback
    drag_source.connect_drag_begin(clone!( @weak row => move |_,drag|{
        println!("draging a row");
        let allocation = row.allocation();
        let drag_wiget = gtk::ListBox::builder().build();
        drag_wiget.add_css_class("boxed-list");

        let drag_row = adw::ActionRow::builder()
            .title(row.title())
            .width_request(allocation.width())
            .height_request(allocation.height())
            .build();

        drag_row.add_prefix(&icon);
        drag_wiget.append(&drag_row);
        drag_wiget.drag_highlight_row(&drag_row);

        let drag_icon = gtk::DragIcon::for_drag(drag);
        drag_icon.set_property("child", drag_wiget);

        drag.set_hotspot(drag_x.get() as i32 ,drag_y.get() as i32);
    }));

    //highlight the current row that is hoverd over
    drop_controller.connect_enter(clone!(@strong row , @strong list  => move|_,_,_|{
        list.drag_highlight_row(&row);
    }));

    // remove the highlighting when hovering over an other row
    drop_controller.connect_leave(clone!(@strong row , @strong list  => move|_|{
        list.drag_unhighlight_row();
    }));

    row.add_controller(drag_source);
    row.add_controller(drop_controller);

    row
}

// drag_source.connect_drag_begin(clone!(@strong row => move|_ , drag| {
