// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * build.rs
 *
 * Description:
 * Build script for Nvidia Gnome Extension (Rust)
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * 
 */
// Imports
use glib_build_tools::compile_resources;

/**
 * Name:
 * main
 *
 * Description:
 * Runs pre-build
 *
 * Made:
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
fn main() {
    // UI
    println!("..Compiling UI resources into `.gresource` file");
    compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "nvidiamonitorrust.gresource",
    );
}
