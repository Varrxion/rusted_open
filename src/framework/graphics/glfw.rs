use std::ffi::CString;

pub fn load_gl_symbols() {
    gl::load_with(|s| {
        let c_str = CString::new(s).unwrap();
        unsafe { glfw::ffi::glfwGetProcAddress(c_str.as_ptr()) as *const _ }
    });
}