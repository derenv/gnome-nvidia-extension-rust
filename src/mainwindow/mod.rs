// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (MainWindow)
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
// Custom GObjects
mod imp;
use imp::SettingsWindowContainer;

// Imports
// std
use std::cell::RefMut;
// gtk-rs
use adwaita::{
    gio, glib,
    prelude::*, subclass::prelude::*
};
use gio::{
    Settings, SimpleAction
};
use glib::{
    clone, closure, Object
};

// Modules
use crate::{provider::Provider, settingswindow::SettingsWindow, APP_ID};

// GObject wrapper for MainWindow
glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/**
 * Trait Name:
 * MainWindow
 *
 * Description:
 * Trait shared by all main windows
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
impl MainWindow {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new MainWindow object
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
    pub fn new(
        app: &adwaita::Application
    ) -> Self {
        // Create new window
        Object::builder::<MainWindow>()
            .property("application", app)
        .build()
    }

    /**
     * Name:
     * setup_settings
     *
     * Description:
     * Load settings for APP_ID
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
    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    /**
     * Name:
     * settings
     *
     * Description:
     * Get settings for APP_ID
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
    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    /**
     * Name:
     * setup_widgets
     *
     * Description:
     * Set up all widgets
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
    fn setup_widgets(&self) {
        // Connect closure to re-load stored views (with different settings) when a settings window is closed
        //NOTE: expected return value seems to be broken - look at imp.rs:1772
        self.connect_closure(
            "update-all-views",
            false,
            closure!(move |window: MainWindow| {
                println!("closure called!"); //TEST

                // Reload views
                println!("reloading views.."); //TEST
                window.imp().refresh_cards();
                println!("views reloaded.."); //TEST

                // Return final value
                0
            }),
        );
    }

    /**
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn restore_data(&self) {
        /*
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let gpu_page_layout: Vec<Vec<String>>


            // Deserialize data from file to vector
            let backup_data: Vec<TaskData> = serde_json::from_reader(file).expect(
                "It should be possible to read `backup_data` from the json file.",
            );

            // Convert `Vec<TaskData>` to `Vec<TaskObject>`
            let task_objects: Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();

            // Insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
            self.append_page();
        }
        */
    }

    /**
     * Name:
     * setup_callbacks
     *
     * Description:
     * Child widget callbacks
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        /*
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_task();
            }),
        );
        */
    }

    /**
     * Name:
     * setup_actions
     *
     * Description:
     * Define all actions from UI
     *
     * Made:
     * 09/10/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * Before we move on to other aspects of actions, let's appreciate a few things
     * that are curious here. The "win" part of "win.close" is the group of the action.
     * But how does GTK know that "win" is the action group of our window? The answer
     * is that it is so common to add actions to windows and applications that there
     * are already two predefined groups available:
     * - "app" for actions global to the application, and
     * - "win" for actions tied to an application window.
     * Also, if we had multiple instances of the same windows we would expect that only
     * the currently focused window will be closed when activating "win.close". And indeed,
     * the "win.close" will be dispatched to the currently focused window. However, that
     * also means that we actually define one action per window instance. If we want to
     * have a single globally accessible action instead, we call add_action on our application instead.
     */
    fn setup_actions(&self) {
        // Create action from key "open_nvidia_settings" and add to action group "win"
        let open_nvidia_settings: SimpleAction = SimpleAction::new("open_nvidia_settings", None);
        open_nvidia_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Get state from settings
            let app_settings_open: bool = window.imp().get_setting::<bool>("nvidia-settings-open");

            //let defaultAppSystem = Shell.AppSystem.get_default();
            //let nvidiaSettingsApp = defaultAppSystem.lookup_app('nvidia-settings.desktop');
            //let def = shell::Edge::Top;
            //let dd = gio::DesktopAppInfo::from_filename("nvidia-settings.desktop");

            match app_settings_open {
                false => {
                    // Grab current stored provider
                    let mut provider: Option<Provider> = window.property("provider");

                    // Check if provider exists
                    match provider {
                        Some(prov) => {
                            // Open Nvidia Settings
                            match prov.open_settings() {
                                Ok(_) => {
                                    println!("Opening the Nvidia Settings app..");

                                    // Set state in settings
                                    window.imp().update_setting::<bool>("nvidia-settings-open", true);
                                },
                                Err(err) => println!(
                                    "An error occured: {}",
                                    err
                                ),
                            }
                        },
                        None => {
                            // Check provider type
                            let provider_type: i32 = window.imp().get_setting::<i32>("provider");

                            // Create new provider
                            //println!("Creating new Provider..");//TEST
                            window.set_property("provider", Some(window.imp().create_provider(provider_type)));

                            // Grab new provider
                            provider = window.property("provider");

                            // Open Nvidia Settings
                            println!("Opening the Nvidia Settings app..");//TEST
                            match provider {
                                Some(prov) => {
                                    // Open Nvidia Settings
                                    match prov.open_settings() {
                                        Ok(_) => {
                                            println!("Opening the Nvidia Settings app..");

                                            // Set state in settings
                                            window.imp().update_setting::<bool>("nvidia-settings-open", true);
                                        },
                                        Err(err) => println!(
                                            "An error occured: {}",
                                            err
                                        ),
                                    }
                                },
                                None => panic!("Cannot find `Provider`!")
                            }
                        }
                    }
                }
                true => println!("Nvidia Settings app already open!"),
            }
        }));
        self.add_action(&open_nvidia_settings);

        let open_app_settings: SimpleAction = SimpleAction::new("open_app_settings", None);
        open_app_settings.connect_activate(clone!(@weak self as window => move |_, _| {
            // Borrow (mutable) the window's container
            let mut settings_window_container: RefMut<SettingsWindowContainer> = window.imp().settings_window.borrow_mut();

            // Get state from settings
            settings_window_container.open = window.imp().get_setting::<bool>("app-settings-open");

            // Check if an object is stored
            match &settings_window_container.window {
                Some(_window) => {
                    println!("..window exists");//DEBUG

                    // Check if the window is already open
                    match settings_window_container.open {
                        false => {
                            println!("....opening window");//DEBUG

                            // Create an app object
                            let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                            // Create new settings window
                            let new_settings_window: SettingsWindow = SettingsWindow::new(&app, &window);

                            // Show new settings window
                            new_settings_window.show();

                            // Store object and state back in container
                            settings_window_container.open = true;
                            settings_window_container.window = Some(new_settings_window);
                        },
                        true => {
                            println!("....window already open");//DEBUG
                        },
                    }
                },
                None => {
                    println!("..window does not exist");//DEBUG
                    println!("....opening window");//DEBUG

                    // Create an app object
                    let app: adwaita::Application = adwaita::Application::builder().application_id(APP_ID).build();

                    // Create settings window
                    let new_settings_window: SettingsWindow = SettingsWindow::new(&app, &window);

                    // Show new settings window
                    new_settings_window.show();

                    // Store object and state back in container
                    settings_window_container.open = true;
                    settings_window_container.window = Some(new_settings_window);
                },
            }

            // Set new state in settings
            window.imp().update_setting::<bool>("app-settings-open", settings_window_container.open);
        }));
        self.add_action(&open_app_settings);

        let about: SimpleAction = SimpleAction::new("about", None);
        about.connect_activate(clone!(@weak self as window => move |_, _| {
            // Show About info pop-up
            println!("About pop-up not yet implemented..");//TODO
        }));
        self.add_action(&about);
    }
}
