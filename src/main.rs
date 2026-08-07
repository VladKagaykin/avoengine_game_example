use avoengine::*;
use std::time::{Duration, Instant};
use std::f64::consts::PI;

fn main() {
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    let mut square = Draw_components{draw_type: "2d_object".to_string(), 
                                     draw_x: settings.window_width as f32 /2.0 , 
                                     draw_y: settings.window_height as f32 /2.0 ,
                                    // draw_x: 20.0 , 
                                    //  draw_y: 20.0 ,
                                     draw_z: 0.0, 
                                     draw_symbol: '#', 
                                     draw_vertices: vec![1.0,1.0,1.0,-1.0,-1.0,-1.0,-1.0,1.0],
                                     draw_RGBA_color: [255,255,255,255]};
    // Draw_queue.lock().unwrap().push(square);
    drop(settings);
    Engine_setup();
    loop{
        let mut plita = false;
        for x in (-10..10).step_by(2) {
            for z in (-10..10).step_by(2) {
                plita = !plita;
                let color = if plita { [255, 255, 255, 255] } else { [3, 78, 37, 255] };
                let vertices = vec![
                    1.0, 0.0, 1.0,  1.0, 0.0, -1.0,  -1.0, 0.0, -1.0,  
                    1.0, 0.0, 1.0,  -1.0, 0.0, -1.0,  -1.0, 0.0, 1.0   
                ];
                Draw_queue.lock().unwrap().push(Draw_components {
                    draw_type: "3d_object".to_string(),
                    draw_x: x as f32,
                    draw_y: 0.0,
                    draw_z: z as f32,
                    draw_symbol: '#',
                    draw_vertices: vertices,
                    draw_RGBA_color: color,
                });
            }
        }
        {
            let mut camera = Camera.lock().unwrap();
            camera.camera_pitch = 90.0;
            camera.camera_y = 40.0;
            camera.camera_yaw += 1.0;
            // camera.camera_z = -30.0;
        }

        avoengine::console_rc_render::Render_image_to_console();
        avoengine::console_rc_render::To_console();
    }
    // let mut angle: f64 = 0.0;
    // let mut hue: f64 = 0.0;
    // let size: f64 = 10.0;
    // let width = Engine_settings.lock().unwrap().window_width as f64;
    // let height = Engine_settings.lock().unwrap().window_height as f64;
    // let center_x = width / 2.0;
    // let center_y = height / 2.0;
    // let frame_duration = Duration::from_millis(50);

    // loop {
    //     let frame_start = Instant::now();
        
    //     let cos_a = angle.cos();
    //     let sin_a = angle.sin();
        
    //     let vertices: Vec<f64> = vec![
    //         size * cos_a - size * sin_a,  size * sin_a + size * cos_a,
    //         -size * cos_a - size * sin_a, -size * sin_a + size * cos_a,
    //         -size * cos_a + size * sin_a, -size * sin_a - size * cos_a,
    //         size * cos_a + size * sin_a,  size * sin_a - size * cos_a,
    //     ];
        
    //     let r = (hue * 6.0).cos() * 0.5 + 0.5;
    //     let g = ((hue * 6.0) - 2.0).cos() * 0.5 + 0.5;
    //     let b = ((hue * 6.0) - 4.0).cos() * 0.5 + 0.5;
        
    //     let rgb = [
    //         (r * 255.0) as u8,
    //         (g * 255.0) as u8,
    //         (b * 255.0) as u8,
    //         255
    //     ];
        
    //     let square = Draw_components {
    //         draw_type: "2d_object".to_string(),
    //         draw_x: center_x,
    //         draw_y: center_y,
    //         draw_z: 0.0,
    //         draw_symbol: '#',
    //         draw_vertices: vertices,
    //         draw_RGBA_color: rgb,
    //     };
        
    //     {
    //         let mut queue = Draw_queue.lock().unwrap();
    //         queue.clear();
    //         queue.push(square);
    //     }
        
    //     let _ = avoengine::console_rc_render::Render_image_to_console();
    //     avoengine::console_rc_render::To_console();
        
    //     angle += 0.05;
    //     hue += 0.005; 
        
    //     let elapsed = frame_start.elapsed();
    //     if elapsed < frame_duration {
    //         std::thread::sleep(frame_duration - elapsed);
    //     }
    // }

    println!("Hello, world!");
}