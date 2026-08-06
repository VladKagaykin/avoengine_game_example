use avoengine::*;

fn main() {
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    let mut square = Draw_components{draw_type: "2d_object".to_string(), 
                                     draw_x: settings.window_width as f64 /2.0 , 
                                     draw_y: settings.window_height as f64 /2.0 ,
                                    // draw_x: 20.0 , 
                                    //  draw_y: 20.0 ,
                                     draw_z: 0.0, 
                                     draw_symbol: '#', 
                                     draw_vertices: vec![1.0,1.0,1.0,-1.0,-1.0,-1.0,-1.0,1.0],
                                     draw_RGBA_color: [255,255,255,255]};
    Draw_queue.lock().unwrap().push(square);
    drop(settings);
    Engine_setup();

    avoengine::console_rc_render::Render_image_to_console();
    avoengine::console_rc_render::To_console();

    println!("Hello, world!");
}
