use avoengine::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn ray_tri(
    origin: [f32; 3],
    dir: [f32; 3],
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
) -> Option<f32> {
    let e1 = sub(v1, v0);
    let e2 = sub(v2, v0);
    let h = cross(dir, e2);
    let a = dot(e1, h);
    if a.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / a;
    let s = sub(origin, v0);
    let u = f * dot(s, h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross(s, e1);
    let v = f * dot(dir, q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * dot(e2, q);
    if t > 1e-4 { Some(t) } else { None }
}

fn raycast_scene(origin: [f32; 3], dir: [f32; 3]) -> Option<[f32; 3]> {
    let scene = avoengine::Static_scene.lock().unwrap();
    let mut best_t = f32::INFINITY;
    let mut best_tri: Option<([f32; 3], [f32; 3], [f32; 3])> = None;

    for obj in scene.iter() {
        let n = obj.vertices.len();
        let mut i = 0;
        while i + 8 < n {
            let v0 = [
                obj.vertices[i]     + obj.x,
                obj.vertices[i + 1] + obj.y,
                obj.vertices[i + 2] + obj.z,
            ];
            let v1 = [
                obj.vertices[i + 3] + obj.x,
                obj.vertices[i + 4] + obj.y,
                obj.vertices[i + 5] + obj.z,
            ];
            let v2 = [
                obj.vertices[i + 6] + obj.x,
                obj.vertices[i + 7] + obj.y,
                obj.vertices[i + 8] + obj.z,
            ];
            if let Some(t) = ray_tri(origin, dir, v0, v1, v2) {
                if t < best_t {
                    best_t = t;
                    best_tri = Some((v0, v1, v2));
                }
            }
            i += 9;
        }
    }

    let (v0, v1, v2) = best_tri?;

    let hp = [
        origin[0] + dir[0] * best_t,
        origin[1] + dir[1] * best_t,
        origin[2] + dir[2] * best_t,
    ];

    let c = [
        (v0[0] + v1[0] + v2[0]) / 3.0,
        (v0[1] + v1[1] + v2[1]) / 3.0,
        (v0[2] + v1[2] + v2[2]) / 3.0,
    ];
    let d = sub(c, hp);
    let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if dl > 1e-6 {
        let k = 0.01 / dl;
        Some([hp[0] + d[0] * k, hp[1] + d[1] * k, hp[2] + d[2] * k])
    } else {
        Some(hp)
    }
}

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

    avoengine::gui::GUI_SCENE.lock().unwrap().push(
        avoengine::gui::GuiElement::Text {
            template: "FPS: {fps}".to_string(),
            color: [255, 255, 255, 255],
            font: "proportional".to_string(),
            size: 20.0,
            anchor: avoengine::gui::Anchor::TopLeft,
            offset: (10.0, 10.0),
        },
    );
    avoengine::gui::GUI_OBJECTS
        .lock()
        .unwrap()
        .insert("fps".to_string(), "0.00".to_string());

    avoengine::render::Bake_static_scene();

    let mut frame_count: u32 = 0;
    let mut last_time = Instant::now();
    let mut uzhas = true;
    let mut t_was_pressed = false;

    loop {
        let keys = avoengine::window::Get_pressed_keys();

        {
            let mut cam = Camera.lock().unwrap();

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
                    82 => { cam.camera_pitch += rot_speed; }
                    81 => { cam.camera_pitch -= rot_speed; }
                    80 => { cam.camera_yaw -= rot_speed; }
                    79 => { cam.camera_yaw += rot_speed; }
                    20 => { uzhas = false; }
                    _ => {}
                }
            }
        }

        let t_pressed = keys.iter().any(|k| *k == 23);
        if t_pressed && !t_was_pressed {
            let (position, forward) = {
                let cam = Camera.lock().unwrap();
                let yaw = cam.camera_yaw.to_radians();
                let pitch = cam.camera_pitch.to_radians();
                (
                    [cam.camera_x, cam.camera_y, cam.camera_z],
                    [
                        pitch.cos() * yaw.sin(),
                        pitch.sin(),
                        pitch.cos() * yaw.cos(),
                    ],
                )
            };

            match raycast_scene(position, forward) {
                Some(hit) => {
                    avoengine::render::Add_spray(
                        "avoengine_rust/src/logo.png".to_string(),
                        hit,
                        [255, 255, 255, 255],
                    );
                }
                None => println!("[game spray] Raycast missed"),
            }
        }
        t_was_pressed = t_pressed;

        avoengine::render::Render_frame();
        window::Update_frame();

        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_millis(1000) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            // println!("FPS: {:.2}", fps);
            avoengine::gui::GUI_OBJECTS
                .lock()
                .unwrap()
                .insert("fps".to_string(), format!("{:.2}", fps));
            frame_count = 0;
            last_time = Instant::now();
        }

        if !uzhas {
            break;
        }
    }

    println!("Hello, world!");
}