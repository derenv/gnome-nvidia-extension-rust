// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/*
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (GpuPage)
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */

// Imports
// std
use std::sync::{
    OnceLock, Arc, Mutex, MutexGuard
};
use std::cell::{
    Cell, OnceCell, RefCell
};
use std::rc::Rc;
// gtk-rs
use gtk::{
    subclass::prelude::*,
    Align, CompositeTemplate, Grid, Label, LayoutChild, Orientation, TemplateChild
};
use adwaita::{
    gio, glib,
    prelude::*,
    ViewStack, ViewSwitcherBar
};
use gio::Settings;
use glib::{
    subclass::Signal,
    translate::FromGlib,
    subclass::InitializingObject, ParamSpec, SourceId,
    variant::FromVariant, variant::Variant,
    value::ToValue, value::Value,
    ControlFlow::Continue
};

// Modules
use crate::{modificationwindow::ModificationWindow, provider::Provider};

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct ModificationWindowContainer {
    pub window: Option<ModificationWindow>,
    pub open: bool,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/gpu-page.ui")]
pub struct GpuPage {
    pub settings: OnceCell<Settings>,
    uuid: OnceCell<String>,
    name: OnceCell<String>,
    provider: OnceCell<Option<Provider>>,
    refreshid: Cell<u32>,

    pub modification_window: Rc<RefCell<ModificationWindowContainer>>,

    #[template_child]
    pub view_switcher: TemplateChild<ViewSwitcherBar>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GpuPage {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionGpuPage";
    type Type = super::GpuPage;
    type ParentType = gtk::Grid;

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
 * GpuPage
 *
 * Description:
 * Trait shared by all GpuPage objects
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
#[gtk::template_callbacks]
impl GpuPage {
    //
}

impl GpuPage {
    /**
     * Name:
     * get_setting
     *
     * Description:
     * Generic function for getting setting value
     *
     * Made:
     * 04/12/2022
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
     * 04/12/2022
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
     * replace_stack
     *
     * Description:
     * Replace current view_stack using passed value
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn replace_stack(
        &self,
        stack: Option<&ViewStack>
    ) {
        self.view_switcher.set_stack(stack);
    }

    /**
     * Name:
     * create_properties
     *
     * Description:
     * Add labels (of properties) to a passed grid, returns the grid afterwards
     *
     * Made:
     * 03/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn create_properties(
        &self,
        grid: Grid,
        properties: Vec<String>,
        mut labels: Vec<Label>,
    ) -> (Grid, Vec<Label>) {
        // Load properties from struct
        let properties_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(properties));

        // Grab grid manager
        let grid_manager = grid.layout_manager().unwrap();

        // For each Property
        for property in Arc::clone(&properties_store).lock().unwrap().iter() {
            // println!("BUILDING PROPERTY LABEL: `{}`", property); //TEST

            //==BUILD==
            // Build grid for 2 labels and attach to this page
            let new_grid_name: String = String::from("property_item_grid");
            let new_grid: Grid = Grid::builder()
                .name(&new_grid_name)
                .orientation(Orientation::Horizontal)
                //.margin_start(12)
                //.margin_end(12)
                //.margin_top(12)
                //.margin_bottom(12)
                //.halign(Align::Start)
                //.valign(Align::Center)
                //.hexpand(true)
                //.vexpand(true)
                .build();
            grid.attach(&new_grid, 0, labels.len() as i32, 100, 12);

            // Set layout properties of grid
            let child_manager: LayoutChild = grid_manager.layout_child(&new_grid);
            child_manager.set_property("row-span", 1);
            child_manager.set_property("column-span", 1);
            //child_manager.set_property("outline-style", "solid");
            //child_manager.set_property("outline-width", 1);
            //child_manager.set_property("border-radius", 3);

            // Fetch layout manager for this (grid) child
            match new_grid.layout_manager() {
                Some(internal_grid_manager) => {
                    // Decide on title label size
                    let space: i32;
                    let pretty_label: &str;
                    //TODO: Update to use global list
                    match property.to_owned().as_str() {
                        "util" => {
                            pretty_label = "GPU Utilization";
                            space = 5
                        }
                        "mem_ctrl_util" => {
                            pretty_label = "Memory Controller Utilization";
                            space = 5
                        }
                        "encoder_util" => {
                            pretty_label = "Encoder Utilization";
                            space = 5
                        }
                        "decoder_util" => {
                            pretty_label = "Decoder Utilization";
                            space = 5
                        }
                        "fan_speed" => {
                            pretty_label = "Fan Speed";
                            space = 5
                        }
                        "temp" => {
                            pretty_label = "Temperature";
                            space = 5
                        }
                        "memory_usage" => {
                            pretty_label = "Memory Usage";
                            space = 8
                        }
                        "memory_total" => {
                            pretty_label = "Memory Total";
                            space = 8
                        }
                        "power_usage" => {
                            pretty_label = "Power Usage";
                            space = 8
                        }
                        "none" => {
                            pretty_label = "None";
                            space = 5
                        }
                        _ => {
                            pretty_label = property;
                            space = 5
                        }
                    }

                    // Build title label & add to grid
                    let new_title: String = String::from(property.to_owned()) + "_label";
                    let new_title_label: Label = Label::builder()
                        .label(pretty_label)
                        .name(&new_title)
                        .hexpand(true)
                        .hexpand_set(true)
                        //.halign(Align::Center)
                        .halign(Align::Start)
                        //.valign(Align::Center)
                        .margin_top(12)
                        .margin_bottom(12)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_title_label, 0, 0, 1, 1);

                    // Set layout properties of (title label) child
                    let title_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_title_label);
                    title_manager.set_property("row-span", 1);
                    title_manager.set_property("column-span", 1);

                    // Decide on content label size
                    let space: i32;
                    match property.to_owned().as_str() {
                        "util" | "fan_speed" | "temp" | "none" => space = 5,
                        "memory_usage" | "memory_total" => space = 8,
                        _ => space = 5,
                    }

                    // Build content label & add to grid
                    let new_content: String = String::from(property.to_owned());
                    let new_content_label: Label = Label::builder()
                        .label("")
                        .name(&new_content)
                        .halign(Align::End)
                        //.valign(Align::Center)
                        .width_chars(space)
                        .build();
                    new_grid.attach(&new_content_label, 1, 0, 1, 1);

                    // Set layout properties of (content label) child
                    let content_manager: LayoutChild =
                        internal_grid_manager.layout_child(&new_content_label);
                    content_manager.set_property("row-span", 1);
                    title_manager.set_property("column-span", 1);

                    // Add to list of content labels, for updating in closure (see below)
                    labels.push(new_content_label);
                }
                None => panic!("Cannot find layout manager.."),
            }
        }

        // for lab in &labels {
        //     println!("LABEL: `{}`", lab.widget_name());
        // }

        // Return modified grid object
        (grid, labels)
    }

    /**
     * Name:
     * create_updater
     *
     * Description:
     * Creates a recurring closure to fill the passed list of labels
     * Stores the ID of the recurring closure to allow removal
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * <https://stackoverflow.com/questions/70986004/gtk-rs-set-label-within-glibtimeout-add>
     * <https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html>
     *
     * !!UNSAFE CODE HERE!!
     */
    pub fn create_updater(&self, labels: Vec<Label>, properties: Vec<String>) {
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
        // Check for any running recurring closures
        // Get stored ID
        let id_raw: u32 = self.refreshid.clone().get();
        unsafe {
            // If the stored ID is valid
            if id_raw != 0 {
                // Re-translate to SourceId object
                let id: SourceId = FromGlib::from_glib(id_raw);

                // Remove recurring closure
                id.remove();

                // println!("REMOVED RECURRING CLOSURE.."); //TEST
            } else {
                // println!("NO PRE-EXISTING RECURRING CLOSURE.."); //TEST
            }
        }
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!

        // Get stored UUID
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("`uuid` wasn't set properly..")
            .to_string();

        // Load refresh time (s) from settings
        let refresh_rate: u32 = self.get_setting::<i32>("refreshrate") as u32;

        // Create thread safe container for properties
        let properties_store: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(properties));

        // Create thread safe container for labels
        let label_store: Arc<Mutex<Vec<Label>>> = Arc::new(Mutex::new(labels));

        // Create thread safe container for uuid, needed for processor
        let uuid_store: Arc<Mutex<String>> = Arc::new(Mutex::new(uuid));

        // Create thread safe container for provider
        let provider: Option<Provider> = self
            .provider
            .clone()
            .get()
            .expect("`provider` wasn't set properly..")
            .to_owned();
        let provider_store: Arc<Mutex<Option<Provider>>> = Arc::new(Mutex::new(provider));

        // Async fill the labels
        let id: SourceId = glib::timeout_add_seconds_local(refresh_rate, move || {
            // Grab locked data
            // list of Properties
            let properties_lock: Arc<Mutex<Vec<String>>> = Arc::clone(&properties_store);
            let properties: MutexGuard<Vec<String>> = properties_lock.lock().unwrap();
            // uuid
            let uuid_lock: Arc<Mutex<String>> = Arc::clone(&uuid_store);
            let uuid: String = uuid_lock.lock().unwrap().as_str().to_owned();
            // current provider for scanning gpu data
            let provider_lock: Arc<Mutex<Option<Provider>>> = Arc::clone(&provider_store);
            let mut provider_container: MutexGuard<Option<Provider>> =
                provider_lock.lock().unwrap();

            let labels_lock: Arc<Mutex<Vec<Label>>> = Arc::clone(&label_store);
            let labels_container: MutexGuard<Vec<Label>> = labels_lock.lock().unwrap();
            // println!("labels: `{}`", labels_container.len());

            // For each Property
            match &mut *provider_container {
                Some(current_provider) => {
                    for property in properties.iter() {
                        // println!("property: `{}`", property); //TEST

                        if property == "none" {
                            // Check if correct label
                            for label in labels_container.iter() {
                                if *property.to_owned() == label.widget_name() {
                                    label.set_label("N/A");
                                    // no break here - could be duplicates
                                }
                            }
                        } else {
                            // Grab current Property from provider
                            match current_provider.get_gpu_data(&uuid, property) {
                                Ok(property_value) => {
                                    // For each output label of the page
                                    for label in labels_container.iter() {
                                        // println!("COMPARING AGAINST 1: `{}`", property);
                                        // println!("COMPARING AGAINST 2: `{}`", label.widget_name());

                                        // Check if correct label
                                        if *property.to_owned() == label.widget_name() {
                                            label.set_label(&property_value);
                                            // no break here - could be duplicates
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!("panicked when fetching gpu data: `{}`", err);
                                    return Continue;
                                }
                            }
                        }
                    }
                }
                None => {
                    println!("panicked when fetching current provider..");
                    return Continue;
                }
            }

            Continue
        });

        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
        // Save ID of recurring closure
        self.refreshid.set(id.as_raw());
        // unsafe {
        //     self.refreshid.set(id.as_raw());
        // }
        // !!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!!!UNSAFE CODE HERE!!
    }

    /**
     * Name:
     * check_properties_for_view
     *
     * Description:
     * Using passed view title, returns list of saved properties
     *
     * Made:
     * 03/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn check_properties_for_view(&self, view_name: &str) -> Vec<String> {
        // Load list of Properties for current Page
        let loaded_properties_data: Vec<String> =
            self.get_setting::<Vec<String>>("viewcomponentconfigs");
        // println!("items saved #: `{}`", loaded_properties_data.len()); //TEST
        // println!("items saved: `{:?}`", loaded_properties_data); //TEST

        // Get stored UUID
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("`uuid` wasn't set properly..")
            .to_string();

        // If present in saved settings, use! otherwise follow below defaults
        if let 0 = loaded_properties_data.len() {
            // println!("no properties.."); //TEST

            vec![]
        } else {
            // Create temporary structure for sorting loaded data
            let mut loaded_properties: Vec<String> =
                vec![String::from(""); loaded_properties_data.len()];

            for index in 0..loaded_properties_data.len() {
                // println!("item: `{}`", loaded_properties_data[index]); //TEST

                // Split current item into the 4 parts
                let parts: Vec<&str> = loaded_properties_data[index]
                    .split(':')
                    .collect::<Vec<&str>>();

                // Catch any malformed items
                if parts.len() != 4 {
                    panic!("Malformed gschema data..");
                }

                // If from valid page
                if parts[0] == uuid {
                    // println!("VALID UUID"); //TEST

                    // If from valid view
                    if parts[1] == view_name {
                        // println!("VALID VIEW #"); //TEST

                        // If a valid position
                        match parts[2].parse::<usize>() {
                            Ok(position) => {
                                // println!("POSITION INDEX: `{}`", position); //TEST
                                if position <= loaded_properties_data.len() {
                                    // println!("VALID POSITION INDEX"); //TEST
                                    // println!("VALID PROPERTY: `{}`", parts[3]); //TEST

                                    // Add to final list
                                    loaded_properties[position] = parts[3].to_owned();
                                }
                            }
                            Err(_) => panic!("Invalid Property position in gschema data.."),
                        }
                    }
                }
            }

            // Remove any empty properties
            loaded_properties.retain(|x| *x != "");

            // Return final list
            loaded_properties.to_owned()
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
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for GpuPage {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
     *
     * Made:
     * 02/11/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    fn constructed(&self) {
        // println!("CONSTRUCTED");//TEST
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        self.refreshid.set(0);
        let _obj: glib::BorrowedObject<super::GpuPage> = self.obj();
        //obj.setup_settings();
        //obj.load_properties();//TODO
        //obj.setup_widgets();
        //obj.setup_callbacks();
        //obj.setup_actions();
    }

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 02/11/2022
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
                glib::ParamSpecString::builder("uuid").build(),
                glib::ParamSpecString::builder("name").build(),
                glib::ParamSpecObject::builder::<Provider>("provider").build(),
                glib::ParamSpecUInt::builder("refreshid").build()
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
     * 02/11/2022
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

        // println!("setting: {:?}", pspec.name());//TEST
        // let x: String = self.uuid.take();
        // self.uuid.set(x.clone());
        // println!("U: `{}`", x);
        // let x: String = self.name.take();
        // self.name.set(x.clone());
        // println!("N: `{}`", x);

        match pspec.name() {
            "uuid" => match value.get() {
                Ok(input_uuid) => self
                    .uuid
                    .set(input_uuid)
                    .expect("`uuid` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "name" => match value.get() {
                Ok(input_name) => self
                    .name
                    .set(input_name)
                    .expect("`name` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "provider" => match value.get() {
                Ok(input_provider) => self
                    .provider
                    .set(input_provider)
                    .expect("`provider` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `Provider`."),
            },
            "refreshid" => match value.get() {
                Ok(input_refreshid_property) => {
                    self.refreshid.replace(input_refreshid_property);
                }
                Err(_) => panic!("The value needs to be of type `u32`."),
            },
            _ => panic!("Property `{}` does not exist..", pspec.name()),
        }

        // let x: String = self.uuid.take();
        // self.uuid.set(x.clone());
        // println!("U: `{}`", x);
        // let x: String = self.name.take();
        // self.name.set(x.clone());
        // println!("N: `{}`", x);
    }

    /**
     * Name:
     * property
     *
     * Description:
     * Accessor for custom GObject properties
     *
     * Made:
     * 02/11/2022
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
            "uuid" => match self.uuid.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `uuid` property.."),
            },
            "name" => match self.name.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `name` property.."),
            },
            "provider" => match self.provider.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `provider` property.."),
            },
            "refreshid" => {
                let value: u32 = self.refreshid.take();

                self.refreshid.set(value.clone());

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
     * 07/01/2023
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
                    .param_types([i32::static_type()])
                    .return_type::<i32>()
                    .build()
            ]
        })

        //println!("PROPERTIES: {:?}", PROPERTIES);//TEST
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
impl WidgetImpl for GpuPage {}

/**
 * Trait Name:
 * GridImpl
 *
 * Description:
 * Trait shared by all grids
 *
 * Made:
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl GridImpl for GpuPage {}
