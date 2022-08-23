use crate::data::Type;

pub struct VertexBufferLayout {
    pub(crate) stride: u32,
    pub(crate) attributes: Vec<(Type, u32)>,
}


impl VertexBufferLayout {
    pub fn new() -> Self {
        VertexBufferLayout {
            stride: 0,
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, r#type: Type, count: u32) {
        self.attributes.push((r#type, count));
        self.stride += count * r#type.to_gl_enum_and_size().1;
    }
}