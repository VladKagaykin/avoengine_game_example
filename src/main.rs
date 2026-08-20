use avoengine::*;
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::event::ElementState;

fn main() {
    let mut fps_last_time = Instant::now();
    let mut fps_frame_count = 0;
    let (window, event_loop) = window_processing::create_window("avoengine".to_string());
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    let mut square = Draw_components{draw_type: "2d_object".to_string(), 
                                     draw_x: settings.window_width as f32 /2.0 , 
                                     draw_y: settings.window_height as f32 /2.0 ,
                                     draw_z: 0.0, 
                                     draw_symbol: '#', 
                                     draw_vertices: vec![1.0,1.0,1.0,-1.0,-1.0,-1.0,-1.0,1.0],
                                     draw_RGBA_color: [255,255,255,25],
                                     draw_texture_path: "data/penza_low.png".to_string(),
                                     draw_special_name: "none".to_string()};
    let cube_vertices = vec![
        -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,

        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,

        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0, -1.0, -1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,

        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0, -1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0, -1.0,  1.0,

        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,
    ];
    
    let vertices = vec![
                1.0, 0.0, 1.0,  1.0, 0.0, -1.0,  -1.0, 0.0, -1.0,  
                1.0, 0.0, 1.0,  -1.0, 0.0, -1.0,  -1.0, 0.0, 1.0   
            ];
    Static_scene.lock().unwrap().push(square);
    drop(settings);
    Engine_setup();
    avoengine::tick_system::Init_tick_system();
    let mut direction_up= true;
    let mut plita = false;
    for x in (-10..10).step_by(2) {
        for z in (-10..10).step_by(2) {
            plita = ((x / 2) + (z / 2)) % 2 == 0;
            let color = if plita { [58, 58, 58, 255] } else { [3, 78, 37, 255] };
            Static_scene.lock().unwrap().push(Draw_components {
                draw_type: "3d_object".to_string(),
                draw_x: x as f32,
                draw_y: 0.0,
                draw_z: z as f32,
                draw_symbol: '█',
                draw_vertices: vertices.clone(),
                draw_RGBA_color: color,
                draw_texture_path: "none".to_string(),
                draw_special_name: "none".to_string()
            });
        }
    }

    Static_scene.lock().unwrap().push(Draw_components {
                draw_type: "3d_object".to_string(),
                draw_x: 0.0,
                draw_y: 10.0,
                draw_z: 0.0,
                draw_symbol: '█',
                draw_vertices: vec![
                                    1.0, 0.0, 1.0,  -1.0, 0.0, -1.0,  1.0, 0.0, -1.0,
                                    1.0, 0.0, 1.0,  -1.0, 0.0, 1.0,  -1.0, 0.0, -1.0,
                                ],
                draw_RGBA_color: [255,255,255,255],
                draw_texture_path: "avoengine_rust/src/logo.png".to_string(),
                draw_special_name: "none".to_string()
            });

    Static_scene.lock().unwrap().push(Draw_components {
        draw_type: "3d_object".to_string(),
        draw_x: 7.0,
        draw_y: 1.0,
        draw_z: 7.0,
        draw_symbol: '#',
        draw_vertices: cube_vertices.clone(),
        draw_RGBA_color: [255, 0, 0, 255],
        draw_texture_path: "none".to_string(),
        draw_special_name: "none".to_string()
    });
    Static_scene.lock().unwrap().push(Draw_components {
        draw_type: "3d_object".to_string(),
        draw_x: -9.0,
        draw_y: 1.0,
        draw_z: -9.0,
        draw_symbol: '#',
        draw_vertices: cube_vertices.clone(),
        draw_RGBA_color: [0, 0, 255, 255],
        draw_texture_path: "none".to_string(),
        draw_special_name: "none".to_string()
    });
    Static_scene.lock().unwrap().push(Draw_components {
        draw_type: "3d_object".to_string(),
        draw_x: -5.0,
        draw_y: 1.0,
        draw_z: 5.0,
        draw_symbol: '#',
        draw_vertices: cube_vertices.clone(),
        draw_RGBA_color: [255, 255, 255, 64],
        draw_texture_path: "none".to_string(),
        draw_special_name: "none".to_string()
    });

    Static_light.lock().unwrap().push(Light_components {
        light_x: 0.0,
        light_y: 5.0,
        light_z: 0.0,
        light_RGB_color: [255, 255, 255],
        light_distance: 58.0,
        light_cone_angle: 90.0,
        light_pitch: 1.0,
        light_yaw: 0.0,
        light_special_name: "none".to_string()
    });
    let mut examination = avoengine::Is_scene_changed.lock().unwrap();
    *examination = true;
    drop(examination);
    
    let mut last_fps_tick = 0;
    let mut frame_count = 0;
    let mut last_frame_count = 0;
    let mut camera_speed:f32 = 0.58;
    let mut camera_angle_speed: f32 = 5.8; 
    let mut yaw_light: f32 = 0.0;
    let mut last_tick: u128 = 0;
    
    event_loop.run(move |event, target| {
    target.set_control_flow(ControlFlow::Poll);
    
    match event {
        Event::WindowEvent {
            event: window_event,
            ..
        } => {
            if let WindowEvent::KeyboardInput { event: key_event, .. } = &window_event {
                if let PhysicalKey::Code(key_code) = key_event.physical_key {
                    let pressed = key_event.state == ElementState::Pressed;
                    avoengine::window_processing::update_key_state(key_code, pressed);
                }
            }
            
            match window_event {
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    if let PhysicalKey::Code(key_code) = key_event.physical_key {
                        let pressed = key_event.state == ElementState::Pressed;
                        window_processing::update_key_state(key_code, pressed);
                    }
                }
                WindowEvent::CloseRequested => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        Event::AboutToWait => {
            avoengine::tick_system::Tick_update();
            let current_tick = avoengine::tick_system::Get_tick();

            let angle = (avoengine::tick_system::Get_tick() as f32 * 0.02) % (2.0 * PI);
            let (sin_a, cos_a) = (angle.sin(), angle.cos());
            
            let mut rotated = cube_vertices.clone();
            for i in (0..rotated.len()).step_by(3) {
                let (x, z) = (rotated[i], rotated[i+2]);
                rotated[i] = x * cos_a - z * sin_a;
                rotated[i+2] = x * sin_a + z * cos_a;
            }

            Light_queue.lock().unwrap().push(Light_components {
                light_x: 0.0,
                light_y: 1.0,
                light_z: 0.0,
                light_RGB_color: [0, 255, 0],
                light_distance: 580.0,
                light_cone_angle: 90.0,
                light_pitch: 90.0,
                light_yaw:yaw_light,
                light_special_name: "none".to_string()
            });
            Light_queue.lock().unwrap().push(Light_components {
                light_x: 0.0,
                light_y: 1.0,
                light_z: 0.0,
                light_RGB_color: [0, 0, 255],
                light_distance: 580.0,
                light_cone_angle: 90.0,
                light_pitch: 90.0,
                light_yaw:yaw_light-90.0,
                light_special_name: "none".to_string()
            });
            Light_queue.lock().unwrap().push(Light_components {
                light_x: 0.0,
                light_y: 1.0,
                light_z: 0.0,
                light_RGB_color: [255, 0, 0],
                light_distance: 580.0,
                light_cone_angle: 90.0,
                light_pitch: 90.0,
                light_yaw:yaw_light-180.0,
                light_special_name: "none".to_string()
            });

            Light_queue.lock().unwrap().push(Light_components {
                light_x: 0.0,
                light_y: 1.0,
                light_z: 0.0,
                light_RGB_color: [255, 255, 255],
                light_distance: 580.0,
                light_cone_angle: 90.0,
                light_pitch: 90.0,
                light_yaw:yaw_light-270.0,
                light_special_name: "none".to_string()
            });
            
            Draw_queue.lock().unwrap().push(Draw_components {
                draw_type: "3d_object".to_string(),
                draw_x: 5.0,
                draw_y: 1.0,
                draw_z: -5.0,
                draw_symbol: '#',
                draw_vertices: rotated,
                draw_RGBA_color: [0, 255, 0, 255],
                draw_texture_path: "none".to_string(),
                draw_special_name: "none".to_string()
            });

            // let keys = avoengine::console_input::get_pressed_keys();
            let keys = avoengine::window_processing::get_pressed_keys(&window);

            fps_frame_count += 1;
            let elapsed = fps_last_time.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = fps_frame_count as f64 / elapsed.as_secs_f64();
                last_frame_count=fps as i128;
                fps_frame_count = 0;
                fps_last_time = Instant::now();
            }

            // {
            //     let mut camera = Camera.lock().unwrap();
            //     match keys.as_slice() {
            //         [i, ..] if *i == 'i' => camera.camera_pitch += camera_angle_speed,
            //         [k, ..] if *k == 'k' => camera.camera_pitch -= camera_angle_speed,
            //         [j, ..] if *j == 'j' => camera.camera_yaw += camera_angle_speed,
            //         [l, ..] if *l == 'l' => camera.camera_yaw -= camera_angle_speed,
            //         [w, ..] if *w == 'w' => {
            //             let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
            //             camera.camera_x += basis.forward[0] * camera_speed;
            //             camera.camera_y += basis.forward[1] * camera_speed;
            //             camera.camera_z += basis.forward[2] * camera_speed;
            //         },
            //         [s, ..] if *s == 's' => {
            //             let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
            //             camera.camera_x -= basis.forward[0] * camera_speed;
            //             camera.camera_y -= basis.forward[1] * camera_speed;
            //             camera.camera_z -= basis.forward[2] * camera_speed;
            //         },
            //         [a, ..] if *a == 'a' => {
            //             let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
            //             camera.camera_x -= basis.right[0] * camera_speed;
            //             camera.camera_y -= basis.right[1] * camera_speed;
            //             camera.camera_z -= basis.right[2] * camera_speed;
            //         },
            //         [d, ..] if *d == 'd' => {
            //             let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
            //             camera.camera_x += basis.right[0] * camera_speed;
            //             camera.camera_y += basis.right[1] * camera_speed;
            //             camera.camera_z += basis.right[2] * camera_speed;
            //         },
            //         [q, ..] if *q == 'q' => break,
            //         _ => {}
            //     }
            // }
            if current_tick != last_tick.try_into().unwrap(){
                yaw_light += 25.0;

                {
                    let mut camera = Camera.lock().unwrap();
                    
                    for key in &keys {
                        match key {
                            KeyCode::KeyI => camera.camera_pitch += camera_angle_speed,
                            KeyCode::KeyK => camera.camera_pitch -= camera_angle_speed,
                            KeyCode::KeyJ => camera.camera_yaw += camera_angle_speed,
                            KeyCode::KeyL => camera.camera_yaw -= camera_angle_speed,
                            KeyCode::KeyW => {
                                let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
                                camera.camera_x += basis.forward[0] * camera_speed;
                                camera.camera_y += basis.forward[1] * camera_speed;
                                camera.camera_z += basis.forward[2] * camera_speed;
                            },
                            KeyCode::KeyS => {
                                let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
                                camera.camera_x -= basis.forward[0] * camera_speed;
                                camera.camera_y -= basis.forward[1] * camera_speed;
                                camera.camera_z -= basis.forward[2] * camera_speed;
                            },
                            KeyCode::KeyA => {
                                let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
                                camera.camera_x -= basis.right[0] * camera_speed;
                                camera.camera_y -= basis.right[1] * camera_speed;
                                camera.camera_z -= basis.right[2] * camera_speed;
                            },
                            KeyCode::KeyD => {
                                let basis = avoengine::console_rc_render::camera_basis(camera.camera_pitch, camera.camera_yaw, camera.camera_roll);
                                camera.camera_x += basis.right[0] * camera_speed;
                                camera.camera_y += basis.right[1] * camera_speed;
                                camera.camera_z += basis.right[2] * camera_speed;
                            },
                            KeyCode::KeyQ => std::process::exit(0),
                            _ => {}
                        }
                    }
                }
            last_tick = current_tick;
            }

            avoengine::console_rc_render::Render_image_to_console();
            // avoengine::console_rc_render::To_console();
            
            let _ = window_processing::update_frame(&window);

            println!("[Q]Exit;    Camera angle: [I]Up;[J]Left;[K]Down;[L]Right;    Camera Position: [W]Forward;[A]Left;[S]Backward;[D]Right;");

            println!("{:?}", keys);

            println!("FPS: {}", last_frame_count);
            }
            _ => {}
        }
    });
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