// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/* *
 * Name:
 * integration_tests_1.rs
 *
 * Description:
 * Integration tests for Nvidia Gnome Extension (Rust)
 *
 * Made:
 * 26/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Imports
extern crate gtk4_macros;

// Declare module
extern crate gtk4_nvidia_monitor_rust;

// Imports
use adwaita::{
    prelude::ApplicationExtManual, Application
};

pub fn init_ui() -> Application {
    // Intialise GTK & Create a new application
    gtk4_nvidia_monitor_rust::create_app()
}

/*
 * Integration tests
 */
#[gtk::test]
fn test_test() {
    // Intialise GTK & Create a new application
    let app: Application = init_ui();

    // Run the application
    assert_eq!(app.run(), 0);
}
