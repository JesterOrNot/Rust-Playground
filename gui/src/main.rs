//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Hello World");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 80);
    let label = gtk::Label::new(Some("Hello World From Rust!"));
    window.add(&label);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.jesterornot"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
