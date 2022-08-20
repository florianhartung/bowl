use crate::opengl::{AttributeType, VertexArrayObject, VertexBufferObject};
use crate::renderable::Renderable;
use gl::types::GLsizei;

pub struct Mesh {
    vertices: Vec<f32>,
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
}

impl Mesh {
    pub fn new(vertices: &[f32]) -> Mesh {
        let mut cloned_vertices = Vec::from(vertices.clone());
        let mut vao = VertexArrayObject::new();
        let vbo = VertexBufferObject::new(gl::ARRAY_BUFFER, &mut cloned_vertices, gl::DYNAMIC_COPY);

        vao.add_attribute(3, AttributeType::FLOAT);
        vao.finalize_attributes();

        Mesh {
            vertices: cloned_vertices,
            vao,
            vbo,
        }
    }

    pub fn add_triangles(&mut self, triangle_vertices: &[f32]) {
        for vertex in triangle_vertices {
            self.vertices.push(*vertex);
        }
        self.vbo.load_data(&self.vertices);
    }
}

impl Renderable for Mesh {
    fn render(&self) {
        self.vao.set();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, (self.vertices.len() / 3) as GLsizei);
        }
    }
}
