extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
use std::env::args;

mod counter;
pub use crate::counter::Counter;

fn main() {
    // The simplest way to initialize a new gtk application
    let application =
        Application::new(None, Default::default()).expect("failed to initialize GTK application");

    // UI initialization
    application.connect_activate(|app| {
        // Window
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK Program");
        window.set_default_size(350, 70);

        // State
        // Have to use a reference counted var to allow for copying done in connections
        use std::sync::Arc;
        let counter = Arc::new(Counter::new(0));

        // Containers
        let container = Box::new(gtk::Orientation::Vertical, 3);

        // Header
        let header = gtk::HeaderBar::new();
        let label = Label::new(Some(&format!("Starting at {}", counter.get())));

        // Content
        let inc_btn = Button::new_with_label("Increment");
        let dec_btn = Button::new_with_label("Decrement");

        // Connections
        // Using blocks to create a new scope
        // Makes naming easier (both get their own label_clone and counter_clone)
        {
            let label_clone = label.clone();
            let counter_clone = counter.clone();
            inc_btn.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Incremented to {}", val));
            });
        }
        {
            let label_clone = label.clone();
            let counter_clone = counter.clone();
            dec_btn.connect_clicked(move |_| {
                let val = counter_clone.decrement();
                label_clone.set_label(&format!("Decremented to {}", val));
            });
        }

        // Compose pieces together
        header.add(&label);
        container.add(&header);
        container.add(&inc_btn);
        container.add(&dec_btn);
        window.add(&container);

        // Display all widgets
        window.show_all();
    });

    // Passing arguments to the app
    application.run(&args().collect::<Vec<_>>());
}
