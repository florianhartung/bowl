use gl::types::GLsizei;
use crate::data::buffer_mode::DrawMode;

use crate::data::Type;
use crate::data::vertex_array::VertexArray;
use crate::data::vertex_buffer::VertexBuffer;
use crate::data::vertex_buffer_layout::VertexBufferLayout;
use crate::renderable::Renderable;
use crate::vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    vao: VertexArray,
    vbo: VertexBuffer,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>) -> Mesh {
        let mut va = VertexArray::new();
        let vb = VertexBuffer::new(&vertices, DrawMode::DYNAMIC);
        let mut layout = VertexBufferLayout::new();

        layout.add_attribute(Type::Float, 3);
        va.add_buffer(&vb, &layout);

        Mesh {
            vertices,
            vao: va,
            vbo: vb,
        }
    }

    pub fn add_vertices(&mut self, triangle_vertices: Vec<Vertex>) {
        for vertex in triangle_vertices {
            self.vertices.push(vertex);
        }
        self.vbo.set_data(&self.vertices);
    }
}

impl Renderable for Mesh {
    fn render(&self) {
        self.vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as GLsizei);
        }
    }
}