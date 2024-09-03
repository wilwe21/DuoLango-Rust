use gtk::prelude::*

pub fn TopBar(label: &str) -> gtk::Box {
    let Bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let name = gtk::Label::builder()
        .label(label)
        .build();
    Bar.append(name);

    Bar
}
