use std::mem::size_of_val;
use std::os::raw::c_void;
use std::ptr::null;

use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr};

pub struct VertexArrayObject {
    glfw_vao: u32,
    next_attribute_index: u32,
}

pub enum AttributeType {
    FLOAT
}

pub struct VertexBufferObject {
    glfw_vbo: u32,
    r#type: GLenum,
    usage: GLenum,
}

fn map_type_to_gl_type(r#type: AttributeType) -> (GLenum, u32) {
    match r#type {
        AttributeType::FLOAT => (gl::FLOAT, 4),
    }
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        return VertexArrayObject {
            glfw_vao: vao,
            next_attribute_index: 0,
        };
    }
    pub fn add_attribute(&mut self, size: GLint, r#type: AttributeType) {
        let (glfw_type, type_size) = map_type_to_gl_type(r#type);

        unsafe {
            gl::BindVertexArray(self.glfw_vao);
            gl::VertexAttribPointer(self.next_attribute_index, size, glfw_type, 0, size * type_size as GLsizei, null());
            gl::EnableVertexAttribArray(self.next_attribute_index);
            self.next_attribute_index += 1;
        }
    }
    pub fn set(&self) {
        unsafe {
            gl::BindVertexArray(self.glfw_vao);
        }
    }
}

impl VertexBufferObject {
    pub fn new<T>(r#type: GLenum, data: &mut [T], usage: GLenum) -> VertexBufferObject {
        let mut buffer: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(r#type, buffer);
            gl::BufferData(r#type, size_of_val(data) as GLsizeiptr, data as *mut _ as *mut c_void, usage);
        }

        VertexBufferObject {
            r#type,
            glfw_vbo: buffer,
            usage,
        }
    }

    pub fn load_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BindBuffer(self.r#type, self.glfw_vbo);
            gl::BufferData(self.r#type, size_of_val(data) as GLsizeiptr, data as *const _ as *const c_void, self.usage);
        }
    }
}