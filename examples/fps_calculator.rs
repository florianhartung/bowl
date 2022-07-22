use bowl::shader::ShaderType::{FRAGMENT, VERTEX};

fn main() {
    let window = bowl::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create();


    window.run(|handle| {
        if handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
    });
}