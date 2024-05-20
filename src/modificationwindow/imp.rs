// SPDX-FileCopyrightText: 2024 Deren Vural
// SPDX-License-Identifier: GPL-3.0-or-later

/**
 * Name:
 * imp.rs
 *
 * Description:
 * Implementation of our custom GObject class (ModificationWindow)
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
// Imports
// std
use std::sync::OnceLock;
use std::cell::{
    Cell, OnceCell, RefCell
};
use std::rc::Rc;
// gtk-rs
use gtk::{
    subclass::prelude::*,
    Button, CompositeTemplate, DropDown, Entry, ListBox, SpinButton, StringList, TemplateChild
};
use adwaita::{
    gio, glib,
    prelude::*, subclass::prelude::*,
    ActionRow
};
use gio::Settings;
use glib::{
    signal::Propagation,
    subclass::InitializingObject, ParamSpec,
    variant::FromVariant, variant::Variant,
    value::Value
};

// Modules
use crate::gpu_page::GpuPage;

/// Structure for storing a SettingsWindow object and any related information
#[derive(Default)]
pub struct ParentContainer {
    pub window: Option<GpuPage>,
}

/// Structure for storing a View item data
#[derive(Default, Clone)]
pub struct ViewComponent {
    pub name: String,
    pub position: i32,
}

/// Object holding the State and any Template Children
#[derive(CompositeTemplate, Default)]
#[template(resource = "/modification-window.ui")]
pub struct ModificationWindow {
    // Public
    pub settings: OnceCell<Settings>,
    pub view_components_list: Cell<Vec<ViewComponent>>,
    pub parent_window: Rc<RefCell<ParentContainer>>,
    pub dropdowns: Cell<Vec<DropDown>>,

    // Private
    uuid: OnceCell<String>,
    old_view_id: OnceCell<i32>,
    new_view_id: Cell<i32>,
    old_view_title: OnceCell<String>,
    new_view_title: Cell<String>,

    // Template Children
    #[template_child]
    pub view_name_input: TemplateChild<Entry>,
    #[template_child]
    pub view_components_amount_input: TemplateChild<SpinButton>,
    #[template_child]
    pub view_position_input: TemplateChild<SpinButton>,
    #[template_child]
    pub view_modifier_listbox: TemplateChild<ListBox>,
    #[template_child]
    pub button_row: TemplateChild<ActionRow>,
    #[template_child]
    pub view_modification_apply_button: TemplateChild<Button>,
    #[template_child]
    pub view_modification_cancel_button: TemplateChild<Button>,
    #[template_child]
    pub view_modification_delete_button: TemplateChild<Button>,
}

/// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ModificationWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NvidiaExtensionModificationWindow";
    type Type = super::ModificationWindow;
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
 * ModificationWindow
 *
 * Description:
 * Trait shared by all ModificationWindow objects
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
     * reorder_data
     *
     * Description:
     * Update view ID's given ???
     *
     * Made:
     * 08/01/2022
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn reorder_data(&self, remove: bool) {
        // Get stored & const view data
        let mut stored_views_data: Vec<String> = self.get_setting::<Vec<String>>("viewconfigs");
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("missing `uuid`..")
            .to_owned();
        // println!("stored views: `{:?}`", stored_views_data); //TEST

        // Get old + new view title
        let new_view_title: String = self.new_view_title.take();
        self.new_view_title.set(new_view_title.clone());

        // Get old ID
        let old_id: i32 = self
            .old_view_id
            .clone()
            .get()
            .expect("`old-view-id` invalid..")
            .to_owned();

        // If present in saved settings
        if stored_views_data.len() == 0 {
            // no views exist
            panic!("no stored views: this shouldn't be happening!"); //programmer error
        } else {
            // Get list of stored viewconfigs
            for index in 0..stored_views_data.len() {
                // Split current viewconfig
                let sub_items: Vec<&str> = stored_views_data[index].split(':').collect();
                // println!("current item: `{}`", stored_views_data[index]); //TEST

                // If viewconfig is for this GPU (i.e. has valid UUID) and IS NOT the current view
                if (sub_items[0] == uuid, sub_items[2] != new_view_title) == (true, true) {
                    // println!("This view needs updated: `{}`", stored_views_data[index]); //TEST

                    // Get ID of current view
                    let this_id: i32 = self.new_view_id.clone().get();

                    // Split current viewconfig
                    let sub_items: Vec<&str> = stored_views_data[index].split(':').collect();

                    // Get ID of this config
                    let mut new_id: i32 = sub_items[1]
                        .parse::<i32>()
                        .expect("Malformed gschema data..");

                    // println!("old_id: `{}`", old_id); //TEST
                    // println!("new_id: `{}`", new_id); //TEST
                    // println!("this_id: `{}`", this_id); //TEST

                    // If viewconfig was at/above current view position
                    match (new_id > this_id, new_id == this_id, remove) {
                        (true, false, true) => {
                            // println!("edit required"); //TEST
                            // println!("removing"); //TEST

                            // If above position & removing
                            // println!("new_id: `{}` modified by -1", new_id); //TEST
                            new_id -= 1; // Modify order
                            // println!("new_id: `{}`", new_id); //TEST

                            // Update record
                            stored_views_data[index] =
                                uuid.clone() + ":" + &new_id.to_string() + ":" + sub_items[2];
                        }
                        (true, false, false) => {
                            // println!("edit required"); //TEST
                            // println!("tff reorder"); //TEST

                            // If above position &  re-order
                            // println!("new_id: `{}` modified by -1", new_id); //TEST
                            new_id -= 1; // Modify order
                            // println!("new_id: `{}`", new_id); //TEST

                            // Update record
                            stored_views_data[index] =
                                uuid.clone() + ":" + &new_id.to_string() + ":" + sub_items[2];
                        }
                        (false, true, false) => {
                            // println!("edit required"); //TEST
                            // println!("ftf reorder"); //TEST

                            // If same position & re-order, we need to check direction of change
                            if old_id == -1 {
                                // if inserting new view
                                // println!("new_id: `{}` modified by +1", new_id); //TEST
                                new_id += 1; // Modify order
                            } else {
                                if old_id < new_id {
                                    // println!("new_id: `{}` modified by -1", new_id); //TEST
                                    new_id -= 1; // Modify order
                                } else if old_id > new_id {
                                    // println!("new_id: `{}` modified by +1", new_id); //TEST
                                    new_id += 1; // Modify order
                                }
                            }
                            // println!("new_id: `{}`", new_id); //TEST

                            // Update record
                            stored_views_data[index] =
                                uuid.clone() + ":" + &new_id.to_string() + ":" + sub_items[2];
                        }
                        _ => {
                            // println!("tuple `{} {} {}`", new_id > this_id, new_id == this_id, remove);

                            // otherwise ignore
                            // println!("new_id: `{}` NOT modified", new_id); //TEST
                            // println!("NO edit required"); //TEST
                        }
                    }
                }
            }

            // Update stored viewconfigs
            // println!("updated views: `{:?}`", stored_views_data); //TEST
            self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
        }
    }

    /**
     * Name:
     * delete_stored_data
     *
     * Description:
     * Delete this view from the stored properties
     *
     * Made:
     * 05/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn delete_stored_data(&self) {
        // Get stored & const view data
        let mut stored_views_data: Vec<String> = self.get_setting::<Vec<String>>("viewconfigs");
        let mut stored_views_components: Vec<String> =
            self.get_setting::<Vec<String>>("viewcomponentconfigs");
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("missing `uuid`..")
            .to_owned();
        // println!("stored views: `{:?}`", stored_views_data); //TEST

        // Get old + new view title
        let old_view_title: String = self
            .old_view_title
            .clone()
            .get()
            .expect("missing `old-view-title`..")
            .to_owned();
        let new_view_title: String = self.new_view_title.take();
        self.new_view_title.set(new_view_title.clone());

        // If present in saved settings
        if stored_views_data.len() == 0 {
            // no views exist
            panic!("no stored views: this shouldn't be happening!"); //programmer error
        } else {
            // index of the view we are deleting
            let mut view_index: i32 = -1;

            // Get list of stored viewconfigs
            for index in 0..stored_views_data.len() {
                // Split current viewconfig
                let sub_items: Vec<&str> = stored_views_data[index].split(':').collect();

                // If viewconfig is for this GPU (i.e. has valid UUID) and has the old name
                if (sub_items[0] == uuid, sub_items[2] == old_view_title) == (true, true) {
                    view_index = index as i32;
                    // println!("match.."); //TEST
                    break;
                }
            }

            // If we found the view
            if view_index == -1 {
                // Not found?
                panic!("view not found: this shouldn't be happening!"); //programmer error
            } else {
                // Delete viewconfig
                stored_views_data.remove(view_index as usize);

                let total_remaining: usize = stored_views_data.len();

                // Update stored viewconfigs
                self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                // println!("viewconfig updated.."); //TEST

                // Re-order remaining views
                if total_remaining != 0 {
                    self.reorder_data(true);
                }

                // Delete associated viewcomponentconfigs
                // println!("Initial components list: `{:?}`", stored_views_components); //TEST
                let mut to_remove: Vec<i32> = vec![];

                // Check if any new viewcomponentconfigs need removed
                for index in 0..stored_views_components.len() {
                    // Split current viewcomponentconfig
                    let sub_items: Vec<&str> = stored_views_components[index].split(':').collect();

                    // If viewcomponentconfig from this view
                    if (sub_items[0] == &uuid, sub_items[1] == old_view_title) == (true, true) {
                        to_remove.push(index as i32);
                        // println!("need to remove: `{}`", index);
                    }
                }

                // If any viewcomponents exist for this view
                if to_remove.len() > 0 {
                    // Reverse order of indices to remove
                    to_remove.reverse();

                    // Update list of viewcomponentconfigs
                    for item_to_remove in to_remove {
                        stored_views_components.remove(item_to_remove as usize);
                        // println!("removed: `{}`", item_to_remove);
                        // println!("new list: `{:?}`", stored_views_components);
                    }
                    // println!("Final components list: `{:?}`", stored_views_components);

                    // Update stored viewcomponentconfigs
                    self.update_setting::<Vec<String>>(
                        "viewcomponentconfigs",
                        stored_views_components,
                    );
                    // println!("saving changes.."); //TEST
                } //  else {
                  // println!("no changes.."); //TEST
                  // }
            }
        }
    }

    /**
     * Name:
     * update_view_components_list
     *
     * Description:
     * Update the stored properties for this view
     *
     * Made:
     * 02/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    pub fn update_stored_data(&self) {
        // Get stored & const view data
        let mut stored_views_data: Vec<String> = self.get_setting::<Vec<String>>("viewconfigs");
        let mut stored_views_components: Vec<String> =
            self.get_setting::<Vec<String>>("viewcomponentconfigs");
        let uuid: String = self
            .uuid
            .clone()
            .get()
            .expect("missing `uuid`..")
            .to_owned();
        // println!("stored views: `{:?}`", stored_views_data); //TEST

        // Get old + new view title
        let old_view_title: String = self
            .old_view_title
            .clone()
            .get()
            .expect("missing `old-view-title`..")
            .to_owned();
        let new_view_title: String = self.new_view_title.take();
        self.new_view_title.set(new_view_title.clone());

        // If present in saved settings, use! otherwise follow below defaults
        if stored_views_data.len() == 0 {
            if stored_views_components.len() != 0 {
                panic!("something really wrong!");
            }

            // Create new viewconfig
            //UUID:POSITION:VIEW_TITLE
            let new_viewconfig: String = String::from(&uuid) + ":0:" + &new_view_title;

            // Save new viewconfig
            self.update_setting::<Vec<String>>("viewconfigs", vec![new_viewconfig]);

            // Get current components
            let mut current_components: Vec<ViewComponent> = self.view_components_list.take();
            let dropdowns: Vec<DropDown> = self.dropdowns.take();
            // println!("number of stored dropdowns: `{}`", dropdowns.len()); //TEST
            // Update list using current state of dropdowns
            for index in 0..current_components.len() {
                // println!("comp: `{}`", current_components[index].name); //TEST
                // println!(" pos: `{}`", current_components[index].position); //TEST

                // Get current dropdown
                let current_dropdown: &DropDown = &dropdowns[index];
                let current_dropdown_value: usize = current_dropdown.selected() as usize;

                // From list of possible properties
                //TODO: Update to use global list
                let items: [&str; 8] = [
                    "none",
                    "util",
                    "temp",
                    "power_usage",
                    "memory_usage",
                    "memory_total",
                    "mem_ctrl_util",
                    "fan_speed",
                ];

                // Update stored name if required
                if current_components[index].name != items[current_dropdown_value] {
                    current_components[index].name = items[current_dropdown_value].to_string();
                }
            }

            // Create new viewcomponentconfigs
            let mut final_viewcomponentconfigs: Vec<String> = vec![];
            // For each selected property component
            for component in current_components {
                //UUID:VIEW_TITLE:POSITION:name
                let formatted_component: String = String::from(&uuid)
                    + ":"
                    + &new_view_title
                    + ":"
                    + &component.position.to_string()
                    + ":"
                    + &component.name;

                // Add to final list
                final_viewcomponentconfigs.push(formatted_component);
            }

            // Save new viewcomponentconfigs
            self.update_setting::<Vec<String>>("viewcomponentconfigs", final_viewcomponentconfigs);
        } else {
            // index used to tell if this is a new viewconfig or not
            let mut view_index: i32 = -1;

            // Get list of stored viewconfigs
            for index in 0..stored_views_data.len() {
                // Split current viewconfig
                let sub_items: Vec<&str> = stored_views_data[index].split(':').collect();

                // If viewconfig is for this GPU (i.e. has valid UUID) and has the old name
                if (sub_items[0] == uuid, sub_items[2] == old_view_title) == (true, true) {
                    view_index = index as i32;
                    // println!("match.."); //TEST
                    break;
                }
            }

            // If we are modifying an existing viewconfig
            if view_index != -1 {
                // Get old + new view id
                let old_view_id: i32 = self
                    .old_view_id
                    .clone()
                    .get()
                    .expect("missing `old-view-id`..")
                    .to_owned();
                let new_view_id: i32 = self.new_view_id.take();
                self.new_view_id.set(new_view_id.clone());

                // Update viewconfigs accordingly
                match (old_view_title == new_view_title, old_view_id == new_view_id) {
                    // MATCH name is different, id is the same
                    (false, true) => {
                        // Remove old viewconfig
                        stored_views_data.remove(view_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &old_view_id.to_string() + ":" + &new_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        // println!("viewconfig updated.."); //TEST
                    }
                    // MATCH name is different, id is different
                    (false, false) => {
                        // Remove old viewconfig
                        stored_views_data.remove(view_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &new_view_id.to_string() + ":" + &new_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        // println!("viewconfig updated.."); //TEST

                        // Re-order remaining views
                        self.reorder_data(false);
                    }
                    // MATCH name is the same, id is different
                    (true, false) => {
                        // Remove old viewconfig
                        stored_views_data.remove(view_index as usize);

                        // Create new viewconfig
                        //UUID:POSITION:VIEW_TITLE
                        let new_viewconfig: String =
                            uuid.clone() + ":" + &new_view_id.to_string() + ":" + &old_view_title;

                        // Update viewconfigs item with new viewconfig
                        stored_views_data.push(new_viewconfig);

                        // Update stored viewconfigs
                        self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);
                        // println!("viewconfig updated.."); //TEST

                        // Re-order remaining views
                        self.reorder_data(false);
                    }
                    // MATCH name is the same, id is the same
                    (true, true) => {
                        // Do nothing
                        // println!("No viewconfig to update.."); //TEST
                    }
                }

                // Create empty final list of viewcomponentconfigs
                let mut final_components_list: Vec<String> = vec![];
                let mut new_components_list: Vec<String> = vec![];
                // println!("Initial components list: `{:?}`", stored_views_components); //TEST

                // Get list current components
                let mut current_components: Vec<ViewComponent> = self.view_components_list.take();
                let dropdowns: Vec<DropDown> = self.dropdowns.take();
                // println!("number of stored components: `{}`", current_components.len()); //TEST
                // println!("number of stored dropdowns: `{}`", dropdowns.len()); //TEST
                // Update list using current state of dropdowns
                for index in 0..current_components.len() {
                    // println!("comp: `{}`", current_components[index].name); //TEST
                    // println!(" pos: `{}`", current_components[index].position); //TEST

                    // Get current dropdown
                    let current_dropdown: &DropDown = &dropdowns[index];
                    let current_dropdown_value: usize = current_dropdown.selected() as usize;

                    // From list of possible properties
                    //TODO: Update to use global list
                    let items: [&str; 8] = [
                        "none",
                        "util",
                        "temp",
                        "power_usage",
                        "memory_usage",
                        "memory_total",
                        "mem_ctrl_util",
                        "fan_speed",
                    ];

                    // Update stored name if required
                    if current_components[index].name != items[current_dropdown_value] {
                        current_components[index].name = items[current_dropdown_value].to_string();
                    }
                }

                // Create list of old components
                let mut old_components_list: Vec<String> = vec![];
                for index in 0..stored_views_components.len() {
                    // Split current viewcomponentconfig
                    let sub_items: Vec<&str> = stored_views_components[index].split(':').collect();

                    // If viewcomponentconfig from this view
                    if (sub_items[0] == &uuid, sub_items[1] == old_view_title) == (true, true) {
                        old_components_list.push(stored_views_components[index].clone());
                    } else {
                        final_components_list.push(stored_views_components[index].clone());
                    }
                }

                // Check if any new viewcomponentconfigs need updated
                // for each new component
                for new_index in 0..current_components.len() {
                    let mut component_modified: bool = false;

                    // Check if old version exists
                    for old_index in 0..old_components_list.len() {
                        // Split current viewcomponentconfig
                        let sub_items: Vec<&str> =
                            old_components_list[old_index].split(':').collect();

                        match (
                            current_components[new_index].name == sub_items[3],
                            old_view_title == new_view_title,
                            current_components[new_index].position.to_string() == sub_items[2],
                        ) {
                            // CORRECT NAME, TITLE CHANGE, POSITION CHANGE => STOP
                            (true, false, false) => {
                                // Create new viewcomponentconfig
                                let new_viewcomponentconfig: String = uuid.clone()
                                    + ":"
                                    + &new_view_title
                                    + ":"
                                    + &current_components[new_index].position.to_string()
                                    + ":"
                                    + &current_components[new_index].name;

                                // Add to updated list of stored viewcomponentconfigs
                                new_components_list.push(new_viewcomponentconfig);
                                component_modified = true;
                                // println!("CHANGES: component modified!"); //TEST
                                break;
                            }
                            // CORRECT NAME, TITLE CHANGE, NO POSITION CHANGE => STOP
                            (true, false, true) => {
                                // Create new viewcomponentconfig
                                let new_viewcomponentconfig: String = uuid.clone()
                                    + ":"
                                    + &new_view_title
                                    + ":"
                                    + sub_items[2]
                                    + ":"
                                    + &current_components[new_index].name;

                                // Add to updated list of stored viewcomponentconfigs
                                new_components_list.push(new_viewcomponentconfig);
                                component_modified = true;
                                // println!("CHANGES: component modified!"); //TEST
                                break;
                            }
                            // CORRECT NAME, NO TITLE CHANGE, POSITION CHANGE => STOP
                            (true, true, false) => {
                                // Create new viewcomponentconfig
                                let new_viewcomponentconfig: String = uuid.clone()
                                    + ":"
                                    + &old_view_title
                                    + ":"
                                    + &current_components[new_index].position.to_string()
                                    + ":"
                                    + &current_components[new_index].name;

                                // Add to updated list of stored viewcomponentconfigs
                                new_components_list.push(new_viewcomponentconfig);
                                component_modified = true;
                                // println!("CHANGES: component modified!"); //TEST
                                break;
                            }

                            // NO CHANGES => STOP
                            (true, true, true) => {
                                // Add to updated list of stored viewcomponentconfigs
                                new_components_list.push(old_components_list[old_index].clone());
                                component_modified = true;
                                // println!("NO CHANGES: component not modified.."); //TEST
                                break;
                            }

                            // INCORRECT NAME => IGNORE
                            (false, _, _) => {}
                        }
                    }

                    // Check if a new component
                    if !component_modified {
                        // Create new viewcomponentconfig
                        let new_viewcomponentconfig: String = uuid.clone()
                            + ":"
                            + &old_view_title
                            + ":"
                            + &current_components[new_index].position.to_string()
                            + ":"
                            + &current_components[new_index].name;

                        // Add to updated list of stored viewcomponentconfigs
                        new_components_list.push(new_viewcomponentconfig);
                        // println!("FINAL new component!"); //TEST
                    } // else {
                      //     println!("FINAL component modified!"); //TEST
                      // }
                }

                // Combine new/modified list with other views
                final_components_list.append(&mut new_components_list);

                // Store final list
                // println!("Final components list: `{:?}`", final_components_list);
                // println!("saving changes.."); //TEST
                self.update_setting::<Vec<String>>("viewcomponentconfigs", final_components_list);
                // println!("changes saved.."); //TEST
            } else {
                // Create new viewconfig
                //UUID:POSITION:VIEW_TITLE
                let new_viewconfig: String = uuid.clone()
                    + ":"
                    + &self.new_view_id.clone().get().to_string()
                    + ":"
                    + &new_view_title;

                // Update viewconfigs item with new viewconfig
                stored_views_data.push(new_viewconfig);

                // Update stored viewconfigs
                self.update_setting::<Vec<String>>("viewconfigs", stored_views_data);

                // Re-order remaining views
                self.reorder_data(false);

                // Get current components
                let mut current_components: Vec<ViewComponent> = self.view_components_list.take();
                let dropdowns: Vec<DropDown> = self.dropdowns.take();
                // println!("number of stored components: `{}`", current_components.len()); //TEST
                // println!("number of stored dropdowns: `{}`", dropdowns.len()); //TEST
                // Update list using current state of dropdowns
                for index in 0..current_components.len() {
                    // println!("comp: `{}`", current_components[index].name); //TEST
                    // println!(" pos: `{}`", current_components[index].position); //TEST

                    // Get current dropdown
                    let current_dropdown: &DropDown = &dropdowns[index];
                    let current_dropdown_value: usize = current_dropdown.selected() as usize;

                    // From list of possible properties
                    //TODO: Update to use global list
                    let items: [&str; 8] = [
                        "none",
                        "util",
                        "temp",
                        "power_usage",
                        "memory_usage",
                        "memory_total",
                        "mem_ctrl_util",
                        "fan_speed",
                    ];

                    // Update stored name if required
                    if current_components[index].name != items[current_dropdown_value] {
                        current_components[index].name = items[current_dropdown_value].to_string();
                    }
                }

                // Create new viewcomponentconfigs
                // For each selected property component
                for component in current_components {
                    //UUID:VIEW_TITLE:POSITION:name
                    let formatted_component: String = uuid.clone()
                        + ":"
                        + &new_view_title
                        + ":"
                        + &component.position.to_string()
                        + ":"
                        + &component.name;

                    // Add to list
                    stored_views_components.push(formatted_component);
                }

                // Update stored viewconfigs
                // println!("Final components list: `{:?}`", stored_views_components);
                self.update_setting::<Vec<String>>("viewcomponentconfigs", stored_views_components);
            }
        }
    }

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
}

/**
 * Name:
 * ModificationWindow
 *
 * Description:
 * Trait defining template callbacks shared by all ModificationWindow objects
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
#[gtk::template_callbacks]
impl ModificationWindow {
    /**
     * Name:
     * view_name_changed
     *
     * Description:
     * Template callback for changing the name of the current view
     *
     * Made:
     * 03/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     * TODO: add a checkmark (or some kind of notification) to tell user the input is valid
     */
    #[template_callback]
    fn view_name_changed(&self, textbox: &Entry) {
        // Get new input
        let new_title: String = textbox.text().to_string();
        // println!("new name: `{}`", new_title); //TEST

        // Update new_view_title
        if new_title != "" {
            // println!("NEW NAME VALID..");
            self.new_view_title.set(new_title);
        } // else {
          //     println!("NEW NAME INVALID..");
          // }
    }

    /**
     * Name:
     * view_position_changed
     *
     * Description:
     * Template callback for changing the position of the current view
     *
     * Made:
     * 08/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn view_position_changed(&self, spinbutton: &SpinButton) {
        // Validate amount
        let new_amount: i32 = spinbutton.value() as i32;
        // println!("input view position: `{}`", new_amount); //TEST
        // println!("new view position: `{}`", new_amount - 1); //TEST

        // Update stored ID
        self.new_view_id.set(new_amount - 1);
    }

    /**
     * Name:
     * view_components_amount_changed
     *
     * Description:
     * Template callback for changing the number of components of the current view
     *
     * Made:
     * 03/01/2023
     *
     * Made by:
     * Deren Vural
     *
     * Notes:
     *
     */
    #[template_callback]
    fn view_components_amount_changed(&self, spinbutton: &SpinButton) {
        // Validate amount
        let new_amount: usize = spinbutton.value() as usize;
        // println!("new number of components: `{}`", spinbutton.value()); //TEST

        let mut components: Vec<ViewComponent> = self.view_components_list.take();
        // println!("stored number of components: `{}`", components.len()); //TEST

        if new_amount < components.len() {
            // Less than previous
            // println!("<"); //TEST

            // subtract end item
            self.view_modifier_listbox.remove(
                &self
                    .view_modifier_listbox
                    .row_at_index((2 + components.len()) as i32)
                    .unwrap(),
            );

            // Modify view component list
            components.remove(components.len() - 1);
        } else if new_amount > components.len() {
            // More than previous
            // println!(">"); //TEST

            // Create list of options
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
            let dropdown_input_name: String =
                String::from("dropdown_input_") + components.len().to_string().as_str();
            let dropdown_input: DropDown = DropDown::builder()
                .name(&dropdown_input_name)
                .model(&model)
                .selected(0)
                .build();

            // Create row to empty
            let row_name: String =
                String::from("view_component_row_") + components.len().to_string().as_str();
            let row_title: String =
                String::from("View Component ") + components.len().to_string().as_str();
            let row: ActionRow = ActionRow::builder()
                .name(&row_name)
                .title(&row_title)
                .subtitle("")
                .activatable(false)
                .selectable(false)
                .build();

            // Add dropdown_input to row
            row.set_child(Some(&dropdown_input));

            // Add new item, needs defaults (i.e. None)
            let pos: i32 = components.len() as i32;
            // println!("inserting in position: `{}`", pos); //TEST
            self.view_modifier_listbox.insert(&row, 3 + pos);

            // Create new item
            let new_item: ViewComponent = ViewComponent {
                name: String::from("none"),
                position: pos,
            };

            // Update list of dropdowns
            let mut dropdowns: Vec<DropDown> = self.dropdowns.take();
            dropdowns.push(dropdown_input);
            // println!("new number of stored dropdowns: `{}`", dropdowns.len()); //TEST
            self.dropdowns.set(dropdowns);

            // Modify view component list
            components.push(new_item);
        } // else if new_amount == components.len() {
          //     // Same as previous
          //     // println!("=="); //TEST

        //     // TODO: ???
        // }

        // Return components list
        self.view_components_list.set(components);
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
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl ObjectImpl for ModificationWindow {
    /**
     * Name:
     * constructed
     *
     * Description:
     * Called during construction, allows calling setup functions
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
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let _obj: glib::BorrowedObject<super::ModificationWindow> = self.obj();
        // obj.setup_settings();
        // obj.setup_widgets();
        // obj.restore_data();
        // obj.setup_callbacks();
        // obj.setup_actions();
    }

    /**
     * Name:
     * properties
     *
     * Description:
     * Create list of custom properties for our GObject
     *
     * Made:
     * 04/12/2022
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
                glib::ParamSpecInt::builder("old-view-id").build(),
                glib::ParamSpecInt::builder("new-view-id").build(),
                glib::ParamSpecString::builder("old-view-title").build(),
                glib::ParamSpecString::builder("new-view-title").build(),
                glib::ParamSpecString::builder("uuid").build()
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
     * 04/12/2022
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
            "old-view-id" => match value.get() {
                Ok(input_old_view_id) => self
                    .old_view_id
                    .set(input_old_view_id)
                    .expect("`old-view-id` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `i32`."),
            },
            "new-view-id" => match value.get() {
                Ok(input_new_view_id) => self.new_view_id.set(input_new_view_id),
                Err(_) => panic!("The value needs to be of type `i32`."),
            },
            "old-view-title" => match value.get() {
                Ok(input_old_view_title) => self
                    .old_view_title
                    .set(input_old_view_title)
                    .expect("`old-view-title` should not be set after calling `setup_widgets()`.."),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "new-view-title" => match value.get() {
                Ok(input_new_view_title) => self.new_view_title.set(input_new_view_title),
                Err(_) => panic!("The value needs to be of type `String`."),
            },
            "uuid" => match value.get() {
                Ok(input_uuid) => self
                    .uuid
                    .set(input_uuid)
                    .expect("`uuid` should not be set after calling constructor.."),
                Err(_) => panic!("The value needs to be of type `String`."),
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
     * 04/12/2022
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
            "old-view-id" => match self.old_view_id.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `old-view-id` property.."),
            },
            "new-view-id" => self.new_view_id.get().to_value(),
            "old-view-title" => match self.old_view_title.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `old-view-title` property.."),
            },
            "new-view-title" => {
                let value: String = self.new_view_title.take();

                self.new_view_title.set(value.clone());

                value.to_value()
            }
            "uuid" => match self.uuid.clone().get() {
                Some(value) => return value.to_value(),
                None => panic!("Cannot get value of `uuid` property.."),
            },
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
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl WidgetImpl for ModificationWindow {}

/**
 * Trait Name:
 * WindowImpl
 *
 * Description:
 * Trait shared by all Window's
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
impl WindowImpl for ModificationWindow {
    /**
     * Name:
     * close_request
     *
     * Description:
     * Run when window closed
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
    fn close_request(&self) -> Propagation {
        // Store state in settings
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
 * 04/12/2022
 *
 * Made by:
 * Deren Vural
 *
 * Notes:
 *
 */
impl AdwWindowImpl for ModificationWindow {}

/**
 * Trait Name:
 * ApplicationWindowImpl
 *
 * Description:
 * Trait shared by all ApplicationWindow's
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
impl ApplicationWindowImpl for ModificationWindow {}

/**
 * Trait Name:
 * AdwApplicationWindowImpl
 *
 * Description:
 * Trait shared by all AdwApplicationWindow's
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
impl AdwApplicationWindowImpl for ModificationWindow {}
