use glam::{Vec2, Vec3};
use image::{GenericImageView, ImageFormat};

use bowl::renderable::Mesh;
use bowl::shader::{Shader, ShaderProgram, Texture};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};
use bowl::vertex::Vertex;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .vsync(false)
        .title(String::from("Textures Example"))
        .create()
        .expect("Could not create bowl window!");

    let mesh = Mesh::new(vec![
        Vertex::from(
            Vec3::new(0.5, 0.5, 0.0),
            Vec3::splat(1.0),
            Vec2::new(1.0, 0.0),
        ),
        Vertex::from(
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::splat(1.0),
            Vec2::new(1.0, 1.0),
        ),
        Vertex::from(
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::splat(1.0),
            Vec2::new(0.0, 1.0),
        ),
    ]);


    let default_vert = Shader::new(VERTEX, include_str!("./shader.vert"));
    let default_frag = Shader::new(FRAGMENT, include_str!("./shader.frag"));
    let mut program = ShaderProgram::new(vec![default_vert, default_frag]);


    let image = image::load_from_memory(include_bytes!("./awesomeface.png")).unwrap();

    let mut texture = Texture::new("awesomefaceTexture", image.width(), image.height(), image.into_bytes());

    program.load_texture_2d_rgba(&mut texture);

    window.run(|handle| {
        program.bind();

        program.set_uniform_texture(&texture);

        handle.render(&mesh, &program);
    });
}
