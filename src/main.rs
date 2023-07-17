use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use std::env;

fn input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let html = input();
    println!("{:?}", html);

    let app = Application::builder().application_id("mmbr").build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("mmbr")
        .build();

    window.present();
}
