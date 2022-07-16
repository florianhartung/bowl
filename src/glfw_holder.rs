use glfw::{Glfw, LOG_ERRORS};
use glfw::OpenGlProfileHint::Core;
use glfw::WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile};
use once_cell::sync::OnceCell;

static mut GLFW_INSTANCE: OnceCell<Glfw> = OnceCell::new();

pub fn get() -> &'static mut Glfw {
    unsafe {
        if GLFW_INSTANCE.get().is_none() {
            GLFW_INSTANCE.set(init_new()).unwrap();
        }

        return GLFW_INSTANCE.get_mut().unwrap();
    }
}

fn init_new() -> Glfw {
    let mut glfw = glfw::init(LOG_ERRORS).expect("[Bowl] Failed to initialize Glfw");
    glfw.window_hint(ContextVersionMajor(3));
    glfw.window_hint(ContextVersionMinor(3));
    glfw.window_hint(OpenGlProfile(Core));

    return glfw;
}