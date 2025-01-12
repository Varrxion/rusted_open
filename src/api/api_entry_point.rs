use std::sync::{Arc, RwLock};

use glfw::{Glfw, Key, PWindow};
use nalgebra::Vector3;

use crate::engine::{graphics::{texture_manager::TextureManager, util::{master_clock::MasterClock, master_graphics_list::MasterGraphicsList}}, key_states::State, scenes::scene_manager::SceneManager};

use super::events::{collision::Collision, movement::Movement};

pub struct ApiEntryPoint {
    first_loop: bool,
}

impl ApiEntryPoint {
    /// Creates a new EntryPoint instance.
    pub fn new() -> Self {
        ApiEntryPoint {
            first_loop: true,
        }
    }

    /// This is the entry point for the framework. I will include sample code here.
    /// Direct changes may be made to the engine itself if needed but this is the "developer-friendly" way to work with the engine.
    pub fn entry_point(&mut self, glfw: &mut Glfw, window: &mut PWindow, master_clock: &mut MasterClock, texture_manager: Arc<RwLock<TextureManager>>, scene_manager: &mut SceneManager, master_graphics_list: &mut MasterGraphicsList, state: &mut State) {

        if self.first_loop==true {
            self.first_loop(texture_manager, scene_manager, master_graphics_list);
        }

        // Retrieve the square from the master graphics list
        let square = master_graphics_list.get_object("debug_playersquare").expect("Object not found");

        // Thou shalt not use frame-based physics
        let delta_time = master_clock.get_delta_time();

        // Apply movement based on active keys (This should really be abstracted somewhere else, but it's here for simplicity. You can move this kind of logic wherever you want.)
        let move_speed = 0.2;
        let rotation_speed = 2.0;
        if state.is_key_pressed(Key::W) {
            Movement::move_object(square.clone(), Vector3::new(0.0, 1.0, 0.0), move_speed, delta_time);
        }
        if state.is_key_pressed(Key::S) {
            Movement::move_object(square.clone(), Vector3::new(0.0, -1.0, 0.0), move_speed, delta_time);
        }
        if state.is_key_pressed(Key::A) {
            Movement::move_object(square.clone(), Vector3::new(-1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if state.is_key_pressed(Key::D) {
            Movement::move_object(square.clone(), Vector3::new(1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if state.is_key_pressed(Key::Q) {
            Movement::rotate_object(square.clone(), rotation_speed*delta_time);
        }
        if state.is_key_pressed(Key::E) {
            Movement::rotate_object(square.clone(), -rotation_speed*delta_time);
        }

        //spin this object for testing
        if let Some(object_2) = master_graphics_list.get_object("testscene_obj1") {
            let mut object_2_read = object_2.write().unwrap(); // Read the `newsquare` object
            let rotfactor = object_2_read.get_rotation()+1.0*delta_time;
            object_2_read.set_rotation(rotfactor);
        } else {
            println!("No object found with name testscene_obj1.");
        }

        // Call the collision checking method
        let collision_events = Collision::check_collisions(&master_graphics_list, "debug_playersquare");

        // Check the collision documentation if the output seems confusing
        for event in collision_events {
            println!("Collision detected between Object ID {} and Object ID {}", event.object_name_1, event.object_name_2);
        }


    }

    /// We'll probably be using some special loading logic for the first loop but if you'd rather make things some other way you can delete this function. This is still part of the example.
    pub fn first_loop(&mut self, texture_manager: Arc<RwLock<TextureManager>>, scene_manager: &mut SceneManager, master_graphics_list: &mut MasterGraphicsList) {
        // load the texture files and the scenes from their respective directories into memory
        let _ = texture_manager.write().unwrap().load_textures_from_directory("src\\resources\\textures");
        let _ = scene_manager.load_scenes_from_directory("src\\resources\\scenes", &texture_manager.read().unwrap());

        // load the test scene from memory into the master graphics list
        if let Some(scene) = scene_manager.get_scene("testscene") {
            let scene = scene.write().expect("Failed to lock the scene for writing");
            master_graphics_list.load_scene(&scene);
        } else {
            println!("Scene 'testscene' not found");
        }

        // flag the first loop as performed.
        self.first_loop=false;
    }
}