// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (MainWindow)
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
// std
use std::sync::OnceLock;
use std::cell::{
    Cell, OnceCell, RefCell, RefMut, Ref
};
use std::rc::Rc;
// gtk-rs
use gtk::{
    subclass::prelude::*,
    Button, CompositeTemplate, PolicyType, ScrolledWindow, Stack, TemplateChild
};
use adwaita::{
    gio, glib,
    prelude::*, subclass::prelude::*
};
use gio::Settings;
use glib::{
    subclass::Signal, signal::Propagation,
    subclass::InitializingObject, ParamSpec,
    variant::FromVariant, variant::Variant,
    value::Value,
    BorrowedObject
};

// Modules
use crate::{
    settingswindow::SettingsWindow,
    gpu_page::GpuPage, 
    formatter::Formatter, processor::Processor, property::Property, provider::Provider
};

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct SettingsWindowContainer {
    pub window: Option<SettingsWindow>,
    pub open: bool,
}
/// Enum for choosing temperature units
#[derive(Debug, PartialEq, Eq)]
enum TemperatureUnit {
    CELCIUS = 0,
    FAHRENHEIT = 1,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindow {
    // Public
    pub settings: OnceCell<Settings>,
    pub settings_window: Rc<RefCell<SettingsWindowContainer>>,
    pub provider: Cell<Option<Provider>>,

    // Private
    gpu_pages: RefCell<Vec<GpuPage>>,
    gpu_names: RefCell<Vec<String>>,

    // Template Children
    #[template_child]
    pub gpu_stack: TemplateChild<Stack>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionMainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

/**
 * Name:
 * MainWindow
 *
 * Description:
 * Trait shared by all MainWindow objects
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
impl MainWindow {
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
    pub fn get_setting<T: FromVariant>(
        &self,
        name: &str
    ) -> T {
        // Return the value of the property
        match self.settings.get() {
            Some(settings) => settings.get::<T>(name),
            None => panic!("`settings` should be set in `setup_settings`."),
        }
    }

    /**
     * Name:
     * update_setting
     *
     * Description:
     * Generic function for updating setting values
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
    pub fn update_setting<T: Into<Variant> + Clone>(
        &self,
        name: &str,
        value: T
    ) {
        // Fetch settings
        match self.settings.get() {
            Some(settings) => match settings.set(name, &value) {
                Ok(_) => println!("..Setting `{}` updated!", name),
                Err(err) => panic!("..Cannot update `{}` setting: `{}`", name, err),
            },
            None => panic!("..Cannot retrieve settings"),
        }
    }

    /**
     * Name:
     * create_gpu_page
     *
     * Description:
     * Create a new object of type GpuPage and add to stack
     *
     * Made:
     * 07/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn create_gpu_page(
        &self,
        uuid: &str,
        name: &str,
        provider: Provider
    ) {
        // Create new GpuPage object
        let new_page: GpuPage = GpuPage::new(uuid, name, provider);

        // Add to list of pages
        let mut gpu_page_list: RefMut<Vec<GpuPage>> = self.gpu_pages.borrow_mut();
        gpu_page_list.push(new_page);
        match gpu_page_list.last() {
            Some(new_page_ref) => {
                // Create scrollable container
                let scrolled_window: ScrolledWindow = ScrolledWindow::new();
                scrolled_window.set_hscrollbar_policy(PolicyType::Never);
                scrolled_window.set_vscrollbar_policy(PolicyType::Automatic);
                scrolled_window.set_child(Some(new_page_ref));

                // Append new ListBoxRow object to GtkListBox
                self.gpu_stack
                    .add_titled(&scrolled_window, Some(uuid), name);
            }
            None => panic!("COULD NOT FETCH GPU PAGE REF"),
        }
    }

    /**
     * Name:
     * create_provider
     *
     * Description:
     * Creates a provider object of a certain type (given as input parameter)
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn create_provider(
        &self,
        provider_type: i32
    ) -> Provider {
        //println!("CREATING PROVIDER");
        //println!("..PROVIDER TYPE: `{}`", provider_type);

        // Create appropriate provider
        match provider_type {
            0 => {
                // Nvidia Settings and Nvidia SMI
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        Some(String::from(input.get(0).unwrap()))
                                    },
                                ),
                                "gpu_name",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Grab current temperature unit from parameters
                                        let current_unit: TemperatureUnit;
                                        if let Some(valid_params) = params {
                                            if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("C"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            } else if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("F"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::FAHRENHEIT;
                                            } else {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            }
                                        } else {
                                            current_unit = TemperatureUnit::CELCIUS;
                                        }

                                        // Apply formatting
                                        match current_unit {
                                            TemperatureUnit::CELCIUS => {
                                                // Apply temperature unit
                                                output.push(char::from_u32(0x00B0).unwrap());
                                                output.push('C');

                                                // Return result
                                                Some(output)
                                            }
                                            TemperatureUnit::FAHRENHEIT => {
                                                match output.parse::<f64>() {
                                                    Ok(temp) => {
                                                        // Convert to fahrenheit
                                                        let fahrenheit_temp: f64 =
                                                            temp * 9.0 / 5.0 + 32.0;

                                                        // Round down to nearest integer
                                                        let rounded_value: f64 =
                                                            fahrenheit_temp.floor();

                                                        // Convert to string
                                                        let mut f_output: String =
                                                            rounded_value.to_string();

                                                        // Apply temperature unit
                                                        f_output
                                                            .push(char::from_u32(0x00B0).unwrap());
                                                        f_output.push('F');

                                                        // Return result
                                                        return Some(f_output);
                                                    }
                                                    Err(_) => {
                                                        //this should catch "" etc
                                                        println!("Not a valid number");

                                                        return None;
                                                    }
                                                };
                                            }
                                        }
                                    },
                                ),
                                "temperature.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.used",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.total",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.memory",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "fan.speed",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let input_str: String = String::from(input.get(0).unwrap());

                                        // Convert to float
                                        match input_str.parse::<f64>() {
                                            Ok(parsed_value) => {
                                                // Round down to nearest integer
                                                let rounded_value: f64 = parsed_value.floor();

                                                // Convert to string
                                                let mut output: String = rounded_value.to_string();

                                                // Apply formatting
                                                output.push(' ');
                                                output.push('W');

                                                // Return result
                                                Some(output)
                                            }
                                            Err(_) => {
                                                //this should catch "" etc
                                                println!("Not a valid number");

                                                None
                                            }
                                        }
                                    },
                                ),
                                "power.draw",
                            ),
                        ]
                    },
                    0,
                )
            }
            1 => {
                // Nvidia Settings
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let line: String = String::from(input.get(0).unwrap());
                                        let list: Vec<&str> = line.split(", ").collect();
                                        let final_list: Vec<&str> = list[0].split('=').collect(); // This grabs `graphics=2` etc

                                        // Grab item in output that we want
                                        let mut output: String = String::from(final_list[1]);

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "GPUUtilization.gpu",
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                &Formatter::new(
                                    |input: Vec<String>, params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Grab current temperature unit from parameters
                                        let current_unit: TemperatureUnit;
                                        if let Some(valid_params) = params {
                                            if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("C"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            } else if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("F"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::FAHRENHEIT;
                                            } else {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            }
                                        } else {
                                            current_unit = TemperatureUnit::CELCIUS;
                                        }

                                        // Apply formatting
                                        match current_unit {
                                            TemperatureUnit::CELCIUS => {
                                                // Apply temperature unit
                                                output.push(char::from_u32(0x00B0).unwrap());
                                                output.push('C');
                                            }
                                            TemperatureUnit::FAHRENHEIT => {
                                                match output.parse::<f64>() {
                                                    Ok(temp) => {
                                                        // Convert to fahrenheit
                                                        let fahrenheit_temp: f64 =
                                                            temp * 9.0 / 5.0 + 32.0;

                                                        // Round down to nearest integer
                                                        let rounded_value: f64 =
                                                            fahrenheit_temp.floor();

                                                        // Convert to string
                                                        let mut f_output: String =
                                                            rounded_value.to_string();

                                                        // Apply temperature unit
                                                        f_output
                                                            .push(char::from_u32(0x00B0).unwrap());
                                                        f_output.push('F');

                                                        // Return result
                                                        Some(f_output)
                                                    }
                                                    Err(_) => {
                                                        //this should catch "" etc
                                                        println!("Not a valid number");

                                                        None
                                                    }
                                                };
                                            }
                                        }

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "GPUCoreTemp",
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "UsedDedicatedGPUMemory",
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "TotalDedicatedGPUMemory",
                            ),
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let line: String = String::from(input.get(0).unwrap());
                                        let list: Vec<&str> = line.split(", ").collect();
                                        let final_list: Vec<&str> = list[1].split('=').collect(); // This grabs `graphics=2` etc

                                        // Grab item in output that we want
                                        let mut output: String = String::from(final_list[1]);

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "GPUUtilization.mem",
                            ),
                            /*
                            Property::new(
                                &Processor::new("nvidia-settings", "-q=[gpu:", Some("]/"), " -t"),
                                //&Processor::new("nvidia-settings", "-q=[gpu:]/", " -t"),
                                &Formatter::new(|input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                    // Grab input
                                    let mut output: String = String::from(input.get(0).unwrap());

                                    // Apply formatting
                                    output.push(' ');
                                    output.push('%');

                                    // Return result
                                    Some(output)
                                }),
                                "GPUCurrentFanSpeedRPM",
                            ),
                            */
                        ]
                    },
                    1,
                )
            }
            2 => {
                // Nvidia SMI
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        Some(String::from(input.get(0).unwrap()))
                                    },
                                ),
                                "gpu_name",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Grab current temperature unit from parameters
                                        let current_unit: TemperatureUnit;
                                        if let Some(valid_params) = params {
                                            if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("C"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            } else if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("F"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::FAHRENHEIT;
                                            } else {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            }
                                        } else {
                                            current_unit = TemperatureUnit::CELCIUS;
                                        }

                                        // Apply formatting
                                        match current_unit {
                                            TemperatureUnit::CELCIUS => {
                                                // Apply temperature unit
                                                output.push(char::from_u32(0x00B0).unwrap());
                                                output.push('C');
                                            }
                                            TemperatureUnit::FAHRENHEIT => {
                                                match output.parse::<f64>() {
                                                    Ok(temp) => {
                                                        // Convert to fahrenheit
                                                        let fahrenheit_temp: f64 =
                                                            temp * 9.0 / 5.0 + 32.0;

                                                        // Round down to nearest integer
                                                        let rounded_value: f64 =
                                                            fahrenheit_temp.floor();

                                                        // Convert to string
                                                        let mut f_output: String =
                                                            rounded_value.to_string();

                                                        // Apply temperature unit
                                                        f_output
                                                            .push(char::from_u32(0x00B0).unwrap());
                                                        f_output.push('F');

                                                        // Return result
                                                        Some(f_output)
                                                    }
                                                    Err(_) => {
                                                        //this should catch "" etc
                                                        println!("Not a valid number");

                                                        None
                                                    }
                                                };
                                            }
                                        }

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "temperature.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.used",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.total",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.memory",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "fan.speed",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let input_str: String = String::from(input.get(0).unwrap());

                                        // Convert to float
                                        match input_str.parse::<f64>() {
                                            Ok(parsed_value) => {
                                                // Round down to nearest integer
                                                let rounded_value: f64 = parsed_value.floor();

                                                // Convert to string
                                                let mut output: String = rounded_value.to_string();

                                                // Apply formatting
                                                output.push(' ');
                                                output.push('W');

                                                // Return result
                                                Some(output)
                                            }
                                            Err(_) => {
                                                //this should catch "" etc
                                                println!("Not a valid number");

                                                None
                                            }
                                        }
                                    },
                                ),
                                "power.draw",
                            ),
                        ]
                    },
                    2,
                )
            }
            3 => {
                // Nvidia Optimus
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        Some(String::from(input.get(0).unwrap()))
                                    },
                                ),
                                "gpu_name",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Grab current temperature unit from parameters
                                        let current_unit: TemperatureUnit;
                                        if let Some(valid_params) = params {
                                            if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("C"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            } else if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("F"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::FAHRENHEIT;
                                            } else {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            }
                                        } else {
                                            current_unit = TemperatureUnit::CELCIUS;
                                        }

                                        // Apply formatting
                                        match current_unit {
                                            TemperatureUnit::CELCIUS => {
                                                // Apply temperature unit
                                                output.push(char::from_u32(0x00B0).unwrap());
                                                output.push('C');
                                            }
                                            TemperatureUnit::FAHRENHEIT => {
                                                match output.parse::<f64>() {
                                                    Ok(temp) => {
                                                        // Convert to fahrenheit
                                                        let fahrenheit_temp: f64 =
                                                            temp * 9.0 / 5.0 + 32.0;

                                                        // Round down to nearest integer
                                                        let rounded_value: f64 =
                                                            fahrenheit_temp.floor();

                                                        // Convert to string
                                                        let mut f_output: String =
                                                            rounded_value.to_string();

                                                        // Apply temperature unit
                                                        f_output
                                                            .push(char::from_u32(0x00B0).unwrap());
                                                        f_output.push('F');

                                                        // Return result
                                                        Some(f_output)
                                                    }
                                                    Err(_) => {
                                                        //this should catch "" etc
                                                        println!("Not a valid number");

                                                        None
                                                    }
                                                };
                                            }
                                        }

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "temperature.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.used",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.total",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.memory",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "fan.speed",
                            ),
                            Property::new(
                                &Processor::new(
                                    "optirun",
                                    "nvidia-smi --query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let input_str: String = String::from(input.get(0).unwrap());

                                        // Convert to float
                                        match input_str.parse::<f64>() {
                                            Ok(parsed_value) => {
                                                // Round down to nearest integer
                                                let rounded_value: f64 = parsed_value.floor();

                                                // Convert to string
                                                let mut output: String = rounded_value.to_string();

                                                // Apply formatting
                                                output.push(' ');
                                                output.push('W');

                                                // Return result
                                                Some(output)
                                            }
                                            Err(_) => {
                                                //this should catch "" etc
                                                println!("Not a valid number");

                                                None
                                            }
                                        }
                                    },
                                ),
                                "power.draw",
                            ),
                        ]
                    },
                    3,
                )
            }
            _ => {
                // Assume Default (Nvidia Settings and Nvidia SMI)
                // Create new provider
                Provider::new(
                    || {
                        vec![
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        Some(String::from(input.get(0).unwrap()))
                                    },
                                ),
                                "gpu_name",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "utilization.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Grab current temperature unit from parameters
                                        let current_unit: TemperatureUnit;
                                        if let Some(valid_params) = params {
                                            if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("C"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            } else if valid_params.iter().any(|i| {
                                                i == &(
                                                    String::from("tempformat"),
                                                    String::from("F"),
                                                )
                                            }) {
                                                current_unit = TemperatureUnit::FAHRENHEIT;
                                            } else {
                                                current_unit = TemperatureUnit::CELCIUS;
                                            }
                                        } else {
                                            current_unit = TemperatureUnit::CELCIUS;
                                        }

                                        // Apply formatting
                                        match current_unit {
                                            TemperatureUnit::CELCIUS => {
                                                // Apply temperature unit
                                                output.push(char::from_u32(0x00B0).unwrap());
                                                output.push('C');
                                            }
                                            TemperatureUnit::FAHRENHEIT => {
                                                match output.parse::<f64>() {
                                                    Ok(temp) => {
                                                        // Convert to fahrenheit
                                                        let fahrenheit_temp: f64 =
                                                            temp * 9.0 / 5.0 + 32.0;

                                                        // Round down to nearest integer
                                                        let rounded_value: f64 =
                                                            fahrenheit_temp.floor();

                                                        // Convert to string
                                                        let mut f_output: String =
                                                            rounded_value.to_string();

                                                        // Apply temperature unit
                                                        f_output
                                                            .push(char::from_u32(0x00B0).unwrap());
                                                        f_output.push('F');

                                                        // Return result
                                                        Some(f_output)
                                                    }
                                                    Err(_) => {
                                                        //this should catch "" etc
                                                        println!("Not a valid number");

                                                        None
                                                    }
                                                };
                                            }
                                        }

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "temperature.gpu",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.used",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input and add formatting
                                        Some(String::from(input.get(0).unwrap()) + " MiB")
                                    },
                                ),
                                "memory.total",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let mut output: String =
                                            String::from(input.get(0).unwrap());

                                        // Apply formatting
                                        output.push(' ');
                                        output.push('%');

                                        // Return result
                                        Some(output)
                                    },
                                ),
                                "fan.speed",
                            ),
                            Property::new(
                                &Processor::new(
                                    "nvidia-smi",
                                    "--query-gpu=",
                                    None,
                                    " --format=csv,noheader -i ",
                                ),
                                &Formatter::new(
                                    |input: Vec<String>, _params: Option<Vec<(String, String)>>| {
                                        // Grab input
                                        let input_str: String = String::from(input.get(0).unwrap());

                                        // Convert to float
                                        match input_str.parse::<f64>() {
                                            Ok(parsed_value) => {
                                                // Round down to nearest integer
                                                let rounded_value: f64 = parsed_value.floor();

                                                // Convert to string
                                                let mut output: String = rounded_value.to_string();

                                                // Apply formatting
                                                output.push(' ');
                                                output.push('W');

                                                // Return result
                                                Some(output)
                                            }
                                            Err(_) => {
                                                //this should catch "" etc
                                                println!("Not a valid number");

                                                None
                                            }
                                        }
                                    },
                                ),
                                "power.draw",
                            ),
                        ]
                    },
                    0,
                )
            }
        }
    }

    /**
     * Name:
     * refresh_cards
     *
     * Description:
     * Re-create the GpuPages and stack contents when called
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn refresh_cards(&self) {
        // Clear current ActionRow objects from GtkListBox
        println!("Clearing gpu_stack!"); //TEST

        // Get reference to stack
        let stack: Stack = self.gpu_stack.get();

        // For each stored GPU name
        for gpu_name in self.gpu_names.take()
        {
            // Get appropriate gpu_page
            let child: Option<gtk::Widget> = stack.child_by_name(&gpu_name);

            // Remove page
            if let Some(valid_child) = child {
                stack.remove(&valid_child);
            }
            else
            {
                //TODO: add better error handling here..
                println!("Missing GPU page for `{}`", gpu_name);
            }
        }

        println!("Populating gpu_stack!"); //TEST
        // Grab copy of current provider
        let provider_container: Option<Provider> = self.provider.take();
        self.provider.set(provider_container.clone());

        match provider_container {
            // If provider does exist
            Some(existing_provider) => {
                println!("Provider exists!"); //TEST
                // Check provider type in settings
                let provider_type: i32 = self.get_setting::<i32>("provider");

                // If type has been changed, update provider
                if existing_provider.property::<i32>("provider_type") != provider_type {
                    // Create new provider
                    let new_provider_container: Option<Provider> =
                        Some(self.create_provider(provider_type));
                    self.provider.set(new_provider_container.clone());

                    // Using the new provider
                    match new_provider_container {
                        Some(new_provider) => {
                            // Update GPU list
                            match new_provider.get_gpu_uuids() {
                                Ok(gpu_uuids) => {
                                    // Construct a row for each GPU
                                    for uuid in gpu_uuids {
                                        // Grab current provider
                                        let provider_container: Option<Provider> =
                                            self.provider.take();
                                        self.provider.set(provider_container.clone());

                                        // Get GPU data
                                        match provider_container {
                                            Some(prov) => match prov.get_gpu_data(&uuid, "name") {
                                                Ok(gpu_name) => {
                                                    // Create new GpuPage object and Add to list of pages
                                                    self.create_gpu_page(&uuid, &gpu_name, prov);
                                                },
                                                Err(err) => {
                                                    println!("..Attempt to read GPU name failed, returning: {}", err);

                                                    // Create new GpuPage object and Add to list of pages
                                                    self.create_gpu_page(&uuid, &uuid, prov);
                                                }
                                            }
                                            None => panic!("Something weird has happened! Cannot grab known existing provider.."),
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!(
                                        "..Attempt to update GPU list failed, returning: {}",
                                        err
                                    )
                                }
                            }
                        }
                        None => todo!(),
                    }
                } else {
                    // Update GPU list
                    match existing_provider.get_gpu_uuids() {
                        Ok(gpu_uuids) => {
                            // Construct a row for each GPU
                            for uuid in gpu_uuids {
                                // Grab current provider
                                let provider_container: Option<Provider> = self.provider.take();
                                self.provider.set(provider_container.clone());
                                // Get GPU data
                                match provider_container {
                                    Some(prov) => match prov.get_gpu_data(&uuid, "name") {
                                        Ok(gpu_name) => {
                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &gpu_name, prov);
                                        },
                                        Err(err) => {
                                            println!("..Attempt to read GPU name failed, returning: {}", err);

                                            // Create new GpuPage object and Add to list of pages
                                            self.create_gpu_page(&uuid, &uuid, prov);
                                        }
                                    }
                                    None => panic!("Something weird has happened! Cannot grab known existing provider.."),
                                }
                            }
                        }
                        Err(err) => {
                            println!("..Attempt to update GPU list failed, returning: {}", err)
                        }
                    }
                }
            }
            // If provider does not exist
            None => {
                println!("No provider exists, checking settings!"); //TEST
                // Check provider type
                let provider_type: i32 = self.get_setting::<i32>("provider");

                // Create new provider
                let new_provider_container: Option<Provider> =
                    Some(self.create_provider(provider_type));
                self.provider.set(new_provider_container.clone());

                // Using the new provider
                match new_provider_container {
                    Some(new_provider) => {
                        // Update GPU list
                        match new_provider.get_gpu_uuids() {
                            Ok(gpu_uuids) => {
                                // Construct a row for each GPU
                                for uuid in gpu_uuids {
                                    // println!("UUID: `{}`", uuid); //TEST
                                    // Grab current provider
                                    let provider_container: Option<Provider> = self.provider.take();
                                    self.provider.set(provider_container.clone());

                                    // Get GPU data
                                    match provider_container {
                                        Some(prov) => match prov.get_gpu_data(&uuid, "name") {
                                            Ok(gpu_name) => {
                                                // Create new GpuPage object and Add to list of pages
                                                self.create_gpu_page(&uuid, &gpu_name, prov); //SEGFAULT
                                            },
                                            Err(err) => {
                                                println!("..Attempt to read GPU name failed, returning: {}", err);

                                                // Create new GpuPage object and Add to list of pages
                                                self.create_gpu_page(&uuid, &uuid, prov);
                                            }
                                        }
                                        None => panic!("Something weird has happened! Cannot grab known existing provider.."),
                                    }
                                }
                            }
                            Err(err) => {
                                println!("..Attempt to update GPU list failed, returning: {}", err)
                            }
                        }
                    }
                    None => todo!(),
                }
            }
        }
    }
}

/**
 * Name:
 * MainWindow
 *
 * Description:
 * Trait defining template callbacks shared by all MainWindow objects
 *
 * Made:
 * 13/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
#[gtk::template_callbacks]
impl MainWindow {
    /**
     * Name:
     * refresh_cards
     *
     * Description:
     * Template callback for GPU list refresh button
     *
     * Made:
     * 28/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn refresh_cards_clicked(&self, _button: &Button) {
        println!("GPU Scan Button Pressed!"); //TEST
        self.refresh_cards();
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
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for MainWindow {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
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
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let obj: BorrowedObject<super::MainWindow> = self.obj();
        obj.setup_settings();
        obj.setup_widgets();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 06/10/2022
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
                glib::ParamSpecObject::builder::<Provider>("provider").build()
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
     * 06/10/2022
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
            "provider" => match value.get() {
                Ok(input_provider_property) => {
                    self.provider.replace(input_provider_property);
                }
                Err(_) => panic!("The value needs to be of type `Provider`."),
            },
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 06/10/2022
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
            "provider" => {
                //TODO: this seems ridiculous..
                let value: Option<Provider> = self.provider.take();

                self.provider.set(value.clone());

                value.to_value()
            }
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }
    }

    /**
     * Name:
     * signals
     *
     * Description:
     * Defines the list of signals
     *
     * Made:
     * 10/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * beware that you need to use kebab-case (<https://en.wikipedia.org/wiki/Letter_case#Kebab_case>)
     *
     * <https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_signals.html>
     *
     * SignalType::from(i32::static_type())
     */
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("update-all-views")
                    .return_type::<i32>()
                    .build()
            ]
        })
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
 * 09/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for MainWindow {}

/**
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
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
impl WindowImpl for MainWindow {
    /**
     * Name:
     * close_request
     *
     * Description:
     * Run when window closed
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
    fn close_request(&self) -> Propagation {
        /*
        //NOTE: this was meant for saving to json, probably unecessary now that i'm not using it..


        // Create vector for final saving
        let mut save_data: Vec<Vec<(String, String)>> = vec![];
        //let mut save_data: gio::ListStore;

        // For each GPU page
        for current_page in self.gpu_pages.take() {
            // Grab data stored in page
            let uuid: String = current_page.property::<String>("uuid");
            //println!("UUID FETCH: {}", current_page.property::<String>("uuid"));
            let name: String = current_page.property::<String>("name");
            //println!("NAME FETCH: {}", current_page.property::<String>("name"));

            // Create vector for current GPU
            let mut current_stats: Vec<(String, String)> = vec![];

            // For stat object in page
            //for label in current_page.property::<>("stats") {
            //
            //}
            // PLACEHOLDER
            current_stats.push(("uuid".to_string(), uuid));
            current_stats.push(("name".to_string(), name));
            // PLACEHOLDER

            // Add current vector to final vector
            save_data.push(current_stats);
            //let test_s = gtk::Label::new(Some("FUCK"));
            //save_data.append(&test_s);
        }

        // TEST OUTPUT
        for gpu in save_data {
            for stat in gpu {
                println!("STAT NAME: `{}` STAT VALUE: `{}`", stat.0, stat.1);
            }
        }
        */

        //NOTE: This doesn't work for some reason, but seems to be to do with
        //      nvidia-settings or the Cancellable implementation in the rust bindings..
        // Cancel any under-way sub processes
        // if let Some(control) = Cancellable::current() {
        //     println!("Closing open windows..");
        //     control.cancel();
        // }

        // Store sub-window states in settings
        self.update_setting("app-settings-open", false);
        self.update_setting("nvidia-settings-open", false);
        self.update_setting("modification-open", false);

        // Pass close request on to the parent
        self.parent_close_request()
    }
}

/**
 * Trait Name:
 * AdwWindowImpl
 *
 * Description:
 * Trait shared by all AdwWindow's
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
impl AdwWindowImpl for MainWindow {}

/**
 * Trait Name:
 * ApplicationWindowImpl
 *
 * Description:
 * Trait shared by all ApplicationWindow's
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
impl ApplicationWindowImpl for MainWindow {}

/**
 * Trait Name:
 * AdwApplicationWindowImpl
 *
 * Description:
 * Trait shared by all AdwApplicationWindow's
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
impl AdwApplicationWindowImpl for MainWindow {}
