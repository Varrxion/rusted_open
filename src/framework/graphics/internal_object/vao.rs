use gl::types::{GLint, GLuint};

pub struct VAO {
    id: GLuint, // Stores the VAO ID generated by OpenGL
    texture_id: Option<GLuint>, // Optional texture ID associated with this VAO
}

impl VAO {
    /// Creates a new Vertex Array Object.
    pub fn new() -> Self {
        let mut vao: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        Self {
            id: vao,
            texture_id: None, // No texture associated initially
        }
    }

    /// Binds the VAO for use (this makes the array active).
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            if let Some(texture_id) = self.texture_id {
                gl::BindTexture(gl::TEXTURE_2D, texture_id); // Bind the texture if present
            }
        }
    }

    /// Unbinds any active VAO.
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0); // Unbind texture
        }
    }

    pub fn setup_vertex_attributes(&mut self, vbo_ids: Vec<(GLuint, GLint, GLuint)>, texture_id: Option<GLuint>) {
        self.texture_id = texture_id; // Store the texture ID

        self.bind();
    
        for (vbo_id, size, index) in vbo_ids {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
                gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
                gl::EnableVertexAttribArray(index);
            }
        }
    
        // Unbind the VBO and VAO
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        VAO::unbind();
    }
}

impl Drop for VAO {
    /// Clean up the VAO when it's no longer needed (automatically called by Rust).
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
