use std::mem::size_of_val;
use std::os::raw::c_void;

use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr};

pub struct VertexArrayObject {
    glfw_vao: u32,
    next_attribute_index: u32,
    attributes: Vec<(GLint, AttributeType)>,
    stride: i32,
}

pub enum AttributeType {
    FLOAT
}

pub struct VertexBufferObject {
    glfw_vbo: u32,
    r#type: GLenum,
    usage: GLenum,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut vao: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        let vao = VertexArrayObject {
            glfw_vao: vao,
            next_attribute_index: 0,
            attributes: Vec::new(),
            stride: 0,
        };

        vao.set();

        return vao;
    }

    pub fn add_attribute(&mut self, size: GLint, r#type: AttributeType) {
        self.stride += size * r#type.to_gl_enum_size().1;
        self.attributes.push((size, r#type));
    }

    pub fn finalize_attributes(&mut self) {
        let mut offset = 0;

        for attribute in &self.attributes {
            let (glfw_type, type_size) = attribute.1.to_gl_enum_size();

            unsafe {
                gl::BindVertexArray(self.glfw_vao);
                gl::VertexAttribPointer(self.next_attribute_index, attribute.0, glfw_type, 0, self.stride as GLsizei, offset as *const c_void);
                gl::EnableVertexAttribArray(self.next_attribute_index);
            }

            self.next_attribute_index += 1;
            offset += type_size * attribute.0;
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

impl AttributeType {
    pub fn to_gl_enum_size(&self) -> (GLenum, i32) {
        match self {
            AttributeType::FLOAT => (gl::FLOAT, 4),
        }
    }
}
