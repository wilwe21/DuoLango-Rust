pub mod menu { 
    use gtk::prelude::*;
    pub fn MM() -> gtk::Box {
        let Boxm = gtk::Box::new(gtk::Orientation::Vertical, 0);
        //let bar = top::TopBar("DuoLango");
        let button = gtk::Button::builder()
            .label("Dupa")
            .build();
        //Boxm.append(&bar);
        Boxm.append(&button);

        Boxm
    }
}
