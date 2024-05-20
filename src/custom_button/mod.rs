// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (CustomButton)
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
// Custom GObjects
mod imp;

// Imports
// std
//
// gtk-rs
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        // Create Object
        Object::builder::<CustomButton>().build()
    }
    // pub fn with_label(label: &str) -> Self {
    //     Object::with_label(&[("label", &label)]).expect("Failed to create `CustomButton`.")
    // }
}
