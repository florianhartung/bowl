use std::ptr::null;

use crate::opengl::{AttributeType, VertexArrayObject, VertexBufferObject};
use crate::renderable::Renderable;

const RECT_INDICES: [i32; 6] = [
    0, 1, 3,
    1, 2, 3,
];

pub struct Rect {
    vao: VertexArrayObject,
}

#[allow(const_item_mutation)]
impl Rect {
    pub fn new(mut vertices: [f32; 4 * 3]) -> Rect {
        let mut vao = VertexArrayObject::new();
        VertexBufferObject::new(gl::ARRAY_BUFFER, &mut vertices, gl::STATIC_DRAW);
        VertexBufferObject::new(gl::ELEMENT_ARRAY_BUFFER, &mut RECT_INDICES, gl::STATIC_DRAW);

        vao.add_attribute(3, AttributeType::FLOAT);
        vao.finalize_attributes();


        Rect {
            vao,
        }
    }
}

impl Renderable for Rect {
    fn render(&self) {
        self.vao.set();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
}
