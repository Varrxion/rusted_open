use gl::types::GLuint;
use crate::framework::graphics::compile::create_shader_program;

pub struct CustomShader {
    shader_program: GLuint,
}


impl CustomShader {
    pub fn new(vertex_shader_src: &str, fragment_shader_src: &str) -> Self {
        let shader_program = create_shader_program(vertex_shader_src, fragment_shader_src);

        let custom_shader = CustomShader {
            shader_program,
        };
        custom_shader
    }

    // Getter for shader_program
    pub fn get_shader_program(&self) -> GLuint {
        self.shader_program
    }
}