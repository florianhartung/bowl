use bowl::renderable::Mesh;
use bowl::shader::{Shader, ShaderProgram};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};
use bowl::vertex::Vertex;

use crate::sin_wave_generator::SinWaveGenerator;

mod sin_wave_generator;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create()
        .expect("Could not create bowl window!");

    let triangle = Mesh::new(vec![
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
        Vertex::new(0.0, 0.5, 0.0),
    ]);


    let default_vert = Shader::new(VERTEX, include_str!("./shader.vert"));
    let default_frag = Shader::new(FRAGMENT, include_str!("./shader.frag"));

    let program = ShaderProgram::new(vec![default_vert, default_frag]);


    let mut sin_wave_generator = SinWaveGenerator::new();
    window.run(|handle| {
        sin_wave_generator.pass_time(handle.dtime);
        let current_brightness = sin_wave_generator.calc_current();

        program.bind();
        program.set_uniform_float("triangleBrightness", vec![current_brightness]);
        handle.render(&triangle, &program);
    });
}