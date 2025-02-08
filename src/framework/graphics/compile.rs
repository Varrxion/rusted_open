use gl::types::GLenum;
use gl::types::GLuint;
use gl::types::GLint;
use gl::types::GLchar;
use std::ffi::CString;

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = GLint::default();
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(511); // Reserve space for null terminator
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "Shader compilation failed: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        shader
    }
}

pub fn create_shader_program(vertex_src: &str, fragment_src: &str) -> GLuint {
    unsafe {
        let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut success = GLint::default();
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(511); // Reserve space for null terminator
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "Shader program linking failed: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        // Clean up shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}
