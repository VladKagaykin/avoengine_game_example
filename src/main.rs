use avoengine::*;

fn main() {
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    Engine_setup();
    println!("Hello, world!");
}
