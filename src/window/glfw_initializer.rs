use glfw::ffi;
use glfw::ffi::{glfwInit, glfwWindowHint};

static mut INITIALIZED: bool = false;

pub fn init() {
    unsafe {
        if !INITIALIZED {
            if glfwInit() == ffi::FALSE {
                panic!("Could not initialize glfw");
            }
            glfwWindowHint(ffi::CONTEXT_VERSION_MAJOR, 3);
            glfwWindowHint(ffi::CONTEXT_VERSION_MINOR, 3);
            glfwWindowHint(ffi::OPENGL_PROFILE, ffi::OPENGL_CORE_PROFILE);
        }
    }
}