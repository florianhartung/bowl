use std::f32::consts::TAU;
use bowl::opengl::{AttributeType, VertexArrayObject, VertexBufferObject};
use bowl::renderable::Renderable;

pub(crate) struct Circle {
    vao: VertexArrayObject,
    sides: i32,
}

impl Circle {
    pub(crate) fn new(center: (f32, f32), radius: f32, sides: i32) -> Circle {
        let mut vao = VertexArrayObject::new();

        let mut vertices: Vec<f32> = Vec::new();
        vertices.push(center.0);
        vertices.push(center.1);
        vertices.push(0.0);

        for i in 0..=sides {
            let angle_radians = TAU * (i as f32 / sides as f32);
            let offset = (radius * angle_radians.cos(), radius * angle_radians.sin());

            vertices.push(center.0 + offset.0);
            vertices.push(center.1 + offset.1);
            vertices.push(0.0);
        }

        VertexBufferObject::new(gl::ARRAY_BUFFER, vertices.as_mut_slice(), gl::STATIC_DRAW);

        vao.add_attribute(3, AttributeType::FLOAT);
        vao.finalize_attributes();

        return Circle {
            vao,
            sides
        };
    }
}

impl Renderable for Circle {
    fn render(&self) {
        self.vao.set();
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.sides + 2);
        }
    }
}
