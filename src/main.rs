use avoengine::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    avoengine::Engine_setup();

    let tile_vertices: Vec<f32> = vec![
        1.0, 0.0,  1.0,
        1.0, 0.0, -1.0,
        -1.0, 0.0, -1.0,
        1.0, 0.0,  1.0,
        -1.0, 0.0, -1.0,
        -1.0, 0.0,  1.0,
    ];

    let mut scene = avoengine::Static_scene.lock().unwrap();
    for x in (-10..=8).step_by(2) {
        for z in (-10..=8).step_by(2) {
            let ix = (x + 10) / 2;
            let iz = (z + 10) / 2;
            let color = if (ix + iz) % 2 == 0 {
                [58u8, 58, 58, 255]
            } else {
                [3u8, 78, 37, 255]
            };
            scene.push(avoengine::Draw_components {
                x: x as f32,
                y: 0.0,
                z: z as f32,
                vertices: tile_vertices.clone(),
                RGBA_color: color,
                properties: std::collections::HashMap::new(),
                special_name: String::new(),
            });
        }
    }

    let cube_vertices: Vec<f32> = vec![
        -1.0, -1.0,  1.0,   1.0, -1.0,  1.0,   1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,   1.0,  1.0,  1.0,  -1.0,  1.0,  1.0,
        -1.0, -1.0, -1.0,  -1.0,  1.0, -1.0,   1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,   1.0,  1.0, -1.0,   1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,  -1.0, -1.0,  1.0,  -1.0,  1.0,  1.0,
        -1.0, -1.0, -1.0,  -1.0,  1.0,  1.0,  -1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,   1.0,  1.0, -1.0,   1.0,  1.0,  1.0,
        1.0, -1.0, -1.0,   1.0,  1.0,  1.0,   1.0, -1.0,  1.0,
        -1.0,  1.0, -1.0,  -1.0,  1.0,  1.0,   1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,   1.0,  1.0,  1.0,   1.0,  1.0, -1.0,
        -1.0, -1.0, -1.0,   1.0, -1.0, -1.0,   1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,   1.0, -1.0,  1.0,  -1.0, -1.0,  1.0,
    ];

    scene.push(avoengine::Draw_components {
        x: 7.0, y: 1.01, z: 7.0,
        vertices: cube_vertices.clone(),
        RGBA_color: [255, 0, 0, 255],
        properties: std::collections::HashMap::new(),
        special_name: String::new(),
    });
    scene.push(avoengine::Draw_components {
        x: -9.0, y: 1.01, z: -9.0,
        vertices: cube_vertices.clone(),
        RGBA_color: [0, 0, 255, 255],
        properties: std::collections::HashMap::new(),
        special_name: String::new(),
    });
    scene.push(avoengine::Draw_components {
        x: -5.0, y: 1.01, z: 5.0,
        vertices: cube_vertices.clone(),
        RGBA_color: [255, 0, 255, 128],
        properties: std::collections::HashMap::new(),
        special_name: String::new(),
    });

    drop(scene);

    avoengine::render::Bake_static_scene();

    let mut frame_count: u32 = 0;
    let mut last_time = Instant::now();

    let mut uzhas = true;

    loop {
        {
            let mut cam = Camera.lock().unwrap();
            let keys = avoengine::window::Get_pressed_keys();

            let speed = 0.2;
            let rot_speed = 2.0;
            let yaw_rad = cam.camera_yaw.to_radians();
            let pitch_rad = cam.camera_pitch.to_radians();

            let fwd_x = pitch_rad.cos() * yaw_rad.sin();
            let fwd_y = pitch_rad.sin();
            let fwd_z = pitch_rad.cos() * yaw_rad.cos();
            let right_x = yaw_rad.cos();
            let right_z = -yaw_rad.sin();

            for key in &keys {
                match *key {
                    26 => {
                        cam.camera_x += fwd_x * speed;
                        cam.camera_y += fwd_y * speed;
                        cam.camera_z += fwd_z * speed;
                    }
                    22 => {
                        cam.camera_x -= fwd_x * speed;
                        cam.camera_y -= fwd_y * speed;
                        cam.camera_z -= fwd_z * speed;
                    }
                    4 => {
                        cam.camera_x -= right_x * speed;
                        cam.camera_z -= right_z * speed;
                    }
                    7 => {
                        cam.camera_x += right_x * speed;
                        cam.camera_z += right_z * speed;
                    }
                    82 => {
                        cam.camera_pitch += rot_speed;
                        if cam.camera_pitch > 89.0 { cam.camera_pitch = 89.0; }
                    }
                    81 => {
                        cam.camera_pitch -= rot_speed;
                        if cam.camera_pitch < -89.0 { cam.camera_pitch = -89.0; }
                    }
                    80 => { cam.camera_yaw -= rot_speed; }
                    79 => { cam.camera_yaw += rot_speed; }
                    20 => { uzhas = false; }
                    _ => {}
                }
            }
        }
        avoengine::render::Render_frame();
        window::Update_frame();

        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_millis(1000) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            last_time = Instant::now();
        }
        if !uzhas {
            break;
        }
    }

    println!("Hello, world!");
}