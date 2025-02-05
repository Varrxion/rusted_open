use std::sync::{Arc, RwLock};

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use nalgebra::Matrix4;

use crate::engine::graphics;

use super::{graphics::{texture_manager::TextureManager, util::{master_clock::{self, MasterClock}, master_graphics_list::MasterGraphicsList}}, key_states::KeyStates, scenes::scene_manager::SceneManager};

pub struct EngineController {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    master_clock: Arc<RwLock<master_clock::MasterClock>>,
    projection_matrix: Matrix4<f32>,
    texture_manager: Arc<RwLock<TextureManager>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    key_states: Arc<RwLock<KeyStates>>,
}

impl EngineController {
    pub fn new(window_name: String) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(false));

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(640 as u32, 480 as u32, &window_name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Set up the projection matrix once
        let projection_matrix = Self::calculate_projection_matrix(640 as f32, 480 as f32);

        // Make the window's context current
        window.make_current();

        // Enable key events
        window.set_key_polling(true);

        // Load OpenGL functions
        graphics::glfw::load_gl_symbols();

        Self {
            glfw,
            window,
            events,
            master_graphics_list: Arc::new(RwLock::new(MasterGraphicsList::new())),
            master_clock: Arc::new(RwLock::new(MasterClock::new())),
            projection_matrix,
            texture_manager: Arc::new(RwLock::new(TextureManager::new())),
            scene_manager: Arc::new(RwLock::new(SceneManager::new())),
            key_states: Arc::new(RwLock::new(KeyStates::new())),
        }
    }

    fn calculate_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0)
    }

    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.window.set_size(width as i32, height as i32);
        self.projection_matrix = Self::calculate_projection_matrix(width, height);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);  // Update the OpenGL viewport
        }
    }

    /// Returns true if the window should close
    pub fn execute_tick(&mut self) -> bool {

        if self.window.should_close() {
            return true;
        }

        // Update the clock
        self.master_clock.write().unwrap().update();

        // Update key events
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                _ => {
                    //Add or remove a key from the list of currently held keys based on the current user input
                    self.key_states.write().unwrap().handle_key_event(event);
                }
            }
        }

        // Render here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
            gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
        }

        // Draw
        self.master_graphics_list.write().unwrap().draw_all(&self.projection_matrix);

        // Swap buffers
        self.window.swap_buffers();

        return false;
    }

    pub fn shutdown(&mut self) {
        self.master_graphics_list.write().unwrap().remove_all();
    }

    pub fn get_texture_manager(&mut self) -> Arc<RwLock<TextureManager>> {
        return self.texture_manager.clone();
    }

    pub fn get_scene_manager(&mut self) -> Arc<RwLock<SceneManager>> {
        return self.scene_manager.clone();
    }

    pub fn get_master_clock(&self) -> Arc<RwLock<MasterClock>> {
        return self.master_clock.clone();
    }

    pub fn get_master_graphics_list(&mut self) -> Arc<RwLock<MasterGraphicsList>> {
        return self.master_graphics_list.clone();
    }

    pub fn get_key_states(&mut self) -> Arc<RwLock<KeyStates>> {
        return self.key_states.clone();
    }
}