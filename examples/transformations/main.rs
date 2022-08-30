use glam::{Mat4, Vec3};

use bowl::renderable::Mesh;
use bowl::shader::{Shader, ShaderProgram, Texture};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};
use bowl::vertex::Vertex;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 800)
        .fullscreen(false)
        .create()
        .expect("Could not create bowl window!");

    let triangle = Mesh::new(vec![
        Vertex::from_position(Vec3::new(-0.5, -0.5, 0.0)),
        Vertex::from_position(Vec3::new(0.5, -0.5, 0.0)),
        Vertex::from_position(Vec3::new(0.0, 0.5, 0.0)),
    ]);


    let default_vert = Shader::new(VERTEX, include_str!("./shader.vert"));
    let default_frag = Shader::new(FRAGMENT, include_str!("./shader.frag"));

    let program = ShaderProgram::new(vec![default_vert, default_frag]);


    // TODO: MVP Matrix, Link with render3d function of WindowHandle
    // let mut i = 0.0;
    // let mut translation_vec = Vec3::new(0.0, 0.0, 0.0);
    // let mut translation = Mat4::from_translation(translation_vec);
    window.run(|handle| {
        // i += handle.dtime as f32 / 1_000_000.;

        program.bind();

        // translation_vec.y = i.sin();

        // translation = Mat4::from_translation(translation_vec);
        // rotation = Mat4::from_rotation_z(i);
        // let mvp = translation * rotation;

        // program.set_uniform_mat4("transform", translation);

        handle.render(&triangle, &program);
    });
}
