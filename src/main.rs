use gtk4::glib;
use gtk4::prelude::*;

mod mainmenu;
use crate::mainmenu::menu;

fn on_activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    let BoxM = menu::Box();
    window.set_child(Some(&BoxM));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("com.github.wilwe21.DuoLango")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
