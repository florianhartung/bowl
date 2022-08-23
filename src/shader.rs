use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLsizei, GLuint};

use crate::gl_call;

pub fn new_shader(r#type: ShaderType, src: &str) -> Shader {
    Shader {
        r#type,
        glfw_shader: compile_shader(r#type, src),
    }
}

#[derive(Clone, Copy)]
pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub struct Shader {
    pub r#type: ShaderType,
    pub glfw_shader: GLuint,
}

pub struct ShaderProgram {
    pub shaders: Vec<String>,
    pub glfw_program: GLuint,
}

impl ShaderProgram {
    pub fn set(&self) {
        gl_call!(gl::UseProgram(self.glfw_program));
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.glfw_program));
    }
}

pub(crate) fn compile_shader(r#type: ShaderType, src: &str) -> GLuint {
    let r#type = map_shader_type_to_glfw(r#type);

    let shader = gl_call!(gl::CreateShader(r#type));

    gl_call!(
        gl::ShaderSource(shader, 1, &(src.as_bytes().as_ptr().cast()), &src.len().try_into().unwrap()),
        gl::CompileShader(shader),
    );

    let mut success: i32 = 0;

    gl_call!(gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success));
    if success == 0 {
        let mut log: [i8; 1024] = [0; 1024];
        gl_call!(gl::GetShaderInfoLog(shader, 1024, 0 as *mut GLsizei, log.as_mut_ptr()));
        let s: String = (unsafe { CString::from_raw(log.as_mut_ptr()) }).into_string().unwrap();
        println!("[Bowl] Could not compile shader of type {}:\n{}", r#type, s);
        exit(1);
    }

    shader
}

pub fn map_shader_type_to_glfw(r#type: ShaderType) -> GLenum {
    match r#type {
        ShaderType::VERTEX => gl::VERTEX_SHADER,
        ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
    }
}

pub fn new_program(shaders: Vec<Shader>) -> ShaderProgram {
    let mut shader_program = ShaderProgram {
        shaders: Vec::new(),
        glfw_program: 0,
    };

    let gl_program = gl_call!(gl::CreateProgram());

    for shader in &shaders {
        let shader = shader.glfw_shader;
        gl_call!(gl::AttachShader(gl_program, shader));
    }

    gl_call!(gl::LinkProgram(gl_program));

    for shader in shaders {
        gl_call!(gl::DetachShader(gl_program, shader.glfw_shader));
    }

    shader_program.glfw_program = gl_program;
    shader_program
}
