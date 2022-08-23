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


    shader::new_shader("default_vert", include_str!("./shader.vert"), VERTEX);
    shader::new_shader("default_frag", include_str!("./shader.frag"), FRAGMENT);

    let program = shader::new_program(["default_vert", "default_frag"].to_vec());


    window.run(|handle| {
        if handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
        handle.render(&circle, &program);
    });
}