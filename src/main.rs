mod details_page;
mod note_row;
mod window;
mod create_page;
mod note_object;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

static APP_ID: &str = "com.jasper.ji.gtk.rs.notes";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gtk4-rs-notes.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}