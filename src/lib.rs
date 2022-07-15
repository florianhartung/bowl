use std::sync::mpsc::Receiver;

use glfw::{Action, Context, flush_messages, Glfw, Key, WindowEvent, WindowMode};
use glfw::WindowMode::{FullScreen, Windowed};

mod glfw_holder;

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


fn create_window(window_builder: &WindowBuilder) -> Window {
    let glfw = glfw_holder::get();

    let create_window_lambda = |glfw: &Glfw, window_mode: WindowMode| {
        glfw.create_window(window_builder.width, window_builder.height, window_builder.title.as_str(), window_mode)
            .expect("[Bowl] Failed to create GLFW window.")
    };


    let (mut window, events) =
        if window_builder.fullscreen {
            glfw.with_primary_monitor(|glfw, monitor| {
                create_window_lambda(glfw, monitor.map_or(Windowed, |m| FullScreen(m)))
            })
        } else {
            create_window_lambda(&glfw, Windowed)
        };

    window.set_key_polling(true);
    window.make_current();

    return Window {
        glfw,
        glfw_window: window,
        events,
    };
}

impl Window {
    pub fn run<F>(mut self, f: F)
        where F: Fn(&mut WindowHandle)
    {
        while !self.glfw_window.should_close() {
            let mut h = WindowHandle {
                window: &mut self,
            };
            f(&mut h);

            self.glfw.poll_events();
            for (_, event) in flush_messages(&self.events) {
                handle_window_event(&mut self.glfw_window, event);
            }
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}