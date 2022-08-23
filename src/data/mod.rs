use gl::types::GLenum;

pub mod vertex_buffer;
pub mod vertex_array;
pub mod indices_array;
pub mod vertex_buffer_layout;
pub mod buffer_mode;

#[derive(Clone, Copy)]
pub enum Type {
    Float,
}

impl Type {
    pub fn to_gl_enum_and_size(&self) -> (GLenum, u32) {
        match self {
            Type::Float => (gl::FLOAT, 4),
        }
    }
}
