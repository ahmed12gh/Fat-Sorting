// In GOD We T-Rust ;

use gtk::prelude::*; 
use adw::HeaderBar ;
use gtk::Button ;
use gtk::glib;
use glib::clone ;


pub fn create_header_bar() -> HeaderBar { 
    
    let headerbar = HeaderBar::builder().build();

    let sort = Button::with_label("Sort") ;
    sort.add_css_class("suggested-action");
    sort.connect_clicked(clone!( @weak sort => move |_|{
        sort.activate_action("win.sort", None).expect("");
    }));
    
    let reset = Button::with_label("Reset") ;
    reset.add_css_class("destructive-action");  
    reset.connect_clicked(clone!( @weak reset => move |_|{
        reset.activate_action("win.reset", None).expect("fuckme");
    }));
    
    headerbar.pack_start(&sort);
    headerbar.pack_end(&reset);
    
    headerbar
    
    
}


// In GOD We T-Rust ;