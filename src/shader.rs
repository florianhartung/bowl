use std::collections::HashMap;
use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLsizei, GLuint};
use once_cell::sync::OnceCell;

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

pub fn new_shader(key: &str, src: &str, r#type: ShaderType) {
    let mut shader = Shader {
        r#type,
        glfw_shader: 0,
        key: key.to_string(),
    };

    shader.glfw_shader = compile_shader(src, map_shader_type_to_glfw(&shader.r#type));

    unsafe {
        try_init_shaders_holder();
        SHADERS.get_mut().unwrap().insert(shader.key.clone(), shader);
    }
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

pub(crate) fn compile_shader(src: &str, t: GLenum) -> GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(t);

        gl::ShaderSource(vertex_shader, 1, &(src.as_bytes().as_ptr().cast()), &src.len().try_into().unwrap());

        gl::CompileShader(vertex_shader);

        let mut success: i32 = 0;


        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log: [i8; 1024] = [0; 1024];
            gl::GetShaderInfoLog(vertex_shader, 1024, 0 as *mut GLsizei, log.as_mut_ptr());
            let s: String = CString::from_raw(log.as_mut_ptr()).into_string().unwrap();
            println!("[Bowl] Could not compile shader of type {}:\n{}", t, s);
            exit(1);
        }

        return vertex_shader;
    }
}

pub fn map_shader_type_to_glfw(r#type: &ShaderType) -> GLenum {
    match r#type {
        ShaderType::VERTEX => gl::VERTEX_SHADER,
        ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
    }
}

pub fn new_program(shader_keys: Vec<&str>) -> ShaderProgram {
    let mut shader_program = ShaderProgram {
        shaders: Vec::new(),
        glfw_program: 0,
    };

    let gl_program = unsafe { gl::CreateProgram() };

    let mut attached_shaders: Vec<GLuint> = Vec::new();
    for shader_key in shader_keys {
        let shader = get(shader_key).glfw_shader;
        attached_shaders.push(shader);
        unsafe {
            gl::AttachShader(gl_program, shader);
        }
    }

    unsafe {
        gl::LinkProgram(gl_program);
    }

    unsafe {
        for shader_key in attached_shaders {
            gl::DetachShader(gl_program, shader_key);
        }
    }

    shader_program.glfw_program = gl_program;
    shader_program
}
