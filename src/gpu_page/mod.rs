// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (GpuPage)
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
// Custom GObjects
mod imp;
use imp::ModificationWindowContainer;

// Imports
// std
use std::cell::RefMut;
// gtk-rs
use adwaita::{
    gio, glib,
    Application, ViewStack
};
use gio::Settings;
use glib::{
    clone, closure, Object
};
use gtk::{
    prelude::*, subclass::prelude::*,
    Align, Button, Grid, Label, LayoutChild, Orientation
};

// Modules
use crate::{modificationwindow::ModificationWindow, provider::Provider, APP_ID};

// GObject wrapper for GpuPage
glib::wrapper! {
    pub struct GpuPage(ObjectSubclass<imp::GpuPage>)
    @extends gtk::Grid, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

/**
 * Trait Name:
 * GpuPage
 *
 * Description:
 * Trait shared by all properties
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
impl GpuPage {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new GpuPage object
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
    pub fn new(uuid: &str, name: &str, provider: Provider) -> Self {
        // Create new page    
        let obj: GpuPage = Object::builder::<GpuPage>().build();

        // Set custom properties
        obj.set_property("uuid", String::from(uuid));
        obj.set_property("name", String::from(name));
        obj.set_property("provider", &provider);

        // Apply any setup actions that need the above properties
        obj.setup_settings();
        obj.setup_widgets();

        // Return final object
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
     * load_views
     *
     * Description:
     * Loads all saved views from settings, then calls functions to populate
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
    fn load_views(&self) {
        // Get reference to settings object
        let settings_obj: &Settings = self.settings();

        // Load list of Views
        let loaded_views_data: Vec<String> = settings_obj.get::<Vec<String>>("viewconfigs");
        // println!("views saved:`{}`", loaded_views_data.len()); //TEST
        // println!("views saved:`{:?}`", loaded_views_data); //TEST

        // Remove old content grid
        match self.child_at(0, 0) {
            Some(child) => self.remove(&child),
            None => panic!("unable to find content grid"),
        }

        // Create new content grid
        let content_grid: Grid = Grid::builder()
            .name("content_grid")
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .vexpand_set(true)
            .build();
        self.attach(&content_grid, 0, 0 as i32, 1, 1);

        // Create Views
        // If present in saved settings, use! otherwise follow below defaults
        if let 0 = loaded_views_data.len() {
            // Create default view
            let new_grid_name: String = String::from("default_grid");
            let new_grid: Grid = Grid::builder()
                .name(&new_grid_name)
                .orientation(Orientation::Horizontal)
                .build();
            content_grid.attach(&new_grid, 0, 0 as i32, 100, 12);

            // Build title label & add to grid
            let label_value: String = String::from("Please add a View using 'Add View' button");
            let new_title_label: Label = Label::builder()
                .label(&label_value)
                .name("default")
                .hexpand(true)
                .hexpand_set(true)
                .halign(Align::Center)
                .margin_top(40)
                .margin_bottom(40)
                .build();
            new_grid.attach(&new_title_label, 1, 1, 1, 1);

            // Add edit button for current view
            // Fetch grid's layout manager
            match new_grid.layout_manager() {
                Some(grid_manager) => {
                    // Create add_view_button
                    let add_view_button: Button = Button::builder()
                        .name("add_view_button")
                        .label("Add View")
                        .margin_start(12)
                        .margin_end(12)
                        .margin_top(12)
                        .margin_bottom(12)
                        .halign(Align::Center)
                        .build();
                    new_grid.attach(&add_view_button, 0, 80 as i32, 1, 1);
                    add_view_button.connect_clicked(clone!(@weak self as gpage => move |_| {
                        // Create modification window
                        // Borrow (mutable) the window's container
                        let mut modification_window_container: RefMut<ModificationWindowContainer> = gpage.imp().modification_window.borrow_mut();

                        // Get state from settings
                        modification_window_container.open = gpage.imp().get_setting::<bool>("modification-open");

                        // Check if an object is stored
                        match &modification_window_container.window {
                            Some(_window) => {
                                // println!("..window exists");//DEBUG

                                // Check if the window is already open
                                match modification_window_container.open {
                                    false => {
                                        // println!("....opening window");//DEBUG

                                        // Create an app object
                                        let app: Application = Application::builder().application_id(APP_ID).build();

                                        // Create new modification window
                                        let new_modification_window: ModificationWindow = ModificationWindow::new(&app, -1, &gpage.property::<String>("uuid"), &gpage);

                                        // Show new modification window
                                        new_modification_window.show();

                                        // Store object and state back in container
                                        modification_window_container.open = true;
                                        modification_window_container.window = Some(new_modification_window);
                                    },
                                    true => {
                                        println!("....window already open");//DEBUG
                                    },
                                }
                            },
                            None => {
                                // println!("..window does not exist");//DEBUG
                                // println!("....opening window");//DEBUG

                                // Create an app object
                                let app: Application = Application::builder().application_id(APP_ID).build();

                                // Create modification window
                                let new_modification_window: ModificationWindow = ModificationWindow::new(&app, -1, &gpage.property::<String>("uuid"), &gpage);

                                // Show new modification window
                                new_modification_window.show();

                                // Store object and state back in container
                                modification_window_container.open = true;
                                modification_window_container.window = Some(new_modification_window);
                            },
                        }

                        // Set new state in settings
                        gpage.imp().update_setting::<bool>("modification-open", modification_window_container.open);
                    }));

                    // Set layout properties of button
                    let child_manager: LayoutChild = grid_manager.layout_child(&add_view_button);
                    child_manager.set_property("row-span", 2);
                    child_manager.set_property("column-span", 2);
                }
                None => panic!("Cannot fetch layout manager of grid.."),
            }

            // Save built view
            // Create new view stack
            let new_stack: ViewStack = ViewStack::new();
            // Add object
            new_stack.add_titled(&new_grid, Some("default"), "Default");
            // Add icon
            //NOTE: see <https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ViewStack.html>
            //      function "add_titled_with_icon" not in stable yet
            let new_item = new_stack.page(&new_grid);
            new_item.set_icon_name(Some("package-x-generic-symbolic"));
            // Replace current view stack
            self.imp().replace_stack(Some(&new_stack));
        } else {
            // Create temporary structure for sorting loaded data
            let mut loaded_views: Vec<String> = vec![String::from(""); loaded_views_data.len()];

            // Create new view stack
            let new_stack = ViewStack::new();

            // For each loaded view
            for index in 0..loaded_views_data.len() {
                // println!("item: `{}`", loaded_views_data[index]); //TEST

                // Split current item into the 4 parts
                let parts: Vec<&str> = loaded_views_data[index].split(':').collect::<Vec<&str>>();

                // Catch any malformed items
                if parts.len() != 3 {
                    panic!("Malformed gschema data..");
                }

                // If from valid page
                if parts[0] == self.property::<String>("uuid") {
                    // println!("VALID UUID"); //TEST

                    // If a valid position
                    match parts[1].parse::<usize>() {
                        Ok(position) => {
                            if position <= loaded_views_data.len() {
                                // println!("VALID POSITION INDEX: `{}`", position); //TEST

                                // Add to final list
                                loaded_views[position] = parts[2].to_owned();
                            }
                        }
                        Err(_) => panic!("Invalid Property position in gschema data.."),
                    }
                }
            }

            // Save final list of Views
            // println!("POPULATING VIEWS");
            let mut labels: Vec<Label> = Vec::new();
            let mut props: Vec<String> = Vec::new();
            for index in 0..loaded_views.len() {
                // println!("VIEW {}", index);
                // Grab all saved properties
                let properties: Vec<String> =
                    self.imp().check_properties_for_view(&loaded_views[index]);

                // println!("GOT {} PROPERTIES FOR VIEW {}", properties.len(), index);

                // Add property items to the final list
                for prop in &properties {
                    props.push(prop.as_str().clone().to_owned());
                }

                // Create new view
                let new_grid_name: String = index.to_string() + "_grid";
                let new_grid: Grid = Grid::builder()
                    .name(&new_grid_name)
                    .orientation(Orientation::Horizontal)
                    .margin_start(12)
                    .margin_end(12)
                    //.margin_top(12)
                    //.margin_bottom(12)
                    //.halign(Align::Center)
                    //.valign(Align::Center)
                    //.hexpand(true)
                    //.vexpand(true)
                    .build();
                content_grid.attach(&new_grid, 0, 0 as i32, 100, 12);

                // Populate view given list of properties
                let output: (Grid, Vec<Label>) =
                    self.imp().create_properties(new_grid, properties, labels);
                let new_view_grid: Grid = output.0;
                labels = output.1;

                // If no labels (from no properties)
                if labels.len() == 0 {
                    // Build title label & add to grid
                    let label_value: String =
                        String::from("Please edit this View using 'Edit View' button");
                    let new_title_label: Label = Label::builder()
                        .label(&label_value)
                        .name("default")
                        .hexpand(true)
                        .hexpand_set(true)
                        .halign(Align::Center)
                        .margin_top(40)
                        .margin_bottom(40)
                        .build();
                    new_view_grid.attach(&new_title_label, 1, 1, 1, 1);
                }

                // Add edit button for current view
                // Fetch grid's layout manager
                match new_view_grid.layout_manager() {
                    Some(grid_manager) => {
                        // Create edit button
                        let edit_button: Button = Button::builder()
                            .name("edit_button")
                            .label("Edit View")
                            .margin_start(12)
                            .margin_end(12)
                            .margin_top(12)
                            .margin_bottom(12)
                            .halign(Align::Center)
                            .build();
                        new_view_grid.attach(&edit_button, 0, 80, 1, 1);
                        edit_button.connect_clicked(clone!(@weak self as gpage => move |_| {
                            // Create modification window
                            // Borrow (mutable) the window's container
                            let mut modification_window_container: RefMut<ModificationWindowContainer> = gpage.imp().modification_window.borrow_mut();

                            // Get state from settings
                            modification_window_container.open = gpage.imp().get_setting::<bool>("modification-open");

                            // Check if an object is stored
                            match &modification_window_container.window {
                                Some(_window) => {
                                    // println!("..window exists");//DEBUG

                                    // Check if the window is already open
                                    match modification_window_container.open {
                                        false => {
                                            // println!("....opening window");//DEBUG

                                            // Create an app object
                                            let app: Application = Application::builder().application_id(APP_ID).build();

                                            // Create new modification window
                                            let new_modification_window: ModificationWindow = ModificationWindow::new(&app, index as i32, &gpage.property::<String>("uuid"), &gpage);

                                            // Show new modification window
                                            new_modification_window.show();

                                            // Store object and state back in container
                                            modification_window_container.open = true;
                                            modification_window_container.window = Some(new_modification_window);
                                        },
                                        true => {
                                            println!("....window already open");//DEBUG
                                        },
                                    }
                                },
                                None => {
                                    // println!("..window does not exist");//DEBUG
                                    // println!("....opening window");//DEBUG

                                    // Create an app object
                                    let app: Application = Application::builder().application_id(APP_ID).build();

                                    // Create modification window
                                    let new_modification_window: ModificationWindow = ModificationWindow::new(&app, index as i32, &gpage.property::<String>("uuid"), &gpage);

                                    // Show new modification window
                                    new_modification_window.show();

                                    // Store object and state back in container
                                    modification_window_container.open = true;
                                    modification_window_container.window = Some(new_modification_window);
                                },
                            }

                            // Set new state in settings
                            gpage.imp().update_setting::<bool>("modification-open", modification_window_container.open);
                        }));

                        // Set layout properties of button
                        let child_manager: LayoutChild = grid_manager.layout_child(&edit_button);
                        child_manager.set_property("row-span", 2);
                        child_manager.set_property("column-span", 2);

                        // Create add_view_button
                        let add_view_button: Button = Button::builder()
                            .name("add_view_button")
                            .label("Add View")
                            .margin_start(12)
                            .margin_end(12)
                            .margin_top(12)
                            .margin_bottom(12)
                            .halign(Align::Center)
                            .build();
                        new_view_grid.attach(&add_view_button, 0, 82 as i32, 1, 1);
                        add_view_button.connect_clicked(clone!(@weak self as gpage => move |_| {
                            // Create modification window
                            // Borrow (mutable) the window's container
                            let mut modification_window_container: RefMut<ModificationWindowContainer> = gpage.imp().modification_window.borrow_mut();

                            // Get state from settings
                            modification_window_container.open = gpage.imp().get_setting::<bool>("modification-open");

                            // Check if an object is stored
                            match &modification_window_container.window {
                                Some(_window) => {
                                    // println!("..window exists");//DEBUG

                                    // Check if the window is already open
                                    match modification_window_container.open {
                                        false => {
                                            // println!("....opening window");//DEBUG

                                            // Create an app object
                                            let app: Application = Application::builder().application_id(APP_ID).build();

                                            // Create new modification window
                                            let new_modification_window: ModificationWindow = ModificationWindow::new(&app, -1, &gpage.property::<String>("uuid"), &gpage);

                                            // Show new modification window
                                            new_modification_window.show();

                                            // Store object and state back in container
                                            modification_window_container.open = true;
                                            modification_window_container.window = Some(new_modification_window);
                                        },
                                        true => {
                                            println!("....window already open");//DEBUG
                                        },
                                    }
                                },
                                None => {
                                    // println!("..window does not exist");//DEBUG
                                    // println!("....opening window");//DEBUG

                                    // Create an app object
                                    let app: Application = Application::builder().application_id(APP_ID).build();

                                    // Create modification window
                                    let new_modification_window: ModificationWindow = ModificationWindow::new(&app, -1, &gpage.property::<String>("uuid"), &gpage);

                                    // Show new modification window
                                    new_modification_window.show();

                                    // Store object and state back in container
                                    modification_window_container.open = true;
                                    modification_window_container.window = Some(new_modification_window);
                                },
                            }

                            // Set new state in settings
                            gpage.imp().update_setting::<bool>("modification-open", modification_window_container.open);
                        }));

                        // Set layout properties of button
                        let child_manager: LayoutChild =
                            grid_manager.layout_child(&add_view_button);
                        child_manager.set_property("row-span", 2);
                        child_manager.set_property("column-span", 2);
                    }
                    None => panic!("Cannot fetch layout manager of grid.."),
                }

                // Save built view
                // Add object
                let new_stack_item_name: String = index.to_string() + "_stack_item";
                new_stack.add_titled(
                    &new_view_grid,
                    Some(&new_stack_item_name),
                    &loaded_views[index],
                );
                // println!("NEW STACK ITEM: `{}`", new_stack_item_name);//TEST
                // Add icon
                //NOTE: see <https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ViewStack.html>
                //      function "add_titled_with_icon" not in stable yet
                let new_item = new_stack.page(&new_view_grid);
                new_item.set_icon_name(Some("package-x-generic-symbolic"));
            }

            // if properties exist, call create_updater() function to add time-delayed callback to update appropriate labels
            if props.len() > 0 {
                self.imp().create_updater(labels, props);
            } // else {
              //     println!("No properties, no callbacks!");
              // }

            // Replace current view stack
            self.imp().replace_stack(Some(&new_stack));
        }
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
        // Load stored views
        self.load_views();

        // Connect closure to re-load (now updated) stored views when a modification window is closed
        //NOTE: expected return value seems to be broken - look at imp.rs:395
        self.connect_closure(
            "update-views",
            false,
            closure!(move |gpage: GpuPage, current_view: i32| {
                // println!("closure parameter: `{}`", current_view); //TEST

                // Reload views
                // println!("reloading views.."); //TEST
                gpage.load_views();
                // println!("views reloaded.."); //TEST

                // If and edit is made (and not a delete)
                if current_view != -1 {
                    // println!("switching to page: `{}`", current_view); //TEST

                    // Set to correct view
                    gpage
                        .imp()
                        .view_switcher
                        .stack()
                        .unwrap()
                        .set_visible_child_name(
                            (current_view.to_string() + "_stack_item").as_str(),
                        );
                }

                // Return final value
                //0.to_value()  // "Invalid return value: expected (), got gint",
                0 // "Invalid return value: expected (), got gint",
                  //() // 'Closure returned no value but the caller expected a value of type gint'
                  //   // 'Closure returned no value but the caller expected a value of type gint'
            }),
        );
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
 * 02/11/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl Default for GpuPage {
    fn default() -> Self {
        Self::new("", "Default", Provider::default())
    }
}
