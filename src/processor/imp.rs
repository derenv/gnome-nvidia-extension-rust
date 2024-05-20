// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Processor)
 *
 * Made:
 * 18/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/src/g_object_properties.md>
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/custom_button/imp.rs>
 * <https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/custom_button/mod.rs>
 */
// Imports
// std
use std::sync::OnceLock;
use std::cell::Cell;
// gtk-rs
use gtk::{
    prelude::*, subclass::prelude::*
};
use adwaita::glib;
use glib::{
    ParamSpec,
    value::Value
};

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct Processor {
    base_call: Cell<String>,
    start_call: Cell<String>,
    middle_call: Cell<Option<String>>,
    end_call: Cell<String>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Processor {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustProcessor";
    // the actual GObject that will be created
    type Type = super::Processor;
    // Parent GObject we inherit from
    type ParentType = gtk::Widget;
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
impl ObjectImpl for Processor {
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
                glib::ParamSpecString::builder("base-call").build(),
                glib::ParamSpecString::builder("start-call").build(),
                glib::ParamSpecString::builder("middle-call").build(),
                glib::ParamSpecString::builder("end-call").build()
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
        value: &Value,
        pspec: &ParamSpec
    ) {
        //println!("setting: {:?}", pspec.name());//TEST

        match pspec.name() {
            "base-call" => match value.get() {
                Ok(input_base_call) => {
                    self.base_call.replace(input_base_call);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "start-call" => match value.get() {
                Ok(input_start_call) => {
                    self.start_call.replace(input_start_call);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "middle-call" => match value.get() {
                Ok(input_middle_call) => {
                    self.middle_call.replace(input_middle_call);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "end-call" => match value.get() {
                Ok(input_end_call) => {
                    self.end_call.replace(input_end_call);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
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
            "base-call" => {
                //TODO: this seems ridiculous..
                let value: String = self.base_call.take();

                self.base_call.set(value.clone());

                value.to_value()
            }
            "start-call" => {
                //TODO: this seems ridiculous..
                let value: String = self.start_call.take();

                self.start_call.set(value.clone());

                value.to_value()
            }
            "middle-call" => {
                //TODO: this seems ridiculous..
                let value: Option<String> = self.middle_call.take();

                self.middle_call.set(value.clone());

                value.to_value()
            }
            "end-call" => {
                //TODO: this seems ridiculous..
                let value: String = self.end_call.take();

                self.end_call.set(value.clone());

                value.to_value()
            }
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
 * 18/09/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Processor {}
