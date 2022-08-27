use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLint, GLsizei, GLuint};

use crate::gl_call;
use crate::util::string_to_c_string;


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

impl Shader {
    pub fn new(r#type: ShaderType, src: &str) -> Self {
        Self {
            r#type,
            opengl_id: Self::compile(r#type, src),
        }
    }

    fn compile(r#type: ShaderType, src: &str) -> GLuint {
        let r#type = r#type.to_gl();

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
}

impl ShaderProgram {
    pub fn new(shaders: Vec<Shader>) -> Self {
        let gl_program = gl_call!(gl::CreateProgram());

        for shader in &shaders {
            let shader = shader.opengl_id;
            gl_call!(gl::AttachShader(gl_program, shader));
        }

        gl_call!(gl::LinkProgram(gl_program));

        for shader in shaders {
            gl_call!(gl::DetachShader(gl_program, shader.opengl_id));
        }

        Self {
            shaders: Vec::new(),
            opengl_id: gl_program,
        }
    }
    pub fn bind(&self) {
        gl_call!(gl::UseProgram(self.opengl_id));
    }

    pub fn set_uniform_float(&self, name: &str, data: Vec<f32>) {
        self.internal_set_uniform_vector(name, &data, match data.len() {
            1 => gl::Uniform1fv,
            2 => gl::Uniform2fv,
            3 => gl::Uniform3fv,
            4 => gl::Uniform4fv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type float with size {}", name, data.len()),
        });
    }

    pub fn set_uniform_int(&self, name: &str, data: Vec<i32>) {
        self.internal_set_uniform_vector(name, &data, match data.len() {
            1 => gl::Uniform1iv,
            2 => gl::Uniform2iv,
            3 => gl::Uniform3iv,
            4 => gl::Uniform4iv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type signed int with size {}", name, data.len()),
        });
    }

    pub fn set_uniform_uint(&self, name: &str, data: Vec<u32>) {
        self.internal_set_uniform_vector(name, &data, match data.len() {
            1 => gl::Uniform1uiv,
            2 => gl::Uniform2uiv,
            3 => gl::Uniform3uiv,
            4 => gl::Uniform4uiv,
            _ => panic!("[Bowl] Could not set shader uniform '{}' of type unsigned int with size {}", name, data.len()),
        });
    }

    fn internal_set_uniform_vector<T>(&self, name: &str, data: &Vec<T>, gl_function: unsafe fn(GLint, GLsizei, *const T)) {
        let uniform_location = gl_call!(gl::GetUniformLocation(self.opengl_id, string_to_c_string(name).as_ptr()));

        // A uniform location of -1 is returned when the requested uniform is not used in the shaders.
        // Thus only uniforms with valid uniform locations ( >= 0 ) can be set
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

impl ShaderType {
    fn to_gl(&self) -> GLenum {
        match self {
            ShaderType::VERTEX => gl::VERTEX_SHADER,
            ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
        }
    }
}