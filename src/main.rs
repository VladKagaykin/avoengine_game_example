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
    
    avoengine::maps::Load_map("data/default_map.txt".to_string());
    // avoengine::maps::Load_map("data/boos.txt".to_string());

    
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

            if current_tick != last_tick.try_into().unwrap(){

                {
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
                            KeyCode::KeyH => {
                                avoengine::maps::Load_map("data/he_man.txt".to_string());
                            },
                            KeyCode::KeyQ => std::process::exit(0),
                            _ => {}
                        }
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