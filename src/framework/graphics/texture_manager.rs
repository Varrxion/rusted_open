use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
use gl::types::{GLint, GLsizei, GLuint};
use image::{self, GenericImageView}; // Ensure you have this crate in your Cargo.toml

pub struct TextureManager {
    textures: RwLock<HashMap<String, GLuint>>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: RwLock::new(HashMap::new()),
        }
    }

    pub fn load_texture(&self, name: &str, path: &str) -> Result<GLuint, String> {
        let mut textures = self.textures.write().unwrap();
        
        // Check if texture is already loaded
        if let Some(&texture_id) = textures.get(name) {
            return Ok(texture_id); // Return existing texture ID
        }

        // Load the texture and store it
        match Self::load_texture_from_file(path) {
            Ok(texture_id) => {
                textures.insert(name.to_string(), texture_id);
                Ok(texture_id) // Return the newly loaded texture ID
            },
            Err(e) => Err(e), // Pass the error up
        }
    }

    fn load_texture_from_file(path: &str) -> Result<GLuint, String> {
        let img = image::open(path).map_err(|_| "Failed to load texture".to_string())?;
        let data = img.to_rgba8();
        let (width, height) = img.dimensions();
    
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);  // Generate texture ID
            gl::BindTexture(gl::TEXTURE_2D, texture);  // Bind texture

            // Upload the texture data
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::GenerateMipmap(gl::TEXTURE_2D);  // Generate mipmaps
            gl::BindTexture(gl::TEXTURE_2D, 0);  // Unbind the texture
        }

        Ok(texture) // Return the texture ID
    }

    pub fn get_texture_id(&self, name: &str) -> Option<GLuint> {
        let textures = self.textures.read().unwrap();
        textures.get(name).copied() // Return the texture ID if it exists
    }

    // New method to load all textures from a specified directory
    pub fn load_textures_from_directory(&self, dir_path: &str) -> Result<(), String> {
        let paths = fs::read_dir(dir_path).map_err(|_| "Failed to read directory".to_string())?;

        for path in paths {
            let entry = path.map_err(|_| "Failed to read directory entry".to_string())?;
            let file_name = entry.file_name().into_string().map_err(|_| "Invalid file name".to_string())?;
            let full_path = entry.path();

            // Only load image files (you may want to check for specific extensions)
            if full_path.is_file() {
                if let Some(extension) = full_path.extension() {
                    if extension == "png" || extension == "jpg" || extension == "jpeg" {
                        // Load the texture with the file name (without extension)
                        let name = file_name.trim_end_matches(".png").trim_end_matches(".jpg").trim_end_matches(".jpeg");
                        self.load_texture(name, full_path.to_str().unwrap()).map_err(|e| format!("Error loading texture '{}': {}", name, e))?;
                    }
                }
            }
        }

        Ok(())
    }
}
