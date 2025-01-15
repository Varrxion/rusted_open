use std::sync::{Arc, RwLock};

use glfw::Key;
use nalgebra::Vector3;

use crate::engine::{engine_controller::EngineController, graphics::{texture_manager::TextureManager, util::{master_clock::MasterClock, master_graphics_list::MasterGraphicsList}}, key_states::KeyStates, scenes::scene_manager::SceneManager};

use super::events::{collision::Collision, movement::Movement};

pub struct ApiEntryPoint {
    engine_controller: EngineController,
}

impl ApiEntryPoint {
    /// Creates a new EntryPoint instance.
    pub fn new() -> Self {
        ApiEntryPoint {
            engine_controller: EngineController::new(),
        }
    }

    /// This is what we call from main to start the API. It will grab the engine parts we want to use, call some setup, and then call our game loop until the app closes.
    /// It does not have to be made this way, any structure can be used but this makes the most sense to me so we will follow it for this example code.
    /// Direct changes may be made to the engine itself if needed but the API is the "developer-friendly" way to work with the engine.
    pub fn init(&mut self) {
        // Grab the parts of the engine_controller you want to use
        let master_clock = self.engine_controller.get_master_clock();
        let texture_manager = self.engine_controller.get_texture_manager();
        let scene_manager = self.engine_controller.get_scene_manager();
        let master_graphics_list = self.engine_controller.get_master_graphics_list();
        let key_states = self.engine_controller.get_key_states();

        let mut flag = false;

        self.load_resources(&texture_manager.write().unwrap(), &mut scene_manager.write().unwrap(), &master_graphics_list.write().unwrap());

        while flag == false {
            flag = self.main_loop(&master_clock, &master_graphics_list, &key_states);
        }
    }

    /// This is the main loop for the framework.
    /// I have included a simple control scheme for the object we control, a random spinning object, and two stationary objects.
    pub fn main_loop(&mut self, master_clock: &Arc<RwLock<MasterClock>>, master_graphics_list: &Arc<RwLock<MasterGraphicsList>>, key_states: &Arc<RwLock<KeyStates>>) -> bool {
        // Retrieve the "player" square from the master graphics list
        let square = master_graphics_list.read().unwrap().get_object("testscene_playersquare").expect("Object not found");

        // Thou shalt not use frame-based physics
        let delta_time = master_clock.read().unwrap().get_delta_time();

        // Apply movement based on active keys (This should really be abstracted somewhere else, but it's here for simplicity. You can move this kind of logic wherever you want.)
        let move_speed = 0.2;
        let rotation_speed = 2.0;
        if key_states.read().unwrap().is_key_pressed(Key::W) {
            Movement::move_object(square.clone(), Vector3::new(0.0, 1.0, 0.0), move_speed, delta_time);
        }
        if key_states.read().unwrap().is_key_pressed(Key::S) {
            Movement::move_object(square.clone(), Vector3::new(0.0, -1.0, 0.0), move_speed, delta_time);
        }
        if key_states.read().unwrap().is_key_pressed(Key::A) {
            Movement::move_object(square.clone(), Vector3::new(-1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if key_states.read().unwrap().is_key_pressed(Key::D) {
            Movement::move_object(square.clone(), Vector3::new(1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if key_states.read().unwrap().is_key_pressed(Key::Q) {
            Movement::rotate_object(square.clone(), rotation_speed*delta_time);
        }
        if key_states.read().unwrap().is_key_pressed(Key::E) {
            Movement::rotate_object(square.clone(), -rotation_speed*delta_time);
        }

        // Spin this object for testing
        if let Some(object_2) = master_graphics_list.read().unwrap().get_object("testscene_obj1") {
            let mut object_2_read = object_2.write().unwrap();
            let rotfactor = object_2_read.get_rotation()+1.0*delta_time;
            object_2_read.set_rotation(rotfactor);
        } else {
            println!("No object found with name testscene_obj1.");
        }

        // Call the collision checking method
        let collision_events = Collision::check_collisions(&master_graphics_list.read().unwrap(), "testscene_playersquare");

        // Check the collision documentation if the output seems confusing
        for event in collision_events {
            println!("Collision detected between Object ID {} and Object ID {}", event.object_name_1, event.object_name_2);
        }

        return self.engine_controller.execute_tick();

    }

    /// Here we will load the json scene configs (basically level files), and load the test scene into the master graphics list.
    pub fn load_resources(&mut self, texture_manager: &TextureManager, scene_manager: &mut SceneManager, master_graphics_list: &MasterGraphicsList) {
        self.engine_controller.set_resolution(1280.0, 720.0);

        // load the texture files and the scenes from their respective directories into memory
        let _ = texture_manager.load_textures_from_directory("src\\resources\\textures");
        let _ = scene_manager.load_scenes_from_directory("src\\resources\\scenes", &texture_manager);

        // load the test scene from memory into the master graphics list
        if let Some(scene) = scene_manager.get_scene("testscene") {
            let scene = scene.write().expect("Failed to lock the scene for writing");
            master_graphics_list.load_scene(&scene);
        } else {
            println!("Scene 'testscene' not found");
        }
    }
}