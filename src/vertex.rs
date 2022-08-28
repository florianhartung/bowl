use glam::{Vec2, Vec3};

/// This struct represents a single vertex.
///
/// Note: The memory layout of this struct must match the layout
/// of a vertex in the related shader written in GLSL
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub texture_coordinates: Vec2,
}

impl Vertex {
    pub fn from_position(position: Vec3) -> Self {
        Self::from(
            position,
            Vec3::new(1.0, 0.0, 0.0),
            Vec2::new(0.0, 0.0),
        )
    }

    pub fn from(position: Vec3, color: Vec3, texture_coordinates: Vec2) -> Self {
        Self {
            position,
            color,
            texture_coordinates,
        }
    }
}