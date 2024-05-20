// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (CustomButton)
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
// Imports
// std
use std::sync::OnceLock;
// gtk-rs
use gtk::{
    glib,
    subclass::prelude::*
};
use glib::{
    ParamSpec,
    value::Value
};

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct CustomButton;

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustCustomButton";
    // the actual GObject that will be created
    type Type = super::CustomButton;
    // Parent GObject we inherit from
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
     *
     * ParamSpec Examples:
     * glib::ParamSpecString::builder("icon").build(),
     * glib::ParamSpecUInt::builder("gpu_count").build(),
     * glib::ParamSpecString::builder("call_extension").build(),
     * TODO: these are from property class
     * glib::ParamSpecBoxed::builder("processor").build(),
     * glib::ParamSpecObject::builder("formatter").build(),
     */
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                //
            ]
        })

        //println!("PROPERTIES: {:?}", PROPERTIES);//TEST
        //println!("trying to add `base_call`: {:?}", glib::ParamSpecString::builder("base_call").build());//TEST
    }

    /**
     * Name:
     * set_property
     *
     * Description:
     * Mutator for custom GObject properties
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn set_property(
        &self,
        _id: usize,
        _value: &Value,
        pspec: &ParamSpec
    ) {
        //println!("setting: {:?}", pspec.name());//TEST

        match pspec.name() {
            //
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessir for custom GObject properties
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn property(
        &self,
        _id: usize,
        pspec: &ParamSpec
    ) -> Value {
        //println!("getting: {:?}", pspec.name());//TEST

        match pspec.name() {
            //
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {}
