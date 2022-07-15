use glfw::{Glfw, LOG_ERRORS};
use once_cell::sync::OnceCell;

static mut GLFW_INSTANCE: OnceCell<Glfw> = OnceCell::new();

pub fn get() -> & 'static mut Glfw {
    unsafe {
        if GLFW_INSTANCE.get().is_none() {
            let glfw = glfw::init(LOG_ERRORS).expect("[Bowl] Failed to initialize Glfw");
            GLFW_INSTANCE.set(glfw).unwrap();
        }

        return GLFW_INSTANCE.get_mut().unwrap();
    }
}