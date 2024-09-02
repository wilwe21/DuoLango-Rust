use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;

pub mod menu { 
    pub fn Box() -> gtk4::Box{
        let Boxm = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        let Boxm2 = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        let butt = gtk4::Button::with_label("chuj");
        Boxm2.append(butt);

        Boxm
    }
}
