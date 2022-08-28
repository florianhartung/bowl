use std::ops::Range;

use glam::{Vec2, Vec3};

use bowl;
use bowl::renderable::Mesh;
use bowl::shader::{Shader, ShaderProgram};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};
use bowl::vertex::Vertex;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create()
        .expect("Could not create bowl window!");

    let mut mesh = Mesh::new(Vec::new());


    let default_vert = Shader::new(VERTEX, include_str!("./shader.vert"));
    let default_frag = Shader::new(FRAGMENT, include_str!("./shader.frag"));

    let program = ShaderProgram::new(vec![default_vert, default_frag]);

    let triangle_spawn_rate = 2 * 1000 * 1000; // one triangle every 2 seconds


    let mut until_next_spawn: i32 = triangle_spawn_rate;
    window.run(|handle| {
        until_next_spawn -= handle.dtime as i32;
        if until_next_spawn <= 0 {
            mesh.add_vertices(random_triangle());
            until_next_spawn += triangle_spawn_rate;
        }

        handle.render(&mesh, &program);
    });
}

fn random_triangle() -> Vec<Vertex> {
    let triangle_color = Vec3::new(
        rand_in_range(0.0..1.0),
        rand_in_range(0.0..1.0),
        rand_in_range(0.0..1.0),
    );

    let mut vertices = Vec::new();
    for _ in 0..=2 {
        let vertex_position = Vec3::new(
            rand_in_range(-1.0..1.0),
            rand_in_range(-1.0..1.0),
            0.0,
        );

        vertices.push(Vertex::from(vertex_position, triangle_color, Vec2::new(0.0, 0.0)));
    }

    return vertices;
}

fn rand_in_range(range: Range<f32>) -> f32 {
    rand::random::<f32>() * (range.end - range.start) + range.start
}