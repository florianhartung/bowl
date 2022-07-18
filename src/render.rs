use std::ffi::CString;
use std::process::exit;

use gl::types::{GLenum, GLsizei, GLuint};

use crate::{opengl, shader, VertexArrayObject};
use crate::opengl::{AttributeType, VertexBufferObject};
use crate::shader::ShaderType;

pub trait Renderable {
    fn render(&self);
}

pub struct Triangle {
    vertices: [f32; 9],
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
}

impl Triangle {
    pub fn new(vertices: [f32; 9]) -> Triangle {
        let mut cloned_vertices = vertices.clone();
        let mut vao = VertexArrayObject::new();
        let vbo = VertexBufferObject::new(gl::ARRAY_BUFFER, &mut cloned_vertices, gl::STATIC_DRAW);

        vao.add_attribute(3, AttributeType::FLOAT);

        Triangle {
            vertices: cloned_vertices,
            vao,
            vbo,
        }
    }

    pub fn m(&mut self) {
        self.vertices[0] = self.vertices[0] + 0.01;
        self.vbo.load_data(&self.vertices);
    }
}

impl Renderable for Triangle {
    fn render(&self) {
        self.vao.set();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

pub fn create_shader_program(shader_keys: &Vec<String>) -> GLuint {
    let shader_program;
    unsafe { shader_program = gl::CreateProgram(); }

    for shader_key in shader_keys {
        let a = shader::get(shader_key.as_str()).glfw_shader;
        unsafe { gl::AttachShader(shader_program, a); }
    }

    unsafe {
        gl::LinkProgram(shader_program);
    }

    for shader_key in shader_keys {
        unsafe {
            gl::DetachShader(shader_program, shader::get(shader_key.as_str()).glfw_shader);
        }
    }

    return shader_program;
}

pub(crate) fn map_shader_type_to_glfw(r#type: &ShaderType) -> GLenum {
    match r#type {
        ShaderType::VERTEX => gl::VERTEX_SHADER,
        ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
    }
}