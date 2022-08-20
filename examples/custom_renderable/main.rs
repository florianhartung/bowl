use bowl::{register_program, shader};
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


    shader::register("default_vert", include_str!("./shader.vert"), VERTEX);
    shader::register("default_frag", include_str!("./shader.frag"), FRAGMENT);

    let program = register_program!("default_vert", "default_frag");


    window.run(|handle| {
        if handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
        handle.render(&circle, &program);
    });
}