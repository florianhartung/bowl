use std::ffi::CString;
use std::os::raw::c_void;
use std::process::exit;

use gl::types::{GLenum, GLint, GLsizei, GLuint};
use glam::Mat4;

use crate::gl_call;
use crate::util::string_to_c_string;

const OPENGL_MAXIMUM_TEXTURES_PER_SHADER_PROGRAM: u32 = 32;

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
    pub texture_count: u32,
}

pub struct Texture<'a> {
    opengl_id: u32,
    texture_id: u32,
    is_loaded: bool,
    name: &'a str,
    width: u32,
    height: u32,
    texture_data: Vec<u8>,
}

impl<'a> Texture<'a> {
    pub fn new(name: &'a str, width: u32, height: u32, data: Vec<u8>) -> Self {
        Texture {
            opengl_id: 0,
            texture_id: 0,
            is_loaded: false,
            name,
            width,
            height,
            texture_data: data,
        }
    }
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
            texture_count: 0,
        }
    }

    pub fn load_texture_2d_rgba(&mut self, texture: &mut Texture) {
        if self.texture_count >= OPENGL_MAXIMUM_TEXTURES_PER_SHADER_PROGRAM {
            panic!("[Bowl] Can not load another texture for shader program with id {}, the maximum texture count of {} has been reached!", self.opengl_id, OPENGL_MAXIMUM_TEXTURES_PER_SHADER_PROGRAM);
        }

        gl_call!(gl::GenTextures(1, &mut texture.opengl_id));

        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + self.texture_count));
        texture.texture_id = self.texture_count;
        self.texture_count += 1;

        gl_call!(gl::BindTexture(gl::TEXTURE_2D, texture.opengl_id));

        // TODO texture wrap modes
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint));
        // TODO Mipmaps and texture scaling interpolation settings
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, texture.width as GLsizei, texture.height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, texture.texture_data.as_mut_ptr() as *const c_void));


        // TODO Generate mipmaps
        gl_call!(gl::GenerateMipmap(gl::TEXTURE_2D));

        texture.is_loaded = true;
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

    pub fn set_uniform_mat4(&self, name: &str, data: Mat4) {
        if let Some(uniform_location) = self.get_uniform_location(name) {
            gl_call!(gl::UniformMatrix4fv(uniform_location, 1, 0, data.to_cols_array().as_ptr()));
        }
    }

    pub fn set_uniform_texture(&self, texture: &Texture) {
        if !texture.is_loaded {
            panic!("Can not set uniform with name {} for unloaded texture with id {}.", texture.name, texture.texture_id);
        }

        self.set_uniform_int(texture.name, vec![texture.texture_id as i32]);
    }

    fn internal_set_uniform_vector<T>(&self, name: &str, data: &Vec<T>, gl_function: unsafe fn(GLint, GLsizei, *const T)) {
        if let Some(uniform_location) = self.get_uniform_location(name) {
            gl_call!(gl_function(uniform_location, data.len() as GLsizei, data.as_slice().as_ptr()));
        }
    }

    fn get_uniform_location(&self, name: &str) -> Option<GLint> {
        let uniform_location = gl_call!(gl::GetUniformLocation(self.opengl_id, string_to_c_string(name).as_ptr()));

        // A uniform location of -1 is returned when the requested uniform is not used in the shaders.
        // Thus only uniforms with valid uniform locations ( >= 0 ) can be set
        if uniform_location == -1 {
            return None;
        }
        Some(uniform_location)
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