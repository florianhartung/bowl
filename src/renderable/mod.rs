mod mesh;
mod rectangle;

pub type Mesh = mesh::Mesh;
pub type Rect = rectangle::Rect;

pub trait Renderable {
    fn render(&self);
}