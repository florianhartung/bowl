use std::f32::consts::TAU;

use bowl::data::Type;
use bowl::data::buffer_mode::DrawMode;
use bowl::data::vertex_array::VertexArray;
use bowl::data::vertex_buffer::VertexBuffer;
use bowl::data::vertex_buffer_layout::VertexBufferLayout;
use bowl::renderable::Renderable;
use bowl::vertex::Vertex;

pub(crate) struct Circle {
    vao: VertexArray,
    sides: i32,
}

impl Circle {
    pub(crate) fn new(center: (f32, f32), radius: f32, sides: i32) -> Circle {
        let mut vao = VertexArray::new();

        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.push(Vertex::new(center.0, center.1, 0.0));

        for i in 0..=sides {
            let angle_radians = TAU * (i as f32 / sides as f32);
            let offset = (radius * angle_radians.cos(), radius * angle_radians.sin());

            let (x, y) = (center.0 + offset.0, center.1 + offset.1);

            vertices.push(Vertex::new(x, y, 0.0));
        }

        let vb = VertexBuffer::new(vertices.as_slice(), DrawMode::STATIC);
        let mut layout = VertexBufferLayout::new();
        layout.add_attribute(Type::Float, 3);

        vao.add_buffer(&vb, &layout);

        return Circle {
            vao,
            sides,
        };
    }
}

impl Renderable for Circle {
    fn render(&self) {
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.sides + 2);
        }
    }
}
