// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (Property)
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
// Imports
// std
use std::sync::OnceLock;
use std::cell::Cell;
// gtk-rs
use gtk::subclass::prelude::*;
use adwaita::glib;
use glib::{
    ParamSpec,
    value::ToValue, value::Value
};

// Modules
use crate::formatter::Formatter;
use crate::processor::Processor;

/// Object holding the State and any Template Children
#[derive(Default)]
pub struct Property {
    processor: Cell<Processor>,
    formatter: Cell<Formatter>,

    id: Cell<String>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Property {
    //Crate+Obj to avoid collisions
    const NAME: &'static str = "NvidiaMonitorRustProperty";
    // the actual GObject that will be created
    type Type = super::Property;
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
impl ObjectImpl for Property {
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
     */
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("id").build(),
                glib::ParamSpecObject::builder::<Processor>("processor").build(),
                glib::ParamSpecObject::builder::<Formatter>("formatter").build(),
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
            "id" => match value.get() {
                Ok(input_id) => {
                    self.id.replace(input_id);
                }
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "processor" => match value.get() {
                Ok(input_processor) => {
                    self.processor.replace(input_processor);
                }
                Err(_) => panic!("The value needs to be of type `Processor`."),
            },
            "formatter" => match value.get() {
                Ok(input_formatter) => {
                    self.formatter.replace(input_formatter);
                }
                Err(_) => panic!("The value needs to be of type `Formatter`."),
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
            "id" => {
                //TODO: this seems ridiculous..
                let value: String = self.id.take();

                self.id.set(value.clone());

                value.to_value()
            }
            "processor" => {
                //TODO: this seems ridiculous..
                let value: Processor = self.processor.take();

                self.processor.set(value.clone());

                value.to_value()
            }
            "formatter" => {
                //TODO: this seems ridiculous..
                let value: Formatter = self.formatter.take();

                self.formatter.set(value.clone());

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
 * 05/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 * leaving blank atm, boilerplate
 */
impl WidgetImpl for Property {}
