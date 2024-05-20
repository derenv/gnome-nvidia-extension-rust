// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Formatter)
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
use adwaita::{
    gio, glib
};
use gio::Settings;
use glib::Object;
use gtk::subclass::prelude::*;

// Modules
use crate::APP_ID;

// GObject wrapper for Formatter
glib::wrapper! {
    pub struct Formatter(ObjectSubclass<imp::Formatter>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
 * Trait Name:
 * Formatter
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
impl Formatter {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new Formatter object
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
    pub fn new(
        func: fn(Vec<String>, Option<Vec<(String, String)>>) -> Option<String>
    ) -> Self {
        // Create Object        
        let obj: Formatter = Object::builder::<Formatter>().build();

        // Set properties
        obj.imp().func.set(Some(func));

        obj
    }

    /**
     * Name:
     * setup_settings
     *
     * Description:
     * Load settings for APP_ID
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
    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    /**
     * Name:
     * format
     *
     * Description:
     * Given some valid string value, remove non-digit chars (excluding points) and apply formatting
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
    pub fn format(self, value: String, clean: bool) -> Option<String> {
        //println!("FORMATTING");//TEST

        // Apply cleaning if required
        if clean {
            // Remove all non-number characters
            let cleaned_value: String = value
                .chars()
                .filter(|c| {
                    // check if (base 10) digit
                    if c.is_digit(10) {
                        true
                    } else {
                        // check if full-stop
                        c.eq(&'.')
                    }
                })
                .collect();

            // Convert to float
            match cleaned_value.parse::<f64>() {
                Ok(parsed_value) => {
                    // Apply any valid formatting
                    match self.imp().func.take() {
                        Some(func) => {
                            // Grab all format info from settings (this is done here to keep in one place)
                            //===
                            // Temperature format
                            let temp_format: i32 = self.imp().get_setting::<i32>("tempformat");
                            let mut params: Vec<(String, String)> = vec![];
                            if let 0 = temp_format {
                                params.push((String::from("tempformat"), String::from("C")));
                            } else if let 1 = temp_format {
                                params.push((String::from("tempformat"), String::from("F")));
                            }
                            //TODO: ???
                            //
                            //===

                            // Use function
                            let result: Option<String>;
                            if !params.is_empty() {
                                result = func(vec![parsed_value.to_string()], Some(params));
                            } else {
                                result = func(vec![parsed_value.to_string()], None);
                            }
                            // Return it!
                            self.imp().func.set(Some(func));

                            // Return final result
                            result
                        }
                        None => panic!("Missing formatting function!"),
                    }
                }
                Err(err) => {
                    // Catch any errors..
                    println!("Not a valid number: {}", err);

                    None
                }
            }
        } else {
            // Apply any valid formatting
            match self.imp().func.take() {
                Some(func) => {
                    // Grab all format info from settings (this is done here to keep in one place)
                    //===
                    let mut _params: Vec<(String, String)> = vec![]; //mut
                                                                     //TODO: ???
                                                                     //
                                                                     //===

                    // Use function
                    let result: Option<String>;
                    if !_params.is_empty() {
                        result = func(vec![value], Some(_params));
                    } else {
                        result = func(vec![value], None);
                    }
                    // Return it!
                    self.imp().func.set(Some(func));

                    // Return final result
                    result
                }
                None => panic!("Missing formatting function!"),
            }
        }
    }
}

/**
 * Trait Name:
 * Default
 *
 * Description:
 * Default object
 *
 * Made:
 * 08/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for Formatter {
    fn default() -> Self {
        let func: fn(Vec<String>, Option<Vec<(String, String)>>) -> Option<String> =
            |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                Some(String::from(input.get(0).unwrap()))
            };

        Self::new(func)
    }
}
