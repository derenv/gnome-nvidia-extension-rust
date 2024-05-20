// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (Provider)
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
// Custom GObjects
mod imp;

// Imports
// std
use std::ffi::OsStr;
// gtk-rs
use gtk::{
    prelude::*, subclass::prelude::*
};
use adwaita::{
    gio, glib
};
use gio::{
    Cancellable, Settings
};
use glib::Object;

// Crates
use crate::{
    processor::Processor, property::Property, subprocess::subprocess::exec_communicate_async,
    APP_ID,
};

// GObject wrapper for Provider
glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

/**
 * Trait Name:
 * Provider
 *
 * Description:
 * Trait shared by all Providers
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
impl Provider {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new Provider object
     *
     * Made:
     * 06/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * NvidiaSettingsandNvidiaSmi = 0,
     * NvidiaSettings = 0,
     * NvidiaSmi = 1,
     * NvidiaOptimus = 2,
     *
     */
    pub fn new(func: fn() -> Vec<Property>, provider_type: i32) -> Self {
        // Create Object        
        let obj: Provider = Object::builder::<Provider>().build();

        // Set type of provider
        obj.set_property("provider-type", provider_type);

        // Set properties
        let res: Vec<Property> = func();
        //println!("PROPERTIES ON CREATION: `{}`", res.len());
        obj.imp().properties.replace(res);

        obj
    }

    /**
     * Name:
     * get_gpu_uuids
     *
     * Description:
     * Get list of all GPU uuid's
     *
     * Made:
     * 24/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
    //NOTE: Leaving this here for future use..
    //p.set_property("base-call", "nvidia-settings");
    //p.set_property("call", "nvidia-settings");
    //p.set_property("tail-call", "t");
     */
    pub fn get_gpu_uuids(&self) -> Result<Vec<String>, String> {
        // Check provider type
        let processor_args: [&str; 2];
        match self.property::<i32>("provider-type") {
            // Nvidia Settings/SMI OR Nvidia Settings
            0 | 1 => {
                processor_args = ["nvidia-settings", "-q GpuUUID -t"];
            }
            // Nvidia SMI
            2 => {
                processor_args = ["nvidia-smi", "-L"];
            }
            // Nvidia Optimus
            3 => {
                processor_args = ["optirun", "nvidia-smi -L"];
            }
            _ => {
                // Return error..
                return Err(String::from("Invalid provider, check preferences.."));
            }
        }

        // Create a processor object with appropriate args
        let processor: Processor = Processor::new(processor_args[0], processor_args[1], None, "");

        // Validate output
        match processor.process(None, None) {
            Ok(output) => match output {
                Some(valid_output) => {
                    // If a valid output given, finally return to main window
                    match self.property::<i32>("provider-type") {
                        // Nvidia Settings/SMI OR Nvidia Settings
                        0 | 1 => Ok(valid_output),
                        // Nvidia SMI or Nvidia Optimus
                        2 | 3 => {
                            let mut cleaned_output: Vec<String> = vec![];
                            for line in valid_output {
                                // Grab mostly-correct contents
                                let wanted: Vec<&str> = line.split("(UUID: ").collect();

                                // Remove any unwanted chars
                                let cleaned_line = wanted[1].replace(")", "");

                                // Add to output
                                cleaned_output.push(cleaned_line);
                            }

                            Ok(cleaned_output)
                        }
                        _ => {
                            // Return error..
                            return Err(String::from("Invalid provider, check preferences.."));
                        }
                    }
                }
                None => {
                    // Return error..
                    Err(String::from("Process encountered an unknown error.."))
                }
            },
            Err(err) => {
                // Return error..
                return Err(String::from(err.message()));
            }
        }
    }

    /**
    * Name:
    * get_gpu_data
    *
    * Description:
    * Grab gpu data from provider program given a GPU uuid and property name
    *
    * Made:
    * 30/10/2022
    *
    * Made by:
    * Deren Vural
    *
    * Notes:
    * Designed to be expanded on later when more data needed..
    *
       let statistics_data: Vec<&str> = vec![
           "util",
           "temp",
           "memory_usage",
           "memory_total",
           "fan_speed",
           "power_usage",
       ];
    */
    pub fn get_gpu_data(&self, uuid: &str, property: &str) -> Result<String, String> {
        // println!("UUID: `{}`", uuid); //TEST
        // println!("ASKED TO FETCH: `{}`", property); //TEST
        // println!("TYPE: `{}`", self.property::<i32>("provider_type")); //TEST

        // Translate to appropriate name
        let final_property: String;
        match self.property::<i32>("provider_type") {
            // Nvidia Settings/SMI
            0 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "mem_ctrl_util" => final_property = String::from("utilization.memory"),
                //"encoder_util" => final_property = String::from("clocks.current.video"),
                //"decoder_util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property..")),
            },
            // Nvidia Settings
            1 => match property {
                "util" => final_property = String::from("GPUUtilization.gpu"),
                "mem_ctrl_util" => final_property = String::from("GPUUtilization.mem"),

                "temp" => final_property = String::from("GPUCoreTemp"),
                "memory_usage" => final_property = String::from("UsedDedicatedGPUMemory"),
                "memory_total" => final_property = String::from("TotalDedicatedGPUMemory"),
                // This isn't queried by GPU UUID, just returns *all*
                //"fan_speed" => final_property = String::from("GPUCurrentFanSpeedRPM"),
                _ => return Err(String::from("Unknown property..")),
            },
            // Nvidia SMI
            2 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "mem_ctrl_util" => final_property = String::from("utilization.memory"),
                //"encoder_util" => final_property = String::from("clocks.current.video"),
                //"decoder_util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property..")),
            },
            // Nvidia Optimus
            3 => match property {
                "name" => final_property = String::from("gpu_name"),
                "util" => final_property = String::from("utilization.gpu"),
                "mem_ctrl_util" => final_property = String::from("utilization.memory"),
                //"encoder_util" => final_property = String::from("clocks.current.video"),
                //"decoder_util" => final_property = String::from("utilization.gpu"),
                "temp" => final_property = String::from("temperature.gpu"),
                "memory_usage" => final_property = String::from("memory.used"),
                "memory_total" => final_property = String::from("memory.total"),
                "fan_speed" => final_property = String::from("fan.speed"),
                "power_usage" => final_property = String::from("power.draw"),
                _ => return Err(String::from("Unknown property..")),
            },
            // ???
            _ => return Err(String::from("Unknown provider type..")),
        }

        // Grab relevant property
        for prop in self.imp().properties.borrow().iter() {
            // println!("current property: `{}`", prop.property::<String>("id")); //TEST
            // println!("looking for property: `{}`", final_property); //TEST

            if prop.property::<String>("id") == final_property {
                // Run and return output
                match prop.to_owned().parse(uuid) {
                    Some(stat) => return Ok(stat),
                    None => {
                        return Err(String::from(
                            "Problem occured when trying to run property..",
                        ))
                    }
                }
            }
        }

        Err(String::from("Cannot find property.."))
    }

    /**
     * Name:
     * open_settings
     *
     * Description:
     * Open settings if provider is capable
     *
     * Made:
     * 24/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn open_settings(&self) -> Result<(), String> {
        // Check provider type
        match self.property::<i32>("provider-type") {
            // Open Nvidia Settings
            0 | 1 => {
                //NOTE: This doesn't work for some reason, but seems to be to do with
                //      nvidia-settings or the Cancellable implementation in the rust bindings..
                // Add new cancellable object to stack
                //let control: Cancellable = Cancellable::new();
                //control.push_current();

                // Start cancellable async process
                match exec_communicate_async(
                    &[OsStr::new("nvidia-settings")],
                    None::<&Cancellable>, /*Some(&control)*/
                    |result| {
                        // Callback
                        match result {
                            Err(err) => {
                                println!("Process failed with error: `{}`", err.to_string());
                            }
                            Ok(buffers) => match buffers {
                                // Success
                                (Some(_), None) | (None, None) => {
                                    println!("Nvidia Settings now closed..");

                                    // Get settings for APP_ID
                                    let settings = Settings::new(APP_ID);

                                    // Update settings
                                    let name: &str = "nvidia-settings-open";
                                    match settings.set_boolean(name, false) {
                                        Ok(_) => println!("..Setting `{}` updated!", name),
                                        Err(err) => {
                                            panic!("..Cannot update `{}` setting: `{}`", name, err)
                                        }
                                    }
                                }
                                // Error
                                (None, Some(stderr_buffer)) | (Some(_), Some(stderr_buffer)) => {
                                    println!(
                                        "Process failed with error: `{}`",
                                        String::from_utf8_lossy(&stderr_buffer)
                                    );
                                }
                            },
                        }
                    },
                ) {
                    Ok(()) => return Ok(()),
                    Err(err) => return Err(err.to_string()),
                };
            }
            // Error Message
            2 | 3 => Err(String::from(
                "Nvidia Settings cabable provider is not enabled in preferences..",
            )),
            _ => Err(String::from("Invalid provider, check preferences..")),
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
 * 06/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for Provider {
    fn default() -> Self {
        Self::new(Vec::new, 0)
    }
}
