use gtk::prelude::*;

mod mainmenu;
use crate::mainmenu::menu;

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .title("DuoLango")
        .application(application)
        .child(&mainBox)
        .build();
    window.show();
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
