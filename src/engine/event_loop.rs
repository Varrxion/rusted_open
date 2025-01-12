use std::sync::{Arc, RwLock};

use glfw::{Action, Context, GlfwReceiver, Key, WindowEvent};
use nalgebra::Matrix4;

use crate::{api::api_entry_point::ApiEntryPoint, engine::graphics};

use super::{graphics::{texture_manager::TextureManager, util::{master_clock, master_graphics_list::MasterGraphicsList}}, scenes::scene_manager::SceneManager, key_states::State};

pub struct EventLoop {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    master_graphics_list: MasterGraphicsList,
    master_clock: master_clock::MasterClock,
    projection_matrix: Matrix4<f32>,
    current_resolution_index: usize, // Index to track the current resolution
    resolutions: Vec<(u32, u32)>, // Vector to hold multiple resolution options
}

impl EventLoop {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        // Define multiple resolution options
        let resolutions = vec![
            (3840, 2160), // 2160p, bugged? Cannot test
            (2560, 1440), // 1440p
            (1920, 1080), // 1080p
            (1280, 720),  // 720p
            (640, 360),   // 360p
        ];

        let window_width = resolutions[3].0;
        let window_height = resolutions[3].1;

        glfw.window_hint(glfw::WindowHint::Resizable(false));

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(window_width as u32, window_height as u32, "Rusted-OpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Set up the projection matrix once
        let projection_matrix = Self::calculate_projection_matrix(window_width as f32, window_height as f32);

        // Make the window's context current
        window.make_current();

        // Enable key events
        window.set_key_polling(true);

        // Load OpenGL functions
        graphics::glfw::load_gl_symbols();

        // Initialize the master graphics list
        let master_graphics_list = MasterGraphicsList::new();

        let master_clock = master_clock::MasterClock::new();

        Self {
            glfw,
            window,
            events,
            master_graphics_list,
            master_clock,
            projection_matrix,
            current_resolution_index: 3,
            resolutions,
        }
    }

    fn calculate_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
        let aspect_ratio = width / height;
        Matrix4::new_orthographic(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, -1.0, 1.0)
    }
    
    pub fn run_event_loop(&mut self) {  
        // Create the state to manage keys and other state
        let mut state = State::new();

        let texture_manager = Arc::new(RwLock::new(TextureManager::new()));

        let mut scene_manager = SceneManager::new();

        let mut api_entry_point = ApiEntryPoint::new();
        
        while !self.window.should_close() {
            // Update the clock
            self.master_clock.update();
    
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::KpSubtract, _, Action::Press, _) => {
                        if  self.current_resolution_index<self.resolutions.capacity()-1 {
                            self.current_resolution_index += 1;
                            self.window.set_size(self.resolutions[self.current_resolution_index].0 as i32, self.resolutions[self.current_resolution_index].1 as i32);
                            self.projection_matrix = Self::calculate_projection_matrix(self.resolutions[self.current_resolution_index].0 as f32, self.resolutions[self.current_resolution_index].1 as f32);
                            unsafe {
                                gl::Viewport(0, 0, self.resolutions[self.current_resolution_index].0 as i32, self.resolutions[self.current_resolution_index].1 as i32);  // Update the OpenGL viewport
                            }
                        }
                    },
                    glfw::WindowEvent::Key(Key::KpAdd, _, Action::Press, _) => {
                        if  self.current_resolution_index>0 {
                            self.current_resolution_index -= 1;
                            self.window.set_size(self.resolutions[self.current_resolution_index].0 as i32, self.resolutions[self.current_resolution_index].1 as i32);
                            self.projection_matrix = Self::calculate_projection_matrix(self.resolutions[self.current_resolution_index].0 as f32, self.resolutions[self.current_resolution_index].1 as f32);
                            unsafe {
                                gl::Viewport(0, 0, self.resolutions[self.current_resolution_index].0 as i32, self.resolutions[self.current_resolution_index].1 as i32);  // Update the OpenGL viewport
                            }
                        }
                    },
                    _ => {
                        state.handle_key_event(event); // Handle other window events
                    }
                }
            }

            // This is the api entry point, so developers have a more abstracted view rather than working with the engine directly
            api_entry_point.entry_point(&mut self.glfw, &mut self.window, &mut self.master_clock, texture_manager.clone(), &mut scene_manager, &mut self.master_graphics_list, &mut state);

            // Render here
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0); // Set background color
                gl::Clear(gl::COLOR_BUFFER_BIT);    // Clear the screen
            }
    
            // Draw
            self.master_graphics_list.draw_all(&self.projection_matrix);
    
            // Swap buffers
            self.window.swap_buffers();
        }
        self.master_graphics_list.remove_all();
    }
}