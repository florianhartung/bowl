use std::f32::consts::TAU;

use glam::{Vec2, Vec3};

use bowl::data::buffer_mode::DrawMode;
use bowl::data::Type;
use bowl::data::vertex_array::VertexArray;
use bowl::data::vertex_buffer::VertexBuffer;
use bowl::data::vertex_buffer_layout::VertexBufferLayout;
use bowl::gl_call;
use bowl::renderable::Renderable;
use bowl::vertex::Vertex;

pub(crate) struct Circle {
    vao: VertexArray,
    sides: i32,
}

impl Circle {
    pub(crate) fn new(center: Vec2, radius: f32, sides: i32) -> Circle {
        let mut vao = VertexArray::new();

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut center_vertex = Vertex::from_position(Vec3::new(center.x, center.y, 0.0));
        center_vertex.color = Vec3::new(0.0, 0.0, 0.0);
        vertices.push(center_vertex);

        for i in 0..=sides {
            let angle_radians = TAU * (i as f32 / sides as f32);
            let offset = Vec2::new(radius * angle_radians.cos(), radius * angle_radians.sin());

            let position = Vec3::new(center.x + offset.x, center.y + offset.y, 0.0);
            let mut vertex = Vertex::from_position(position);

            // Make color of outer vertices fade from black to green
            vertex.color = Vec3::new(0.0, i as f32 / sides as f32, 0.0);

            vertices.push(vertex);
        }

        let vb = VertexBuffer::new(vertices.as_slice(), DrawMode::STATIC);
        let mut layout = VertexBufferLayout::new();
        layout.add_attribute(Type::Float, 3);
        layout.add_attribute(Type::Float, 3);
        layout.add_attribute(Type::Float, 2);

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
        gl_call!(gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.sides + 2));
    }
}
