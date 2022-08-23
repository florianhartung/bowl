use glam::Vec3;

/// This struct represents a single vertex.
///
/// Note: The memory layout of this struct must match the layout
/// of a vertex in the related shader written in GLSL
pub struct Vertex {
    pub position: Vec3,
}

impl Vertex {
    pub fn from_vec(position: Vec3) -> Self {
        Self { position }
    }

    pub fn from_array(position: [f32; 3]) -> Self {
        Self::from_vec(Vec3::from_array(position))
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec(Vec3::new(x, y, z))
    }
}