use std::ffi::c_void;
use std::mem::size_of_val;

use gl::types::{GLenum, GLsizeiptr};
use crate::data::buffer_mode::BufferMode;


pub struct IndicesBuffer {
    opengl_id: u32,
    usage: GLenum,
}

impl IndicesBuffer {
    pub fn new(data: &[u32], usage: impl BufferMode) -> Self {
        let usage = usage.to_gl_enum();

        let mut buffer: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(data) as GLsizeiptr, data.as_ptr() as *mut c_void, usage);
        }

        IndicesBuffer {
            opengl_id: buffer,
            usage,
        }
    }

    pub fn set_data(&self, data: &[u32]) {
        self.bind();
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(data) as GLsizeiptr, data.as_ptr() as *mut c_void, self.usage);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.opengl_id);
        }
    }
}