use avoengine::*;
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::event::ElementState;

fn main() {
    let (window, event_loop) = window_processing::create_window("avoengine".to_string());
    let settings = Engine_settings.lock().unwrap();
    Setup_window(&settings.window_width, &settings.window_height);
    drop(settings);
    Engine_setup();
    avoengine::tick_system::Init_tick_system();
    avoengine::sound::Init_sound();
    avoengine::sound::Load_sound("data/voyager.wav");
    
    avoengine::maps::Load_map("data/default_map.txt".to_string());
    // avoengine::maps::Load_map("data/boos.txt".to_string());

    {
    let radius = 1.0;
    let curvature = 2.5;
    let nr = 12;
    let ntheta = 32;
    let thickness = 0.8;
    let mut verts = Vec::new();
    for &sign in &[1.0, -1.0] {
        for i in 0..nr {
            for j in 0..ntheta {
                let r = (i as f32 / (nr - 1) as f32) * radius;
                let theta = (j as f32 / ntheta as f32) * 2.0 * std::f32::consts::PI;
                let x = r * theta.cos();
                let y = r * theta.sin();
                let z_sphere = (curvature * curvature - r * r).sqrt();
                let z = sign * (z_sphere - (curvature - thickness / 2.0));
                verts.push(x);
                verts.push(y);
                verts.push(z);
            }
        }
    }
    let angle = -45.0_f32.to_radians();
    let (cos_a, sin_a) = angle.sin_cos();
    for chunk in verts.chunks_exact_mut(3) {
        let x = chunk[0];
        let z = chunk[2];
        chunk[0] = x * cos_a - z * sin_a;
        chunk[2] = x * sin_a + z * cos_a;
    }
    let mut triangles = Vec::new();
    let n = nr * ntheta;
    let offset_front = 0;
    let offset_back = n;
    fn push_tri(out: &mut Vec<f32>, verts: &[f32], i0: usize, i1: usize, i2: usize) {
        out.extend_from_slice(&verts[i0 * 3..i0 * 3 + 3]);
        out.extend_from_slice(&verts[i1 * 3..i1 * 3 + 3]);
        out.extend_from_slice(&verts[i2 * 3..i2 * 3 + 3]);
    }
    for j in 0..ntheta {
        let j_next = (j + 1) % ntheta;
        let center = 0;
        let ring0 = 1 * ntheta + j;
        let ring1 = 1 * ntheta + j_next;
        push_tri(&mut triangles, &verts, offset_front + center, offset_front + ring0, offset_front + ring1);
    }
    for i in 1..(nr - 1) {
        for j in 0..ntheta {
            let j_next = (j + 1) % ntheta;
            let idx00 = i * ntheta + j;
            let idx01 = i * ntheta + j_next;
            let idx10 = (i + 1) * ntheta + j;
            let idx11 = (i + 1) * ntheta + j_next;
            push_tri(&mut triangles, &verts, offset_front + idx00, offset_front + idx10, offset_front + idx01);
            push_tri(&mut triangles, &verts, offset_front + idx01, offset_front + idx10, offset_front + idx11);
        }
    }
    for j in 0..ntheta {
        let j_next = (j + 1) % ntheta;
        let center = 0;
        let ring0 = 1 * ntheta + j;
        let ring1 = 1 * ntheta + j_next;
        push_tri(&mut triangles, &verts, offset_back + center, offset_back + ring1, offset_back + ring0);
    }
    for i in 1..(nr - 1) {
        for j in 0..ntheta {
            let j_next = (j + 1) % ntheta;
            let idx00 = i * ntheta + j;
            let idx01 = i * ntheta + j_next;
            let idx10 = (i + 1) * ntheta + j;
            let idx11 = (i + 1) * ntheta + j_next;
            push_tri(&mut triangles, &verts, offset_back + idx00, offset_back + idx01, offset_back + idx10);
            push_tri(&mut triangles, &verts, offset_back + idx01, offset_back + idx11, offset_back + idx10);
        }
    }
    let edge_idx = nr - 1;
    for j in 0..ntheta {
        let j_next = (j + 1) % ntheta;
        let f0 = edge_idx * ntheta + j;
        let f1 = edge_idx * ntheta + j_next;
        let b0 = offset_back + edge_idx * ntheta + j;
        let b1 = offset_back + edge_idx * ntheta + j_next;
        push_tri(&mut triangles, &verts, f0, f1, b0);
        push_tri(&mut triangles, &verts, b0, f1, b1);
    }
    let lens = avoengine::Draw_components {
        draw_type: "3d_object".to_string(),
        draw_x: 5.0,
        draw_y: 1.0,
        draw_z: 5.0,
        draw_symbol: '◆',
        draw_vertices: triangles,
        draw_RGBA_color: [255, 255, 255, 25],
        draw_texture_path: "none".to_string(),
        special_properties: "refraction[1.5]".to_string(),
        draw_special_name: "".to_string(),
    };
    avoengine::Static_scene.lock().unwrap().push(lens);
    *avoengine::Is_scene_changed.lock().unwrap() = true;
}

    
    let mut camera_speed:f32 = 0.58;
    let mut camera_angle_speed: f32 = 5.8; 
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

            let keys = avoengine::window_processing::get_pressed_keys(&window);

            let mut camera = Camera.lock().unwrap();

            avoengine::Light_queue.lock().unwrap().push(
                        avoengine::Light_components{
                            light_x: camera.camera_x.clone(),
                            light_y: camera.camera_y.clone(),
                            light_z: camera.camera_z.clone(),
                            light_RGB_color: [255,255,255],
                            light_distance: 580.0,
                            light_cone_angle: 29.0,
                            light_pitch: camera.camera_pitch.clone(),
                            light_yaw: camera.camera_yaw.clone(),
                            light_special_name: "none".to_string()
                        }
                    );
            drop(camera);

            if current_tick != last_tick.try_into().unwrap(){

                {
                    let mut camera = Camera.lock().unwrap();

                    let mut play_sound_3d = false;
                    
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
                            KeyCode::KeyV => {
                                avoengine::sound::Play_sound("data/voyager.wav", 1.0);
                            },
                            KeyCode::KeyC => {
                                play_sound_3d = true;
                            },
                            KeyCode::KeyR => {
                                avoengine::sound::Stop_all_sounds();
                            },
                            KeyCode::KeyH => {
                                avoengine::maps::Load_map("data/he_man.txt".to_string());
                            },
                            KeyCode::KeyQ => std::process::exit(0),
                            _ => {}
                        }
                    }
                    
                    drop(camera);
                    
                    if play_sound_3d {
                        avoengine::sound::Play_sound_3d(0.0, 1.0, 0.0, "data/voyager.wav", 1.0);
                    }
                }
            last_tick = current_tick;
            // avoengine::maps::Do_all_scripts();
            }

            avoengine::maps::Do_all_scripts();
            avoengine::console_rc_render::Render_image_to_console();
            
            let _ = window_processing::update_frame(&window);
            }
            _ => {}
        }
    });

    println!("Hello, world!");
}