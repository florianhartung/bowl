fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create()
        .expect("Could not initialize window");


    window.run(|handle| {
        if handle.dtime > 0 {
            println!("{}", 1_000_000.0 / handle.dtime as f32);
        }
    });
}