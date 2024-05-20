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
// Declare module
extern crate gtk4_nvidia_monitor_rust;

// Imports
// std
//
// gtk-rs
use adwaita::{
    prelude::ApplicationExtManual,
    Application
};

/**
 * Name:
 * main
 *
 * Description:
 * Create application using lib.rs functions and run
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
fn main() {
    // Intialise GTK & Create a new application
    let app: Application = gtk4_nvidia_monitor_rust::create_app();

    // Run the application
    println!("{:?}", app.run());
}
