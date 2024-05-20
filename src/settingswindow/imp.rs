// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (SettingsWindow)
 *
 * Made:
 * 10/10/2022
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
    //Cell,
    OnceCell, RefCell, RefMut
};
use std::rc::Rc;
// gtk-rs
use gtk::{
    subclass::prelude::*,
    CheckButton, CompositeTemplate, SpinButton, TemplateChild
};
use adwaita::{
    gio, glib,
    prelude::*, subclass::prelude::*,
    ComboRow
};
use gio::Settings;
use glib::{
    signal::Propagation,
    ParamSpec, subclass::InitializingObject,
    variant::Variant,
    value::Value
};

// Modules
//use crate::utils::data_path;
use crate::mainwindow::MainWindow;

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct ParentContainer {
    pub window: Option<MainWindow>,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/settings-window.ui")]
pub struct SettingsWindow {
    // Public
    pub settings: OnceCell<Settings>,
    pub parent_window: Rc<RefCell<ParentContainer>>,

    // Template Children
    #[template_child]
    pub refreshrate_input: TemplateChild<SpinButton>,
    #[template_child]
    pub temp_unit_c: TemplateChild<CheckButton>,
    #[template_child]
    pub temp_unit_f: TemplateChild<CheckButton>,
    #[template_child]
    pub provider_input: TemplateChild<ComboRow>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SettingsWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionSettingsWindow";
    type Type = super::SettingsWindow;
    //type ParentType = adwaita::PreferencesWindow;
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
 * SettingsWindow
 *
 * Description:
 * Trait shared by all SettingsWindow objects
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
impl SettingsWindow {
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
     * refreshrate_set
     *
     * Description:
     * Template callback for setting refresh rate of application
     *
     * Made:
     * 23/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn refreshrate_set(
        &self,
        button: &SpinButton
    ) {
        // Get new refresh rate input
        let new_value: i32 = button.value_as_int();

        // Set refresh rate property
        self.update_setting("refreshrate", new_value);
    }

    /**
     * Name:
     * temp_unit_set
     *
     * Description:
     * Template callback for setting temperature unit of application
     *
     * Made:
     * 23/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn temp_unit_set(
        &self,
        button: &CheckButton
    ) {
        // Get list of buttons
        let check_buttons: [&CheckButton; 2] = [&self.temp_unit_c, &self.temp_unit_f];

        // For each button in the group
        for current_button in check_buttons {
            // Check if current button active
            if current_button.is_active() {
                // Get new unit
                match button.label() {
                    Some(unit) => {
                        // Set appropriate setting
                        match String::from(unit).as_str() {
                            "Celcius (C)" => {
                                // Set temperature unit as C
                                self.update_setting("tempformat", 0);
                            }
                            "Fahrenheit (F)" => {
                                // Set temperature unit as F
                                self.update_setting("tempformat", 1);
                            }
                            _ => panic!("..Unexpected temperature unit"),
                        }
                    }
                    None => panic!("..Could not fetch contents of temperature unit button label"),
                }
            }
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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for SettingsWindow {
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
        let obj: glib::BorrowedObject<super::SettingsWindow> = self.obj();
        obj.setup_settings();
        obj.restore_data();
        obj.setup_widgets();
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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for SettingsWindow {}

/**
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
 *
 * Made:
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WindowImpl for SettingsWindow {
    fn close_request(&self) -> Propagation {
        /*
        // Store task data in vector
        let backup_data: Vec<TaskData> = window
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();

        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file");

        */

        // Store state in settings
        self.update_setting("app-settings-open", false);

        // Emit signal to notify changes made to view (and thus reload required)
        let modification_window_container: RefMut<ParentContainer> =
            self.parent_window.borrow_mut();
        let _result = modification_window_container
            .window
            .as_ref()
            .unwrap()
            .emit_by_name::<i32>("update-all-views", &[]);

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
 * 10/10/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl AdwWindowImpl for SettingsWindow {}

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
impl ApplicationWindowImpl for SettingsWindow {}

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
impl AdwApplicationWindowImpl for SettingsWindow {}
