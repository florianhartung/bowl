use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLint, GLsizei, GLuint};

use crate::gl_call;
use crate::util::string_to_c_string;

pub fn new_shader(r#type: ShaderType, src: &str) -> Shader {
    Shader {
        r#type,
        opengl_id: compile_shader(r#type, src),
    }
}

#[derive(Clone, Copy)]
pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub struct Shader {
    pub r#type: ShaderType,
    pub opengl_id: GLuint,
}

pub struct ShaderProgram {
    pub shaders: Vec<String>,
    pub opengl_id: GLuint,
}

impl ShaderProgram {
    pub fn set(&self) {
        gl_call!(gl::UseProgram(self.opengl_id));
    }

    pub fn set_uniform_float(&self, name: &str, data: Vec<f32>) {
        self.internal_set_uniform(name, &data, match data.len() {
            1 => gl::Uniform1fv,
            2 => gl::Uniform2fv,
            3 => gl::Uniform3fv,
            4 => gl::Uniform4fv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type unsigned int with size {}", name, data.len()),
        });
    }

    pub fn set_uniform_int(&self, name: &str, data: Vec<i32>) {
        self.internal_set_uniform(name, &data, match data.len() {
            1 => gl::Uniform1iv,
            2 => gl::Uniform2iv,
            3 => gl::Uniform3iv,
            4 => gl::Uniform4iv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type unsigned int with size {}", name, data.len()),
        });
    }

    pub fn set_uniform_uint(&self, name: &str, data: Vec<u32>) {
        self.internal_set_uniform(name, &data, match data.len() {
            1 => gl::Uniform1uiv,
            2 => gl::Uniform2uiv,
            3 => gl::Uniform3uiv,
            4 => gl::Uniform4uiv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type unsigned int with size {}", name, data.len()),
        });
    }

    fn internal_set_uniform<T>(&self, name: &str, data: &Vec<T>, gl_function: unsafe fn(GLint, GLsizei, *const T)) {
        let uniform_location = gl_call!(gl::GetUniformLocation(self.opengl_id, string_to_c_string(name).as_ptr()));

        if uniform_location >= 0 {
            gl_call!(gl_function(uniform_location, data.len() as GLsizei, data.as_slice().as_ptr()));
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.opengl_id));
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
        opengl_id: 0,
    };

    let gl_program = gl_call!(gl::CreateProgram());

    for shader in &shaders {
        let shader = shader.opengl_id;
        gl_call!(gl::AttachShader(gl_program, shader));
    }

    gl_call!(gl::LinkProgram(gl_program));

    for shader in shaders {
        gl_call!(gl::DetachShader(gl_program, shader.opengl_id));
    }

    shader_program.opengl_id = gl_program;
    shader_program
}
