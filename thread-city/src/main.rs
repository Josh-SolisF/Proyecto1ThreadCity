use gtk::Application;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};

mod cityblock;
mod vehicle;
mod city;
mod GUI;

fn main() {
    let app = Application::builder()
        .application_id("com.helberth.citygtk")
        .build();

    app.connect_activate(|app| {
        let hooks = crate::GUI::main::make_hooks_from_world();
        crate::GUI::main::build_ui(app, hooks);
    });

    app.run();
}