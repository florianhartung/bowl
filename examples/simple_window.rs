use bowl;

fn main() {
    let window = bowl::WindowBuilder::new()
        .size(800, 600)
        .fullscreen(false)
        .create();

    window.run(|_handle| {});
}