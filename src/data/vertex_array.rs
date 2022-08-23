use std::ffi::c_void;

use gl::types::GLsizei;

use crate::data::vertex_buffer::VertexBuffer;
use crate::data::vertex_buffer_layout::VertexBufferLayout;

pub struct VertexArray {
    opengl_id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut va: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut va);
        }

        let vao = VertexArray {
            opengl_id: va,
        };

        vao.bind();

        return vao;
    }

    pub fn add_buffer(&mut self, vb: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        vb.bind();

        let mut current_attribute_index = 0;
        let mut offset = 0;

        for (r#type, element_count) in &layout.attributes {
            let (gl_type, size_per_element) = r#type.to_gl_enum_and_size();

            unsafe {
                gl::VertexAttribPointer(current_attribute_index, *element_count as GLsizei, gl_type, 0, layout.stride as GLsizei, offset as *const c_void);
                gl::EnableVertexAttribArray(current_attribute_index);
            }

            current_attribute_index += 1;
            offset += element_count * size_per_element;
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.opengl_id);
        }
    }
}