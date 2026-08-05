use avoengine::*;

fn main() {
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    drop(settings);
    Engine_setup();

    avoengine::console_rc_render::To_console();

    println!("Hello, world!");
}
