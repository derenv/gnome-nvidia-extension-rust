// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Property)
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
// Custom GObjects
mod imp;

// Imports
// std
//
// gtk-rs
use gtk::{
    prelude::*,
    glib
};
use glib::{
    value::FromValue,
    Object
};

// Modules
use crate::formatter::Formatter;
use crate::processor::Processor;

// GObject wrapper for Property
glib::wrapper! {
    pub struct Property(ObjectSubclass<imp::Property>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
 * Trait Name:
 * Property
 *
 * Description:
 * Trait shared by all properties
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
impl Property {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new Property object
     *
     * Made:
     * 05/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * processor, name, callExtension, icon, formatter, gpuCount
     *     ^      skip  utilization.gpu  ??     ^         i32
     *
     * given proc and gpuCount
     */
    pub fn new(
        processor: &Processor,
        formatter: &Formatter,
        id: &str
    ) -> Self {
        // Create Object        
        let obj: Property = Object::builder::<Property>().build();

        // Set properties
        obj.set_property("processor", processor);
        obj.set_property("formatter", formatter);

        obj.set_property("id", String::from(id));

        obj
    }

    /**
     * Name:
     * parse
     *
     * Description:
     * Run processor with passed uuid and stored formatter + property name, then return to caller
     *
     * Made:
     * 12/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn parse(
        self,
        uuid: &str
    ) -> Option<String> {
        // println!("UUID: `{}`", uuid); //TEST
        // Grab formatter & processor
        let formatter: Formatter = self.property("formatter");
        let processor: Processor = self.property("processor");
        // Grab property name
        let mut property: String = self.property("id");
        // println!("ID: `{}`", property); //TEST
        if let "GPUUtilization.gpu" | "GPUUtilization.mem" = property.as_str() {
            property = String::from("GPUUtilization");
        }
        // println!("NEW ID: `{}`", property); //TEST

        // Give processor the uuid and property we want
        match processor.process(Some(uuid), Some(&property)) {
            Ok(result) => {
                // println!("PROCESS COMPLETE"); //TEST

                match result {
                    Some(valid_result) => {
                        // println!("RESULT IS NOT NONE"); //TEST
                        // println!("RESULT: `{:?}`", valid_result); //TEST
                        // println!("RESULT len: `{}`", valid_result.len()); //TEST
                        // println!("RESULT[0]: `{}`", valid_result[0].to_owned()); //TEST

                        // Catch these as formatting screws them up
                        let mut clean_required: bool = true;
                        if let "gpu_name" | "GPUUtilization" = property.as_str() {
                            clean_required = false;
                        }
                        // Format returned property using formatter and then return
                        match formatter.format(valid_result[0].to_owned(), clean_required) {
                            Some(formatted_result) => return Some(formatted_result),
                            None => return None,
                        }
                    }
                    None => panic!("Processor error..."),
                }
            }
            Err(err) => panic!("Processor error: `{}`", err.message()),
        }
    }

    /**
     * Name:
     * get_value
     *
     * Description:
     * Get a property and return it's value
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
    pub fn get_value<T: for<'a> FromValue<'a> + 'static>(
        &self,
        name: &str
    ) -> T {
        // Return the value of the property
        self.property::<T>(name)
    }

    // /**
    //  * Name:
    //  * update_value
    //  *
    //  * Description:
    //  * Update a property with a new value
    //  *
    //  * Made:
    //  * 29/10/2022
    //  *
    //  * Made by:
    //  * Deren Vural
    //  *
    //  * Notes:
    //  *
    //  */
    // pub fn update_value<T: Into<Variant>>(
    //     &self,
    //     property_name: &str,
    //     value: T
    // ) {
    //     // Update the property with new value
    //     self.set_property(property_name, value);
    // }
}

/**
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
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
impl Default for Property {
    fn default() -> Self {
        Self::new(&Processor::new("", "", None, ""), &Formatter::default(), "")
    }
}
