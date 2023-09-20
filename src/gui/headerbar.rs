// In GOD We T-Rust ;

use adw::HeaderBar;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::Button;

//TODO : add a settings menu and about menu and another sort menu by alpahbic orders and custom

pub fn create_header_bar() -> HeaderBar {
    let headerbar = HeaderBar::builder().build();

    // the blue sort button
    let sort = Button::with_label("Sort");
    sort.add_css_class("suggested-action");
    sort.connect_clicked(clone!( @weak sort => move |_|{
        sort.activate_action("win.sort", None).expect("");
    }));

    let reset = Button::with_label("Reset");
    reset.connect_clicked(clone!( @weak reset => move |_|{
        reset.activate_action("win.reset", None).expect("fuckme");
    }));

    headerbar.pack_start(&sort);
    headerbar.pack_end(&reset);

    headerbar
}

// In GOD We T-Rust ;
