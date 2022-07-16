extern crate core;

use std::mem;
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::mpsc::Receiver;

use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr};
use glfw::{Action, Context, flush_messages, Glfw, Key, WindowEvent, WindowMode};
use glfw::ffi::GLFWwindow;
use glfw::WindowMode::{FullScreen, Windowed};

use crate::render::Shader;
use crate::render::ShaderType::{FRAGMENT, VERTEX};

mod glfw_holder;
pub mod render;

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
    pub fn run<F>(mut self, f: F)
        where F: Fn(&mut WindowHandle)
    {
        while !self.glfw_window.should_close() {
            self.glfw.poll_events();
            for (_, event) in flush_messages(&self.events) {
                handle_window_event(&mut self.glfw_window, event);
            }

            let mut h = WindowHandle {
                window: &mut self,
            };
            f(&mut h);

            gl_draw(self.glfw_window.window_ptr());
        }
    }
}

extern "C" fn update_size(window: *mut GLFWwindow, width: GLint, height: GLint) {
    unsafe {
        gl::Viewport(0, 0, width, height);
        gl_draw(window);
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
    let mut vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    gl::load_with(|s| glfw_instance.get_proc_address_raw(s));
    unsafe {
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);

        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&vertices) as GLsizeiptr, &mut vertices as *mut _ as *mut c_void, gl::STATIC_DRAW);


        let shaders = vec![
            Shader::from(include_str!("shader.vert"), VERTEX),
            Shader::from(include_str!("shader.frag"), FRAGMENT),
        ];
        let shader_program = render::create_shader_program(shaders);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, 3 * 4, null());
        gl::EnableVertexAttribArray(0);


        gl::UseProgram(shader_program);
        gl_draw(window.window_ptr());
    }

    unsafe { gl::Viewport(0, 0, window_builder.width as GLsizei, window_builder.height as GLsizei); }
    return Window {
        glfw: glfw_instance,
        glfw_window: window,
        events,
    };
}

fn gl_draw(window: *mut GLFWwindow) {
    unsafe {
        gl::ClearColor(255 as GLfloat, 0 as GLfloat, 0 as GLfloat, 1 as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        glfw::ffi::glfwSwapBuffers(window);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height); }
            gl_draw(window.window_ptr());
        }
        _ => {}
    }
}