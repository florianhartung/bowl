use std::collections::HashMap;

use gl::types::GLuint;
use once_cell::sync::OnceCell;

use crate::render;

static mut SHADERS: OnceCell<HashMap<String, Shader>> = OnceCell::new();

pub fn get(key: &str) -> &'static Shader {
    unsafe {
        try_init_shaders_holder();
        return SHADERS.get().unwrap().get(key).expect(format!("[Bowl] Could not find shader by key {}", key).as_str());
    }
}

unsafe fn try_init_shaders_holder() {
    if SHADERS.get().is_none() {
        SHADERS.set(HashMap::new()).ok().unwrap();
    }
}

pub fn register(key: &str, src: &str, r#type: ShaderType) {
    let mut shader = Shader {
        r#type,
        glfw_shader: 0,
        key: key.to_string(),
    };

    shader.glfw_shader = render::compile_shader(src, render::map_shader_type_to_glfw(&shader.r#type));

    unsafe {
        try_init_shaders_holder();
        SHADERS.get_mut().unwrap().insert(shader.key.clone(), shader);
    }
}

#[macro_export]
macro_rules! register_program {
    ($( $key:literal),* ) => {
        {
            let mut temp_program = $crate::shader::ShaderProgram {
                shaders: Vec::new(),
                glfw_program: 0,
            };

            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($key));
            )*
            // temp_program.shaders = temp_vec;
            temp_program.glfw_program = $crate::render::create_shader_program(&temp_vec);

            temp_program
        }
    };
}

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub struct Shader {
    pub r#type: ShaderType,
    pub glfw_shader: GLuint,
    pub key: String,
}

pub struct ShaderProgram {
    pub shaders: Vec<String>,
    pub glfw_program: GLuint,
}

impl ShaderProgram {
    pub fn set(&self) {
        unsafe {
            gl::UseProgram(self.glfw_program);
        }
    }
}