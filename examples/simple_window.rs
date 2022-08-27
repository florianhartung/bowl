use bowl;

fn main() {
    let window = bowl::window::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create()
        .expect("Could not initialize window");

    window.run(|_handle| {});
}