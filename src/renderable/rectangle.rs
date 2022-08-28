use std::ptr::null;
use crate::data::buffer_mode::DrawMode;

use crate::data::Type;
use crate::data::indices_array::IndicesBuffer;
use crate::data::vertex_array::VertexArray;
use crate::data::vertex_buffer::VertexBuffer;
use crate::data::vertex_buffer_layout::VertexBufferLayout;
use crate::renderable::Renderable;

const RECT_INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3,
];

pub struct Rect {
    vao: VertexArray,
    ib: IndicesBuffer,
}

impl Rect {
    pub fn new(vertices: [f32; 4 * 3], draw_mode: DrawMode) -> Rect {
        let mut vao = VertexArray::new();
        let ib = IndicesBuffer::new(&RECT_INDICES, DrawMode::STATIC);
        let vb = VertexBuffer::new(&vertices, draw_mode);
        let mut layout = VertexBufferLayout::new();

        layout.add_attribute(Type::Float, 3);
        layout.add_attribute(Type::Float, 3);
        layout.add_attribute(Type::Float, 2);

        vao.add_buffer(&vb, &layout);

        Rect {
            vao,
            ib,
        }
    }
}

impl Renderable for Rect {
    fn render(&self) {
        self.vao.bind();
        self.ib.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
}
