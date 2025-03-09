use gl::types::GLuint;
use nalgebra::{Matrix4, Vector3};
use std::{ffi::CString, sync::{Arc, RwLock}};
use super::{animation::{backward_animation, forward_animation, random_animation}, animation_config::AnimationConfig, atlas_config::AtlasConfig, vao::VAO, vbo::VBO};

pub struct Generic2DGraphicsObject {
    name: String,
    vertex_data: Vec<f32>,
    texture_coords: Vec<f32>,
    vao: Arc<RwLock<VAO>>,
    position_vbo: Arc<VBO>, // VBO for positions
    tex_vbo: Arc<RwLock<VBO>>, // VBO for texture coordinates
    shader_program: GLuint,
    position: nalgebra::Vector3<f32>,
    rotation: f32,
    scale: f32,
    model_matrix: Matrix4<f32>,
    atlas_config: Option<AtlasConfig>,
    animation_config: Option<AnimationConfig>,
    elapsed_time: f32,
}

impl Clone for Generic2DGraphicsObject {
    fn clone(&self) -> Self {
        Generic2DGraphicsObject {
            name: self.name.clone(),
            vertex_data: self.vertex_data.clone(),
            texture_coords: self.texture_coords.clone(),
            vao: Arc::clone(&self.vao),
            position_vbo: Arc::clone(&self.position_vbo),
            tex_vbo: Arc::clone(&self.tex_vbo),
            shader_program: self.shader_program,
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
            model_matrix: self.model_matrix,
            atlas_config: self.atlas_config.clone(),
            animation_config: self.animation_config.clone(),
            elapsed_time: self.elapsed_time,
        }
    }
}

impl Generic2DGraphicsObject {
    const FULL_ROTATION: f32 = 2.0 * std::f32::consts::PI; // 360 degrees in radians

    pub fn new(
        name: String,
        vertex_data: Vec<f32>,
        texture_coords: Vec<f32>,
        shader_program: GLuint,
        position: Vector3<f32>,
        rotation: f32,
        scale: f32,
        texture_id: Option<GLuint>,
        atlas_config: Option<AtlasConfig>,
        animation_config: Option<AnimationConfig>,
    ) -> Self {
        let mut object = Self {
            name,
            vertex_data,
            texture_coords,
            vao: Arc::new(RwLock::new(VAO::new())), // Create a new VAO wrapped in RwLock
            position_vbo: Arc::new(VBO::new(&[])), // Placeholder for position VBO
            tex_vbo: Arc::new(RwLock::new(VBO::new(&[]))), // Placeholder for texture VBO
            shader_program,
            position,
            rotation,
            scale,
            model_matrix: Matrix4::identity(), // Identity matrix for 2D
            atlas_config,
            animation_config,
            elapsed_time: 0.0,
        };
        object.initialize(texture_id); // Pass texture ID to initialize
        object
    }

    fn initialize(&mut self, texture_id: Option<GLuint>) {
        // Ensure the shader program is active before interacting with any attributes or uniforms
        unsafe {
            gl::UseProgram(self.shader_program);
        }

        let mut vao = self.vao.write().unwrap(); // Lock the RwLock for mutable access
        // Bind the VAO
        vao.bind();

        // Initialize the VBOs with vertex data and texture coordinates
        self.position_vbo = Arc::new(VBO::new(&self.vertex_data)); // Initialize position VBO
        self.tex_vbo = Arc::new(RwLock::new(VBO::new(&self.texture_coords))); // Initialize texture VBO

        // Setup vertex attributes for the VAO
        vao.setup_vertex_attributes(vec![
            (self.position_vbo.id(), 2, 0), // Position VBO
            (self.tex_vbo.read().unwrap().id(), 2, 1),       // Texture coordinate VBO
        ], texture_id); // Pass texture ID dynamically

        if let Some(atlas_config) = &self.atlas_config {
            self.initilize_animation_properties(&atlas_config);
        }

        // Unbind the VAO
        VAO::unbind();
    }

    // Apply translation, rotation, and scale as a combined transform
    pub fn update_model_matrix(&mut self) {
        let translation_matrix = Matrix4::new_translation(&self.position);
        let rotation_matrix = Matrix4::new_rotation(Vector3::z() * self.rotation);
        let scale_matrix = Matrix4::new_scaling(self.scale);

        self.model_matrix = translation_matrix * rotation_matrix * scale_matrix; // Combine transformations
    }

    pub fn apply_transform(&self, projection_matrix: &Matrix4<f32>) {
        unsafe {
            // Use the shader program
            gl::UseProgram(self.shader_program);

            // Set the projection matrix
            let projection_location = gl::GetUniformLocation(self.shader_program, CString::new("projection").unwrap().as_ptr());
            let projection_array: [f32; 16] = projection_matrix.as_slice().try_into().expect("Matrix conversion failed");
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_array.as_ptr());

            // Set the model matrix
            let model_location = gl::GetUniformLocation(self.shader_program, CString::new("model").unwrap().as_ptr());
            let model_array: [f32; 16] = self.model_matrix.as_slice().try_into().expect("Matrix conversion failed");
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_array.as_ptr());
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            let vao = self.vao.read().unwrap(); // Lock the RwLock for read access
            vao.bind();
            // Draw elements based on the number of vertices
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, (self.vertex_data.len() / 2) as i32);
            VAO::unbind();
        }
    }

    // Method to calculate width and height based on vertex data
    pub fn dimensions(&self) -> (f32, f32) {
        let min_x = self.vertex_data.iter()
            .step_by(2) // Take x-coordinates
            .cloned()
            .fold(f32::INFINITY, f32::min);
        
        let max_x = self.vertex_data.iter()
            .step_by(2) // Take x-coordinates
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        
        let min_y = self.vertex_data.iter()
            .skip(1) // Take y-coordinates
            .step_by(2) // Skip every other (x)
            .cloned()
            .fold(f32::INFINITY, f32::min);
        
        let max_y = self.vertex_data.iter()
            .skip(1) // Take y-coordinates
            .step_by(2) // Skip every other (x)
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        
        let width = (max_x - min_x) * self.scale;
        let height = (max_y - min_y) * self.scale;
        
        (width, height)
    }

    pub fn initilize_animation_properties(&self, atlas_config: &AtlasConfig) {
        unsafe {    
            // Get the uniform location for number of columns in the atlas
            let atlas_columns_location = gl::GetUniformLocation(self.shader_program, CString::new("atlasColumns").unwrap().as_ptr());
            if atlas_columns_location == -1 {
                println!("Error: uniform 'atlasColumns' not found in shader!");
            } else {
                gl::Uniform1f(atlas_columns_location, atlas_config.atlas_columns as f32);
            }
    
            // Get the uniform location for number of rows in the atlas
            let atlas_rows_location = gl::GetUniformLocation(self.shader_program, CString::new("atlasRows").unwrap().as_ptr());
            if atlas_rows_location == -1 {
                println!("Error: uniform 'atlasRows' not found in shader!");
            } else {
                gl::Uniform1f(atlas_rows_location, atlas_config.atlas_rows as f32);
            }

            // For animation debugging
            //println!("Set atlasColumns to {}, atlasRows to {}.", self.atlas_columns, self.atlas_rows);
        }
    }
    

    // Update method to handle animation logic
    pub fn update_animation(&mut self, delta_time: f32) {
        if let Some(atlas_config) = &mut self.atlas_config {
            if let Some(animation_config) = &self.animation_config {
                if animation_config.frame_duration != 0.0 {
                    self.elapsed_time += delta_time;
        
                    let frame_advance = (self.elapsed_time / animation_config.frame_duration).floor() as usize;
        
                    if frame_advance > 0 {
                        self.elapsed_time %= animation_config.frame_duration;
        
                        atlas_config.current_frame = match animation_config.mode.as_str() {
                            "forward" => forward_animation(frame_advance, atlas_config, animation_config),
                            "backward" => backward_animation(frame_advance, atlas_config, animation_config),
                            "random" => random_animation(&animation_config),
                            _ => atlas_config.current_frame, // No animation or unrecognized mode
                        };
                    }
                }
            }
            self.update_texture_coords();
        }
    }

    // Update texture coordinates based on the current frame
    pub fn update_texture_coords(&mut self) {
        if let Some(atlas_config) = &mut self.atlas_config {
            // Calculate the current frame's position in the atlas (grid)
            let frame_x = (atlas_config.current_frame % atlas_config.atlas_columns) as f32;
            let frame_y = (atlas_config.current_frame / atlas_config.atlas_columns) as f32;

            // Calculate texture coordinates for the frame
            let u1 = frame_x;
            let v1 = frame_y;
            let u2 = u1 + 1.0;
            let v2 = v1 + 1.0;

            // Update the texture coordinates for the current frame
            self.texture_coords = vec![
                u2, v1,
                u2, v2,
                u1, v2,
                u1, v1,
            ];

            // For animation debugging
            //println!("Current Frame: {}, Current texture_coords to be passed into VBO:\n {}, {},\n {}, {},\n {}, {},\n {}, {}", self.current_frame,u2,v1,u2,v2,u1,v2,u1,v1);

            // Now update the texture VBO with the new texture coordinates
            self.update_texture_vbo();
        }
    }

    fn update_texture_vbo(&mut self) {
        let mut tex_vbo = self.tex_vbo.write().unwrap();
        tex_vbo.update_data(&self.texture_coords);
    }

    pub fn get_radius(&self) -> f32 {
        self.vertex_data
            .chunks(2)
            .map(|v| (v[0].powi(2) + v[1].powi(2)).sqrt() * self.scale)
            .fold(0.0, f32::max)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_atlas_config(&self) -> Option<AtlasConfig> {
        self.atlas_config.clone()
    }

    pub fn get_animation_config(&self) -> Option<AnimationConfig> {
        self.animation_config.clone()
    }

    pub fn set_atlas_config(&mut self, atlas_config: Option<AtlasConfig>) {
        self.atlas_config = atlas_config;
    }

    pub fn set_animation_config(&mut self, animation_config: Option<AnimationConfig>) {
        self.animation_config = animation_config;
    }

    pub fn set_position(&mut self, position: nalgebra::Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation % Self::FULL_ROTATION;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }

    pub fn get_position(&self) -> nalgebra::Vector3<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn print_debug(&self) {
        println!("Debug Info for Generic2DGraphicsObject:");
        println!("Name: {}", self.name);
        println!("Vertex Data: {:?}", self.vertex_data);
        println!("Texture Coordinates: {:?}", self.texture_coords);
        println!("Shader Program: {}", self.shader_program);
        println!("Position: {:?}", self.position);
        println!("Rotation: {}", self.rotation);
        println!("Scale: {}", self.scale);
        println!("Model Matrix: {:?}", self.model_matrix);
        println!("Position VBO ID: {}", self.position_vbo.id());
        println!("Texture VBO ID: {}\n", self.tex_vbo.read().unwrap().id());
    }
}