use gtk::prelude::*;

mod mainmenu;
use crate::mainmenu::menu;

fn on_activate(application: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let bbb = gtk::Box::new(gtk::Orientation::Vertical, 0);
    mainBox.append(&bbb);
    let butter = gtk::Button::builder()
        .label("dupa")
        .build();
    bbb.append(&butter);
    let window = gtk::ApplicationWindow::builder()
        .title("DuoLango")
        .application(application)
        .child(&mainBox)
        .build();
    unsafe {
        butter.connect_clicked(move |_| BoxNewChild(&mainBox, &menu::MM(), &bbb));
        window.show();
    }
}

fn BoxNewChild(parrent: &gtk::Box, child: &gtk::Box, ded: &gtk::Box) {
    parrent.remove(ded);
    parrent.append(child);
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.DuoLango")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
