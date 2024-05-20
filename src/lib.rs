// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * gtk4-nvidia-monitor-rust
 *
 * Description:
 * GTK-rs app for monitoring Nvidia GPU statistics
 *
 * Made:
 * 12/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
// Modules
mod formatter;
mod gpu_page;
mod mainwindow;
mod processor;
mod property;
mod provider;
mod subprocess;
use mainwindow::MainWindow;
mod custom_button;
mod modificationwindow;
mod settingswindow;

// Imports
// std
//
// gtk-rs
use adwaita::{
    gio,
    prelude::*,
    Application
};
use gdk::Display;
use gio::resources_register_include;
use gtk::{
    CssProvider,
    style_context_add_provider_for_display
};

// Constants
const APP_ID: &str = "com.gtk_d.NvidiaMonitorRust";

/**
 * Name:
 * main
 *
 * Description:
 * Load resources, initialise GTK, create application and connect signals
 *
 * Made:
 * 13/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
pub fn create_app() -> Application {
    // Resources
    resources_register_include!("nvidiamonitorrust.gresource")
        .expect("Failed to register resources.");

    // Intialise GTK
    gtk::init().expect("Failed to initialise gtk");

    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to signals of `app`
    app.connect_startup(setup_shortcuts);
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Return the application
    app
}

/**
 * Name:
 * setup_shortcuts
 *
 * Description:
 * Add keyboard shortcuts to the program
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/main.rs>
 * <https://gtk-rs.org/gtk4-rs/stable/latest/book/todo_3.html>
 *
 * <https://gtk-rs.org/gtk4-rs/git/book/actions.html>
 * <https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Window.html#actions>
 */
fn setup_shortcuts(app: &Application) {
    //app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    //app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    //app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
    //app.set_accels_for_action("win.filter('Close')", &["<Ctrl>q"])
    app.set_accels_for_action("window.close", &["<Ctrl>q"]);
    app.set_accels_for_action("window.toggle-maximized", &["<Ctrl>w"]);
    app.set_accels_for_action("window.minimize", &["<Ctrl>m"]);
}

/**
 * Name:
 * load_css
 *
 * Description:
 * Load css from file and add as style provider
 *
 * Made:
 * 23/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    // Add the provider to the default screen
    style_context_add_provider_for_display(
        &Display::default().expect("..Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/**
 * Name:
 * build_ui
 *
 * Description:
 * Build the main window (given the application object) and run
 *
 * Made:
 * 13/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window: MainWindow = MainWindow::new(app);

    // Create custom action to close window (for shortcuts)
    // Was a "close" replica
    /*
    let action_close = SimpleAction::new("something", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_close);
    */

    // Present window
    window.show();

    /*
    // App Indicator
    //let mut indicator = AppIndicator::new("Nvidia App", "");
    //indicator.set_status(AppIndicatorStatus::Active);
    //let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    //indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    //indicator.set_icon_full("rust-logo", "icon");
    //indicator.set_menu(&mut menu);
     */
}
