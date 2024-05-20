// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Formatter)
 *
 * Made:
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * Most of this left blank, may add fields later
 */
// Imports
// std
use std::sync::OnceLock;
use std::cell::{
    Cell, OnceCell
};
// gtk-rs
use gtk::subclass::prelude::*;
use adwaita::{
    gio, glib,
    prelude::*,// subclass::prelude::*
};
use gio::Settings;
use glib::{
    ParamSpec,
    variant::FromVariant,
    value::Value
};

// Modules
//

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct Formatter {
    pub settings: OnceCell<Settings>,
    pub func: Cell<Option<fn(Vec<String>, Option<Vec<(String, String)>>) -> Option<String>>>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Formatter {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustFormatter";
    // the actual GObject that will be created
    type Type = super::Formatter;
    // Parent GObject we inherit from
    type ParentType = gtk::Widget;
}

impl Formatter {
    /**
     * Name:
     * get_setting
     *
     * Description:
     * Generic function for getting setting value
     *
     * Made:
     * 30/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn get_setting<T: FromVariant>(&self, name: &str) -> T {
        // Return the value of the property
        match self.settings.get() {
            Some(settings) => settings.get::<T>(name),
            None => panic!("`settings` should be set in `setup_settings`."),
        }
    }
}

/**
 * Trait Name:
 * ObjectImpl
 *
 * Description:
 * Trait shared by all GObjects
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
impl ObjectImpl for Formatter {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
     *
     * Made:
     * 18/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj: glib::BorrowedObject<super::Formatter> = self.obj();
        obj.setup_settings();
    }

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

/**
 * Trait Name:
 * WidgetImpl
 *
 * Description:
 * Trait shared by all widgets
 *
 * Made:
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Formatter {}
