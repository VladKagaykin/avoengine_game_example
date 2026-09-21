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
    let center = [5.0f32, 1.0, 5.0];
    let yaw = 45.0f32;
    let radius = 1.0f32;

    let aperture_radius = radius.max(1e-3f32);
    let surface_radius = aperture_radius * 1.6f32;
    let rings = 48usize;
    let segments = 144usize;
    let ior = 1.5f32;

    let d = (surface_radius * surface_radius - aperture_radius * aperture_radius).max(0.0f32);
    let sagitta = surface_radius - d.sqrt();

    let mut front_rings: Vec<Vec<[f32; 3]>> = Vec::new();

    for i in 1..=rings {
        let rho = aperture_radius * (i as f32 / rings as f32);
        let d = (surface_radius * surface_radius - rho * rho).max(0.0f32);
        let z = sagitta - surface_radius + d.sqrt();

        let mut ring = Vec::with_capacity(segments);

        for j in 0..segments {
            let theta = 2.0f32 * std::f32::consts::PI * (j as f32 / segments as f32);
            ring.push([
                rho * theta.cos(),
                rho * theta.sin(),
                z,
            ]);
        }

        front_rings.push(ring);
    }

    let front_center = [0.0f32, 0.0f32, sagitta];
    let mut triangles: Vec<[[f32; 3]; 3]> = Vec::new();

    {
        let first = &front_rings[0];

        for j in 0..segments {
            let j1 = (j + 1) % segments;
            triangles.push([front_center, first[j], first[j1]]);
        }
    }

    for i in 1..rings {
        let inner = &front_rings[i - 1];
        let outer = &front_rings[i];

        for j in 0..segments {
            let j1 = (j + 1) % segments;

            triangles.push([inner[j], outer[j], outer[j1]]);
            triangles.push([inner[j], outer[j1], inner[j1]]);
        }
    }

    let mut vertices: Vec<f32> = Vec::new();

    let rad = yaw * std::f32::consts::PI / 180.0f32;
    let syaw = rad.sin();
    let cyaw = rad.cos();

    for tri in triangles {
        let a = tri[0];
        let b = tri[1];
        let c = tri[2];

        let front = [a, b, c];

        for p in front.iter() {
            let x = p[0] * cyaw + p[2] * syaw;
            let y = p[1];
            let z = -p[0] * syaw + p[2] * cyaw;

            vertices.push(x);
            vertices.push(y);
            vertices.push(z);
        }

        let ma = [a[0], a[1], -a[2]];
        let mb = [b[0], b[1], -b[2]];
        let mc = [c[0], c[1], -c[2]];

        let back = [ma, mc, mb];

        for p in back.iter() {
            let x = p[0] * cyaw + p[2] * syaw;
            let y = p[1];
            let z = -p[0] * syaw + p[2] * cyaw;

            vertices.push(x);
            vertices.push(y);
            vertices.push(z);
        }
    }

    let mut scene = avoengine::Static_scene.lock().unwrap();

    // scene.push(avoengine::Draw_components {
    //     draw_type: "3d_object".to_string(),
    //     draw_x: center[0],
    //     draw_y: center[1],
    //     draw_z: center[2],
    //     draw_symbol: '.',
    //     draw_vertices: vertices,
    //     draw_RGBA_color: [200, 230, 255, 25],
    //     draw_texture_path: "none".to_string(),
    //     draw_uv_path: "".to_string(),
    //     special_properties: format!("refraction {:.3}", ior),
    //     draw_special_name: "convex_lens".to_string(),
    // });

    drop(scene);

    *avoengine::Is_scene_changed.lock().unwrap() = true;
}

    let (s_vertices, s_uvs, s_texture_path) = obj_loader::load_obj_and_texture("data/render");

    let obj_component = Draw_components {
        draw_type: "3d_object".to_string(),
        draw_x: 0.0,
        draw_y: 0.5,
        draw_z: 0.0,
        draw_symbol: '#',
        draw_vertices: s_vertices,
        draw_RGBA_color: [255, 255, 255, 255],
        draw_texture_path: s_texture_path,
        draw_uvs: s_uvs,
        pitch: 0.0,
        yaw: 0.0,
        roll: 0.0,
        special_properties: "".to_string(),
        draw_special_name: "".to_string()
    };

    Static_scene.lock().unwrap().push(obj_component);
    *Is_scene_changed.lock().unwrap() = true;    

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

            // avoengine::Light_queue.lock().unwrap().push(
            //             avoengine::Light_components{
            //                 light_x: camera.camera_x.clone(),
            //                 light_y: camera.camera_y.clone(),
            //                 light_z: camera.camera_z.clone(),
            //                 light_RGB_color: [255,255,255],
            //                 light_distance: 580.0,
            //                 light_cone_angle: 29.0,
            //                 light_pitch: camera.camera_pitch.clone(),
            //                 light_yaw: camera.camera_yaw.clone(),
            //                 light_special_name: "none".to_string()
            //             }
            //         );
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