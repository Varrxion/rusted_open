use std::sync::{Arc, RwLock};

use glfw::Context;
use nalgebra::Matrix4;

use crate::framework::graphics;

use super::graphics::{texture_manager::TextureManager, util::{master_clock::{self, MasterClock}, master_graphics_list::MasterGraphicsList}};

pub struct FrameworkController {
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    master_clock: Arc<RwLock<master_clock::MasterClock>>,
    projection_matrix: Matrix4<f32>,
    texture_manager: Arc<RwLock<TextureManager>>,
}

impl FrameworkController {
    pub fn new() -> Self {
        // Set up the projection matrix once
        let projection_matrix = Self::calculate_projection_matrix(640 as f32, 480 as f32);

        // Load OpenGL functions
        graphics::glfw::load_gl_symbols();

        Self {
            master_graphics_list: Arc::new(RwLock::new(MasterGraphicsList::new())),
            master_clock: Arc::new(RwLock::new(MasterClock::new())),
            projection_matrix,
            texture_manager: Arc::new(RwLock::new(TextureManager::new())),
        }
    }

    fn calculate_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0)
    }

    /// Sets the resolution of the openGL viewport and updates the projection matrix
    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.projection_matrix = Self::calculate_projection_matrix(width, height);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);  // Update the OpenGL viewport
        }
    }

    /// Returns true if the window should close
    pub fn execute_tick(&mut self, mut window: glfw::PWindow) {

        // Update the clock
        self.master_clock.write().unwrap().update();

        // Render here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
            gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
        }

        // Draw
        self.master_graphics_list.write().unwrap().draw_all(&self.projection_matrix);

        // Swap buffers
        window.swap_buffers();
    }

    pub fn shutdown(&mut self) {
        self.master_graphics_list.write().unwrap().remove_all();
    }

    pub fn get_texture_manager(&mut self) -> Arc<RwLock<TextureManager>> {
        return self.texture_manager.clone();
    }

    pub fn get_master_clock(&self) -> Arc<RwLock<MasterClock>> {
        return self.master_clock.clone();
    }

    pub fn get_master_graphics_list(&mut self) -> Arc<RwLock<MasterGraphicsList>> {
        return self.master_graphics_list.clone();
    }
}