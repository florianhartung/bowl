use glfw::{Action, Key};
use bowl;
use bowl::{register_program, shader};
use bowl::render::{Triangle};
use bowl::shader::ShaderType::{FRAGMENT, VERTEX};

fn main() {
    let window = bowl::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create();

    let mut my_triangle = Triangle::new([
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0f32,
    ]);


    shader::register("default_vert", include_str!("./shader.vert"), VERTEX);
    shader::register("default_frag", include_str!("./shader.frag"), FRAGMENT);

    let program = register_program!("default_vert", "default_frag");


    window.run(|handle| {
        if handle.window.glfw_window.get_key(Key::L) == Action::Press {
            my_triangle.m();
        }
        handle.render(&my_triangle, &program);
    });
}
