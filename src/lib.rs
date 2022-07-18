extern crate core;

use std::mem;
use std::sync::mpsc::Receiver;

use gl::types::{GLfloat, GLint, GLsizei};
use glfw::{Action, Context, flush_messages, Glfw, Key, WindowEvent, WindowMode};
use glfw::ffi::GLFWwindow;
use glfw::WindowMode::{FullScreen, Windowed};

use crate::opengl::AttributeType::FLOAT;
use crate::opengl::VertexArrayObject;
use crate::render::Renderable;
use crate::shader::{ShaderProgram};

mod glfw_holder;
pub mod render;
pub mod shader;
mod opengl;

pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    fullscreen: bool,
}

pub struct Window {
    pub glfw: &'static mut Glfw,
    pub glfw_window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

pub struct WindowHandle<'a> {
    pub window: &'a mut Window,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            width: 400,
            height: 300,
            title: "".to_string(),
            fullscreen: false,
        }
    }

    pub fn size(mut self, width: u32, height: u32) -> WindowBuilder {
        self.width = width;
        self.height = height;
        return self;
    }
    pub fn title(mut self, title: String) -> WindowBuilder {
        self.title = title;
        return self;
    }
    pub fn fullscreen(mut self, fullscreen: bool) -> WindowBuilder {
        self.fullscreen = fullscreen;
        return self;
    }

    pub fn create(self) -> Window {
        return create_window(&self);
    }
}

impl Window {
    pub fn run<F>(mut self, mut f: F)
        where F: FnMut(&mut WindowHandle)
    {
        while !self.glfw_window.should_close() {
            self.glfw.poll_events();
            for (_, event) in flush_messages(&self.events) {
                handle_window_event(&mut self.glfw_window, event);
            }
            gl_clear(self.glfw_window.window_ptr());

            let mut h = WindowHandle {
                window: &mut self,
            };
            f(&mut h);
            unsafe { glfw::ffi::glfwSwapBuffers(self.glfw_window.window_ptr()); }
        }
    }

    pub(crate) fn draw_frame(&mut self) {
        todo!("");
    }
}

impl WindowHandle<'_> {
    pub fn render(&self, renderable: &impl Renderable, shader_program: &ShaderProgram) {
        shader_program.set();
        renderable.render();
    }
}

extern "C" fn update_size(window: *mut GLFWwindow, width: GLint, height: GLint) {
    unsafe {
        gl::Viewport(0, 0, width, height);
        // let w: *mut Window = glfw::ffi::glfwGetWindowUserPointer(window) as *mut Window;
    }
}

fn create_window(window_builder: &WindowBuilder) -> Window {
    let glfw_instance = glfw_holder::get();

    let create_window_lambda = |glfw: &Glfw, window_mode: WindowMode| {
        glfw.create_window(window_builder.width, window_builder.height, window_builder.title.as_str(), window_mode)
            .expect("[Bowl] Failed to create GLFW window.")
    };


    let (mut window, events) =
        if window_builder.fullscreen {
            glfw_instance.with_primary_monitor(|glfw, monitor| {
                create_window_lambda(glfw, monitor.map_or(Windowed, |m| FullScreen(m)))
            })
        } else {
            create_window_lambda(&glfw_instance, Windowed)
        };

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    unsafe {
        glfw::ffi::glfwSetFramebufferSizeCallback(window.window_ptr(), Some(update_size));
    }

    window.make_current();

    gl::load_with(|s| glfw_instance.get_proc_address_raw(s));

    unsafe { gl::Viewport(0, 0, window_builder.width as GLsizei, window_builder.height as GLsizei); }
    let w = Window {
        glfw: glfw_instance,
        glfw_window: window,
        events,
    };

    // unsafe { glfw::ffi::glfwSetWindowUserPointer(w.glfw_window.window_ptr(), mem::transmute(&w)); }
    return w;
}

fn gl_clear(window: *mut GLFWwindow) {
    unsafe {
        gl::ClearColor(0 as GLfloat, 0 as GLfloat, 0 as GLfloat, 1 as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height); }
        }
        _ => {}
    }
}