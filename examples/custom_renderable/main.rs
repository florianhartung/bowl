use glam::Vec2;

use bowl::shader::{Shader, ShaderProgram};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};

use crate::circle::Circle;

mod circle;

const PRINT_FPS: bool = false;

//noinspection RsConstantConditionIf
fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 800)
        .vsync(false)
        .title(String::from("lol"))
        .create()
        .expect("Could not create bowl window!");

    let circle = Circle::new(Vec2::new(0.0, 0.0), 0.8, 500);

    let default_vert = Shader::new(VERTEX, include_str!("./shader.vert"));
    let default_frag = Shader::new(FRAGMENT, include_str!("./shader.frag"));

    let program = ShaderProgram::new(vec![default_vert, default_frag]);


    window.run(|handle| {
        if PRINT_FPS && handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
        handle.render(&circle, &program);
    });
}