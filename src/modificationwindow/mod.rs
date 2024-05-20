// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * mod.rs
 *
 * Description:
 * Public-facing interface/wrapper for our custom GObject (ModificationWindow)
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
// Custom GObjects
mod imp;

// Imports
// std
use std::cell::RefMut;
// gtk-rs
use adwaita::{
    gio, glib,
    prelude::*, subclass::prelude::*,
    ActionRow
};
use gio::Settings;
use glib::{
    clone, Object
};
use gtk::{
    Adjustment, DropDown, StringList
};

// Modules
use crate::{
    gpu_page::GpuPage, modificationwindow::imp::ParentContainer,
    modificationwindow::imp::ViewComponent, APP_ID,
};

// GObject wrapper for ModificationWindow
glib::wrapper! {
    pub struct ModificationWindow(ObjectSubclass<imp::ModificationWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

/**
 * Trait Name:
 * ModificationWindow
 *
 * Description:
 * Trait shared by all main windows
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
impl ModificationWindow {
    /**
     * Name:
     * new
     *
     * Description:
     * Create a new ModificationWindow object
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
    pub fn new(
        app: &adwaita::Application,
        view_id: i32,
        uuid: &str,
        parent_window: &GpuPage,
    ) -> Self {
        // Create new window
        let obj: ModificationWindow = Object::builder::<ModificationWindow>()
            .property("application", app)
        .build();

        // Set custom properties
        obj.set_property("old-view-id", view_id);
        obj.set_property("new-view-id", view_id);
        obj.set_property("uuid", String::from(uuid));

        // Set ref to parent
        {
            let mut modification_window_container: RefMut<ParentContainer> =
                obj.imp().parent_window.borrow_mut();
            modification_window_container.window = Some(parent_window.to_owned());
        }

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
     * 04/12/2022
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
            .expect("`settings` should not be set before calling `setup_settings`..");
    }

    /**
     * Name:
     * settings
     *
     * Description:
     * Get settings for APP_ID
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
    // fn settings(&self) -> &Settings {
    //     self.imp()
    //         .settings
    //         .get()
    //         .expect("`settings` should be set in `setup_settings`.")
    // }

    /**
     * Name:
     * setup_widgets
     *
     * Description:
     * Set up all widgets
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
    fn setup_widgets(&self) {
        // Retrieve names of stored views
        let view_title_list: Vec<String> = self.imp().get_setting::<Vec<String>>("viewconfigs");
        // let view_title_list: Vec<GString> = self.imp().get_setting::<Vec<GString>>("viewconfigs");
        // let view_title_list: Vec<GString> = self.settings().strv("viewconfigs");
        let view_id: String = self.property::<i32>("old-view-id").to_string();

        // Create empty string for the title
        let mut view_title: String = String::from("Default");

        // Retrieve name of current view
        let mut view_amount: usize = view_title_list.len();
        if view_amount == 0 {
            // First view being added
            // Set new view id ('0' as is first view)
            self.set_property("new-view-id", 0);
        } else if view_id == "-1" {
            // Create new view title and id
            view_title = String::from("New");
            self.set_property("new-view-id", view_amount as i32);

            // Increment count for spinwheel
            view_amount += 1;

            // println!("   View ID: {}", view_id); //TEST
            // println!("View Title: {}", view_title); //TEST
        } else {
            for item in view_title_list {
                let sub_items: Vec<&str> = item.split(':').collect();
                if sub_items[1] == view_id {
                    view_title = sub_items[2].to_owned();
                    break;
                }
            }

            // println!("   View ID: {}", view_id); //TEST
            // println!("View Title: {}", view_title); //TEST
        }

        // Store name of current view
        self.set_property("old-view-title", view_title.clone());
        self.set_property("new-view-title", view_title);

        // Retrieve list of in-use properties
        let view_components_list: Vec<String> = self.imp().get_setting::<Vec<String>>("viewcomponentconfigs");
        // let view_components_list = self.settings().strv("viewcomponentconfigs");
        // println!("Possible Components: {:?}", view_components_list); //TEST

        // Create list of components in current view
        let mut final_components: Vec<ViewComponent> = vec![];

        // Create final list of dropdowns
        let mut dropdowns: Vec<DropDown> = vec![];

        // println!("LETS GET LOOPIN"); //TEST
        // If list of components is not empty
        if view_components_list.len() != 0 {
            // For the list of components
            for index in 0..view_components_list.len() {
                // Check if valid component
                // println!("item: `{}`", view_components_list[index]); //TEST
                let sub_items: Vec<&str> = view_components_list[index].split(':').collect();
                if sub_items[1] == self.property::<String>("old-view-title") {
                    // println!("View Component: {}", sub_items[3]); //TEST

                    // Create new item
                    let new_item: ViewComponent = ViewComponent {
                        name: String::from(sub_items[3]),
                        position: sub_items[2]
                            .parse::<i32>()
                            .expect("Malformed gschema data.."),
                    };
                    final_components.push(new_item);

                    // Create list of options
                    //TODO: Update to use global list
                    let items: [&str; 8] = [
                        "None",
                        "GPU Utilization",
                        "GPU Temperature",
                        "Power Usage",
                        "Memory Usage",
                        "Memory Total",
                        "Memory Controller Usage",
                        "Fan Speed",
                    ];
                    let model: StringList = StringList::new(&items);

                    // Create dropdown choice
                    let dropdown_input_name: String = String::from("dropdown_input_")
                        + final_components.len().to_string().as_str();
                    let dropdown_input: DropDown = DropDown::builder()
                        .name(&dropdown_input_name)
                        .model(&model)
                        .build();

                    // Set current selected option
                    //TODO: Update to use global list
                    match sub_items[3] {
                        "none" => {
                            dropdown_input.set_selected(0);
                            // println!("`none` active"); //TEST
                        }
                        "util" => {
                            dropdown_input.set_selected(1);
                            // println!("`util` active"); //TEST
                        }
                        "temp" => {
                            dropdown_input.set_selected(2);
                            // println!("`temp` active"); //TEST
                        }
                        "power_usage" => {
                            dropdown_input.set_selected(3);
                            // println!("`power_usage` active"); //TEST
                        }
                        "memory_usage" => {
                            dropdown_input.set_selected(4);
                            // println!("`memory_usage` active"); //TEST
                        }
                        "memory_total" => {
                            dropdown_input.set_selected(5);
                            // println!("`memory_total` active"); //TEST
                        }
                        "mem_ctrl_util" => {
                            dropdown_input.set_selected(6);
                            // println!("`mem_ctrl_util` active"); //TEST
                        }
                        "fan_speed" => {
                            dropdown_input.set_selected(7);
                            // println!("`fan_speed` active"); //TEST
                        }
                        _ => panic!("..Unknown property in view config"),
                    }

                    // Create row to hold dropdown_input
                    let row_name: String = String::from("view_component_row_")
                        + final_components.len().to_string().as_str();
                    let row_title: String = String::from("View Component ")
                        + final_components.len().to_string().as_str();
                    let row: ActionRow = ActionRow::builder()
                        .name(&row_name)
                        .title(&row_title)
                        .subtitle("")
                        .activatable(false)
                        .selectable(false)
                        .build();

                    // Add dropdown_input to row
                    row.set_child(Some(&dropdown_input));

                    // Add dropdown to list of dropdowns
                    dropdowns.push(dropdown_input);
                    // println!("new number of stored dropdowns: `{}`", dropdowns.len()); //TEST

                    // Add row to ListBox
                    // println!("inserting in position: `{}`", (1 + final_components.len())); //TEST
                    self.imp()
                        .view_modifier_listbox
                        .insert(&row, (2 + final_components.len()) as i32);
                }
            }
        }
        // println!("DONE"); //TEST

        // Set dropdowns
        self.imp().dropdowns.set(dropdowns);

        // Get current number of view components before we get rid of this..
        let current_view_component_amount: f64 = final_components.len() as f64;
        // println!(
        //     "amount of current components: `{}`",
        //     current_view_component_amount
        // ); //TEST

        // Bind components list to struct member
        self.imp().view_components_list.set(final_components);

        // Insert old name as current content of view-name text box
        self.imp()
            .view_name_input
            .set_placeholder_text(Some(&self.property::<String>("old-view-title")));

        // Set character limit to textbox (10)
        self.imp().view_name_input.set_max_length(10);
        self.imp().view_name_input.set_max_width_chars(10);

        // Create adjustment settings for number of view components SpinButton
        // NOTE: Linked to the upper limit to the total different properties
        let adjustment: Adjustment =
            Adjustment::new(current_view_component_amount, 0.0, 10.0, 1.0, 2.0, 0.0);
        self.imp()
            .view_components_amount_input
            .configure(Some(&adjustment), 1.0, 0);

        // Create adjustment settings for view position SpinButton
        // NOTE: linked to the upper limit to the total views
        if view_id == "-1" {
            let adjustment: Adjustment =
                Adjustment::new(view_amount as f64, 1.0, view_amount as f64, 1.0, 2.0, 0.0);
            self.imp()
                .view_position_input
                .configure(Some(&adjustment), 1.0, 0);
        } else {
            let adjustment: Adjustment = Adjustment::new(
                view_id.parse::<f64>().expect("Missing `old-view-id`..") + 1.0,
                1.0,
                view_amount as f64,
                1.0,
                2.0,
                0.0,
            );
            self.imp()
                .view_position_input
                .configure(Some(&adjustment), 1.0, 0);
        }

        // Buttons
        // Apply
        self.imp().view_modification_apply_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                // Save any changes to the view
                // println!("APPLYING CHANGES.."); //TEST
                window.imp().update_stored_data();
                // println!("CHANGES APPLIED.."); //TEST

                // Emit signal to notify changes made to view (and thus reload required)
                let modification_window_container: RefMut<ParentContainer> = window.imp().parent_window.borrow_mut();
                let _result = modification_window_container.window.as_ref().unwrap().emit_by_name::<i32>("update-views", &[&window.property::<i32>("new-view-id")]);

                // Close window
                window.close();
            }),
        );
        // Cancel
        self.imp().view_modification_cancel_button.connect_clicked(
            clone!(@weak self as window => move |_| {
                // Cancel any changes to the view
                // println!("NOT APPLYING CHANGES..");

                // Close window
                window.close();
            }),
        );
        // Delete
        if view_id == "-1" {
            // If a new view, we don't need to delete
            self.imp().view_modification_delete_button.hide();
        } else {
            self.imp().view_modification_delete_button.connect_clicked(
                clone!(@weak self as window => move |_| {
                    // Delete the view
                    // println!("DELETING VIEW.."); //TEST
                    window.imp().delete_stored_data();
                    // println!("VIEW DELETED.."); //TEST

                    // Emit signal to notify changes made to view (and thus reload required)
                    let modification_window_container: RefMut<ParentContainer> = window.imp().parent_window.borrow_mut();
                    let _result = modification_window_container.window.as_ref().unwrap().emit_by_name::<i32>("update-views", &[&(-1).to_value()]);

                    // Close window
                    window.close();
                }),
            );
        }
    }

    /**
     * Name:
     * restore_data
     *
     * Description:
     * Restore properties from settings
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn restore_data(&self) {
        //
    }

    /**
     * Name:
     * setup_callbacks
     *
     * Description:
     * Child widget callbacks
     *
     * Made:
     * 04/12/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO
     */
    fn setup_callbacks(&self) {
        //
    }

    /**
     * Name:
     * setup_actions
     *
     * Description:
     * Define all actions from UI
     *
     * Made:
     * 04/12/2022
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
        //
    }
}
