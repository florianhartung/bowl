use bowl;
use bowl::{register_program, shader};
use bowl::renderable::Mesh;
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create()
        .expect("Could not create bowl window!");

    let mut mesh = Mesh::new(&[]);


    shader::register("default_vert", include_str!("./shader.vert"), VERTEX);
    shader::register("default_frag", include_str!("./shader.frag"), FRAGMENT);

    let program = register_program!("default_vert", "default_frag");

    let triangle_spawn_rate = 2 * 1000 * 1000; // one triangle every 2 seconds


    let mut until_next_spawn: i32 = triangle_spawn_rate;
    window.run(|handle| {
        until_next_spawn -= handle.dtime as i32;
        if until_next_spawn <= 0 {
            mesh.add_triangles(random_triangle().as_slice());
            until_next_spawn += triangle_spawn_rate;
        }

        handle.render(&mesh, &program);
    });
}

fn random_triangle() -> Vec<f32> {
    let mut vertices = Vec::new();
    for _ in 0..=2 {
        vertices.push(2.0 * rand::random::<f32>() - 1.0);
        vertices.push(2.0 * rand::random::<f32>() - 1.0);
        vertices.push(0.0);
    }

    return vertices;
}
