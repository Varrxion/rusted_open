use std::sync::{Arc, RwLock};

use glfw::Context;
use nalgebra::{Matrix4, Vector3};

use crate::framework::graphics;

use super::graphics::{camera::Camera, texture_manager::TextureManager, util::master_graphics_list::MasterGraphicsList};

pub struct FrameworkController {
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    projection_matrix: Matrix4<f32>,
    texture_manager: Arc<RwLock<TextureManager>>,
    camera: Camera,
    width: f32,
    height: f32,
}

impl FrameworkController {
    pub fn new() -> Self {
        // There is something holy about this resolution
        let width: f32 = 640.0; 
        let height: f32 = 480.0;
        // Set up the projection matrix once
        let projection_matrix = Self::init_projection_matrix(width, height);

        // Load OpenGL functions
        graphics::glfw::load_gl_symbols();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearDepth(1.0);
        }

        Self {
            master_graphics_list: Arc::new(RwLock::new(MasterGraphicsList::new())),
            projection_matrix,
            texture_manager: Arc::new(RwLock::new(TextureManager::new())),
            camera: Camera::new(0.1),
            width,
            height,
        }
    }

    fn calculate_projection_matrix(width: f32, height: f32, camera_position: &Vector3<f32>) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        
        // Create an orthogonal projection matrix
        let projection = Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0);
        
        // Create a view matrix that translates the world by the negative camera position
        let translation = Matrix4::new_translation(&Vector3::new(-camera_position.x, -camera_position.y, 0.0));
        
        let scale = Matrix4::new_scaling(camera_position.z); // Higher zoom = see less. Lower zoom = see more.

        // Combine the projection and view matrices, then scale to apply zoom
        projection * scale * translation
    }

    fn init_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0)
    }
    

    /// Sets the resolution of the openGL viewport and updates the projection matrix
    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.projection_matrix = Self::calculate_projection_matrix(width, height, &self.camera.get_position());
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);  // Update the OpenGL viewport
        }
    }

    /// Returns true if the window should close
    pub fn render(&mut self, window: &mut glfw::PWindow, delta_time: f32) {
        // Update the camera and projection
        self.camera.update_position(&self.master_graphics_list.read().unwrap());
        self.projection_matrix = Self::calculate_projection_matrix(self.width, self.height, &self.camera.get_position());

        // Render here
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);    // Clear the screen + depth buffer
        }

        // Draw
        self.master_graphics_list.write().unwrap().draw_all(&self.projection_matrix, delta_time);

        // Swap buffers
        window.swap_buffers();
    }

    pub fn shutdown(&mut self) {
        self.master_graphics_list.write().unwrap().remove_all();
    }

    pub fn get_texture_manager(&mut self) -> Arc<RwLock<TextureManager>> {
        return self.texture_manager.clone();
    }

    pub fn get_master_graphics_list(&mut self) -> Arc<RwLock<MasterGraphicsList>> {
        return self.master_graphics_list.clone();
    }

    pub fn set_camera_tracking_target(&mut self, target_name: String) {
        self.camera.set_tracking_target(Some(target_name));
    }

    pub fn set_camera_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
    }
}