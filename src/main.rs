mod ui;
mod markdown_renderer;
mod file_manager;
mod sidebar_controls;

use gtk::prelude::*;
use gtk::glib;
use adw::{Application, ApplicationWindow};

const APP_ID: &str = "com.example.MarkdownEditor";

fn main() -> glib::ExitCode {
    // Initialize GTK and Adwaita
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    // Set up custom CSS
    app.connect_startup(|_| {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_string(include_str!("styles/app.css"));
        
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    });

    // Run the application
    app.run()
}