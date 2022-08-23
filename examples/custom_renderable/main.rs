use bowl::shader;
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};

use crate::circle::Circle;

mod circle;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 800)
        .vsync(false)
        .title(String::from("lol"))
        .create()
        .expect("Could not create bowl window!");

    let circle = Circle::new((0.0, 0.0), 0.8, 500);

    let default_vert = shader::new_shader(VERTEX, include_str!("./shader.vert"));
    let default_frag = shader::new_shader(FRAGMENT, include_str!("./shader.frag"));

    let program = shader::new_program(vec![default_vert, default_frag]);


    window.run(|handle| {
        if handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
        handle.render(&circle, &program);
    });
}