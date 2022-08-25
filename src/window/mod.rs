use core::option::Option;
use std::os::raw::c_int;
use std::ptr::null_mut;

use gl::types::GLfloat;
use glfw::ffi::{glfwCreateWindow, glfwGetProcAddress, glfwMakeContextCurrent, glfwPollEvents, glfwSetFramebufferSizeCallback, glfwSwapInterval, GLFWwindow, glfwWindowShouldClose};
use glfw::with_c_str;

use fps_timer::FPSTimer;

use crate::renderable::Renderable;
use crate::shader::ShaderProgram;
use crate::util::string_to_c_string;

mod fps_timer;
mod glfw_initializer;

pub struct Window {
    pub glfw_window: *mut GLFWwindow,
    pub max_fps: Option<f32>,
}

pub struct WindowHandle<'a> {
    pub window: &'a mut Window,
    pub dtime: u64,
}

pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    fullscreen: bool,
    max_fps: Option<f32>,
    vsync: bool,
}


impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            width: 400,
            height: 300,
            title: "".to_string(),
            fullscreen: false,
            max_fps: None,
            vsync: false,
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        return self;
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        return self;
    }
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        return self;
    }
    pub fn max_fps(mut self, max_fps: f32) -> Self {
        self.max_fps = Some(max_fps);
        return self;
    }
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        return self;
    }

    pub fn create(self) -> Option<Window> {
        glfw_initializer::init();

        let glfw_window = unsafe {
            glfwCreateWindow(self.width as c_int,
                             self.height as c_int,
                             string_to_c_string(self.title).as_ptr(),
                             null_mut(),
                             null_mut())
        };

        if glfw_window.is_null() {
            return None;
        }

        // Focus window
        unsafe { glfwMakeContextCurrent(glfw_window); }

        // VSync
        unsafe { glfwSwapInterval(if self.vsync { 1 } else { 0 }); }

        // Set gl load function
        setup_gl_function_loader();

        // Setup viewport resize on window resize
        unsafe { glfwSetFramebufferSizeCallback(glfw_window, Some(update_viewport_size)); }

        let window = Window {
            glfw_window,
            max_fps: self.max_fps,
        };
        return Some(window);
    }
}

impl WindowHandle<'_> {
    pub fn render(&self, renderable: &impl Renderable, shader_program: &ShaderProgram) {
        shader_program.set();
        renderable.render();
    }
}

impl Window {
    pub fn run<F>(mut self, mut f: F)
        where F: FnMut(&mut WindowHandle) {
        // In micros
        let mut fps_timer = FPSTimer::new(self.max_fps);

        while unsafe { glfwWindowShouldClose(self.glfw_window) == glfw::ffi::FALSE } {
            let dtime = fps_timer.frame();

            unsafe { glfwPollEvents(); }

            gl_clear();

            let mut h = WindowHandle {
                window: &mut self,
                dtime,
            };
            f(&mut h);

            unsafe { glfw::ffi::glfwSwapBuffers(self.glfw_window); }
        }
    }
}


fn gl_clear() {
    unsafe {
        gl::ClearColor(0 as GLfloat, 0 as GLfloat, 0 as GLfloat, 1 as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn setup_gl_function_loader() {
    gl::load_with(|name| {
        with_c_str(name, |c_name| unsafe {
            glfwGetProcAddress(c_name)
        })
    });
}

extern "C" fn update_viewport_size(_window: *mut GLFWwindow, width: c_int, height: c_int) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}