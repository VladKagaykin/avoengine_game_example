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

fn raycast_scene(origin: [f32; 3], dir: [f32; 3]) -> Option<([f32; 3], [f32; 3])> {
    let static_result: Option<(f32, [f32; 3], [f32; 3])> = {
        let scene = avoengine::Static_scene.lock().unwrap();
        let mut best_t = f32::INFINITY;
        let mut best_hit: Option<([f32; 3], [f32; 3])> = None;

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
                        let e1 = sub(v1, v0);
                        let e2 = sub(v2, v0);
                        let nrm = cross(e1, e2);
                        let len = (nrm[0]*nrm[0] + nrm[1]*nrm[1] + nrm[2]*nrm[2]).sqrt();
                        let normal = if len > 1e-8 {
                            [nrm[0]/len, nrm[1]/len, nrm[2]/len]
                        } else {
                            [0.0, 1.0, 0.0]
                        };
                        let hp = [
                            origin[0] + dir[0]*t,
                            origin[1] + dir[1]*t,
                            origin[2] + dir[2]*t,
                        ];
                        best_hit = Some((hp, normal));
                    }
                }
                i += 9;
            }
        }

        best_hit.map(|(p, n)| (best_t, p, n))
    };

    let dynamic_result = avoengine::render::Raycast_dynamic(origin, dir);

    match (static_result, dynamic_result) {
        (Some((st, sp, sn)), Some((dt, dp, dn))) => {
            if st <= dt { Some((sp, sn)) } else { Some((dp, dn)) }
        }
        (Some((_, sp, sn)), None) => Some((sp, sn)),
        (None, Some((_, dp, dn))) => Some((dp, dn)),
        (None, None) => None,
    }
}

fn offset_toward_triangle(hit: [f32; 3], tri_centroid: [f32; 3]) -> [f32; 3] {
    let d = sub(tri_centroid, hit);
    let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if dl > 1e-6 {
        let k = 0.01 / dl;
        [hit[0] + d[0] * k, hit[1] + d[1] * k, hit[2] + d[2] * k]
    } else {
        hit
    }
}

const PLAYER_NAME: &str = "__player";
const PLAYER_HEIGHT: f32 = 1.5;
const WALK_FORCE: f32 = 7.5;
const JUMP_FORCE: f32 = 15.0;

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

    {
        let mut scene = avoengine::Static_scene.lock().unwrap();
        for x in (-10..=10).step_by(2) {
            for z in (-10..=10).step_by(2) {
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
                    pitch: 0.0,
                    yaw: 0.0,
                    roll: 0.0,
                    vertices: tile_vertices.clone(),
                    RGBA_color: color,
                    properties: HashMap::new(),
                    special_name: String::new(),
                });
            }
        }
        scene.push(avoengine::Draw_components {
            x: 7.0, y: 1.1, z: 7.0,
            pitch: 0.0, yaw: 0.0, roll: 0.0,
            vertices: cube_vertices.clone(),
            RGBA_color: [255, 0, 0, 255],
            properties: HashMap::new(),
            special_name: String::new(),
        });
        scene.push(avoengine::Draw_components {
            x: -9.0, y: 1.1, z: -9.0,
            pitch: 0.0, yaw: 0.0, roll: 0.0,
            vertices: cube_vertices.clone(),
            RGBA_color: [0, 0, 255, 255],
            properties: HashMap::new(),
            special_name: String::new(),
        });
        scene.push(avoengine::Draw_components {
            x: -5.0, y: 1.1, z: 5.0,
            pitch: 0.0, yaw: 0.0, roll: 0.0,
            vertices: cube_vertices.clone(),
            RGBA_color: [255, 0, 255, 128],
            properties: HashMap::new(),
            special_name: String::new(),
        });
    }

    {
        let mut physics = avoengine::Physics_scene.lock().unwrap();
        let mut floor_props = HashMap::new();
        floor_props.insert("collision".to_string(), "1".to_string());
        floor_props.insert("resistance".to_string(), "1".to_string());
        physics.push(avoengine::Physics_components {
            x: 0.0, y: 0.0, z: 0.0,
            pitch: 0.0, yaw: 0.0, roll: 0.0,
            vertices: vec![
                 11.0, 0.0,  11.0,
                 11.0, 0.0, -11.0,
                -11.0, 0.0, -11.0,
                 11.0, 0.0,  11.0,
                -11.0, 0.0, -11.0,
                -11.0, 0.0,  11.0,
            ],
            properties: floor_props,
            special_name: "floor".to_string(),
        });
        for (px, py, pz, name) in [
            (7.0f32, 1.0f32, 7.0f32, "cube_red"),
            (-9.0, 1.0, -9.0, "cube_blue"),
            (-5.0, 1.0, 5.0, "cube_magenta"),
        ] {
            let mut props = HashMap::new();
            props.insert("collision".to_string(), "1".to_string());
            physics.push(avoengine::Physics_components {
                x: px, y: py, z: pz,
                pitch: 0.0, yaw: 0.0, roll: 0.0,
                vertices: cube_vertices.clone(),
                properties: props,
                special_name: name.to_string(),
            });
        }
    }

    match avoengine::gltf_parser::Import_gltf("data/mark(27_10_2026).gltf".to_string()) {
        Some(import) => {
            let mut dyn_obj = avoengine::render::Dynamic_components::new("mark".to_string());
            dyn_obj.x = 15.0;
            dyn_obj.y = 1.0;
            dyn_obj.z = 0.0;
            dyn_obj.Load_from_import(&import);
            dyn_obj.Set_default_animation("standart".to_string());
            avoengine::render::Draw_queue.lock().unwrap().push(dyn_obj);
            println!("[game] Imported model 'mark'");
        }
        None => println!("[game] Failed to import model data/mark(27_10_2026).gltf"),
    }

    avoengine::gui::GUI_SCENE.lock().unwrap().push(
        avoengine::gui::GuiElement::Text {
            template: "FPS: {fps}".to_string(),
            color: [255, 255, 255, 255],
            font: "proportional".to_string(),
            size: 15.0,
            anchor: avoengine::gui::Anchor::TopLeft,
            offset: (10.0, 10.0),
        },
    );
    avoengine::gui::GUI_OBJECTS
        .lock()
        .unwrap()
        .insert("fps".to_string(), "0.00".to_string());

    avoengine::render::Bake_static_scene();
    avoengine::physics::Bake_physics_scene();

    avoengine::window::Mouse_capture();

    {
        let mut player = avoengine::physics::Dynamic_physics_components::new(PLAYER_NAME.to_string());
        player.x = 0.0;
        player.y = 5.0;
        player.z = 0.0;
        player.properties.insert("collision".to_string(), "1".to_string());
        player.properties.insert("has_gravity".to_string(), "1".to_string());
        player.properties.insert("resistance".to_string(), "0.5".to_string());
        avoengine::physics::Physics_queue.lock().unwrap().push(player);
    }

    {
        let mut cam = Camera.lock().unwrap();
        cam.camera_x = 0.0;
        cam.camera_y = 5.0 + PLAYER_HEIGHT;
        cam.camera_z = 0.0;
    }

    let mut frame_count: u32 = 0;
    let mut last_time = Instant::now();
    let mut running = true;
    let mut noclip = true;
    let mut lmb_was_pressed = false;
    let mut last_tick = avoengine::tick_system::Get_current_tick();
    let mut t_was_pressed = false;
    let mut h_was_pressed = false;

    loop {
        let keys = avoengine::window::Get_pressed_keys();
        let mouse = avoengine::window::Get_mouse();

        if keys.iter().any(|k| *k == 20) {
            running = false;
        }

        {
            let mut cam = Camera.lock().unwrap();
            let sens = 0.15;
            cam.camera_yaw += mouse[2] as f32 * sens;
            cam.camera_pitch -= mouse[3] as f32 * sens;
        }

        {
            let rot_speed = 5.8;
            let mut cam = Camera.lock().unwrap();
            for key in &keys {
                match *key {
                    82 => cam.camera_pitch += rot_speed,
                    81 => cam.camera_pitch -= rot_speed,
                    80 => cam.camera_yaw -= rot_speed,
                    79 => cam.camera_yaw += rot_speed,
                    _ => {}
                }
            }
        }

        let lmb_pressed = (mouse[4] & 1) != 0;
        if lmb_pressed && !lmb_was_pressed {
            noclip = !noclip;
            let cam_pos = {
                let cam = Camera.lock().unwrap();
                [cam.camera_x, cam.camera_y, cam.camera_z]
            };
            let mut q = avoengine::physics::Physics_queue.lock().unwrap();
            if let Some(player) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                if noclip {
                    player.properties.insert("collision".to_string(), "0".to_string());
                    player.properties.insert("has_gravity".to_string(), "0".to_string());
                    player.Clear_forces();
                    player.x = cam_pos[0];
                    player.y = cam_pos[1] - PLAYER_HEIGHT;
                    player.z = cam_pos[2];
                } else {
                    player.properties.insert("collision".to_string(), "1".to_string());
                    player.properties.insert("has_gravity".to_string(), "1".to_string());
                    player.Clear_forces();
                }
            }
        }
        lmb_was_pressed = lmb_pressed;

        if noclip {
            let (cx, cy, cz) = {
                let mut cam = Camera.lock().unwrap();
                let speed = 0.58;
                let yaw_rad = cam.camera_yaw.to_radians();
                let pitch_rad = cam.camera_pitch.to_radians();
                let fwd_x = pitch_rad.cos() * yaw_rad.sin();
                let fwd_y = pitch_rad.sin();
                let fwd_z = pitch_rad.cos() * yaw_rad.cos();
                let right_x = yaw_rad.cos();
                let right_z = -yaw_rad.sin();
                for key in &keys {
                    match *key {
                        26 => { cam.camera_x += fwd_x * speed; cam.camera_y += fwd_y * speed; cam.camera_z += fwd_z * speed; }
                        22 => { cam.camera_x -= fwd_x * speed; cam.camera_y -= fwd_y * speed; cam.camera_z -= fwd_z * speed; }
                        4 => { cam.camera_x -= right_x * speed; cam.camera_z -= right_z * speed; }
                        7 => { cam.camera_x += right_x * speed; cam.camera_z += right_z * speed; }
                        _ => {}
                    }
                }
                (cam.camera_x, cam.camera_y, cam.camera_z)
            };
            let mut q = avoengine::physics::Physics_queue.lock().unwrap();
            if let Some(player) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                player.x = cx;
                player.y = cy - PLAYER_HEIGHT;
                player.z = cz;
                player.Clear_forces();
            }
        } else {
            let current_tick = avoengine::tick_system::Get_current_tick();
            if current_tick != last_tick {
                last_tick = current_tick;
                let yaw_rad = Camera.lock().unwrap().camera_yaw.to_radians();
                let fwd = [yaw_rad.sin(), 0.0, yaw_rad.cos()];
                let right = [yaw_rad.cos(), 0.0, -yaw_rad.sin()];
                let mut q = avoengine::physics::Physics_queue.lock().unwrap();
                if let Some(player) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                    for key in &keys {
                        match *key {
                            26 => player.Add_force([fwd[0] * WALK_FORCE, 0.0, fwd[2] * WALK_FORCE]),
                            22 => player.Add_force([-fwd[0] * WALK_FORCE, 0.0, -fwd[2] * WALK_FORCE]),
                            4 => player.Add_force([-right[0] * WALK_FORCE, 0.0, -right[2] * WALK_FORCE]),
                            7 => player.Add_force([right[0] * WALK_FORCE, 0.0, right[2] * WALK_FORCE]),
                            _ => {}
                        }
                    }
                }
            }

            if mouse[5] > 0 {
                let has_contact = {
                    let q = avoengine::physics::Physics_queue.lock().unwrap();
                    q.iter()
                        .find(|p| p.special_name == PLAYER_NAME)
                        .map(|p| !p.Get_impacts().is_empty())
                        .unwrap_or(false)
                };
                if has_contact {
                    let mut q = avoengine::physics::Physics_queue.lock().unwrap();
                    if let Some(player) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                        player.Add_force([0.0, JUMP_FORCE, 0.0]);
                    }
                }
            }
        }

        physics::Update_physics();

        if !noclip {
            let ppos = {
                let q = avoengine::physics::Physics_queue.lock().unwrap();
                q.iter().find(|p| p.special_name == PLAYER_NAME).map(|p| (p.x, p.y, p.z))
            };
            if let Some((px, py, pz)) = ppos {
                let mut cam = Camera.lock().unwrap();
                cam.camera_x = px;
                cam.camera_y = py + PLAYER_HEIGHT;
                cam.camera_z = pz;
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
                Some((hit, normal)) => {
                    let hit = offset_toward_triangle(hit, [hit[0] - normal[0], hit[1] - normal[1], hit[2] - normal[2]]);
                    avoengine::render::Add_spray_at(
                        "avoengine_rust/src/logo.png".to_string(),
                        hit,
                        normal,
                        [255, 255, 255, 255],
                    );
                }
                None => println!("[game spray] Raycast missed"),
            }
        }
        t_was_pressed = t_pressed;

        let h_pressed = keys.iter().any(|k| *k == 11);
        if h_pressed && !h_was_pressed {
            let mut queue = avoengine::render::Draw_queue.lock().unwrap();
            if let Some(obj) = queue.iter_mut().find(|o| o.special_name == "mark") {
                obj.Play_animation("head".to_string());
            }
        }
        h_was_pressed = h_pressed;

        avoengine::render::Render_frame();
        window::Update_frame();

        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_millis(1000) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            avoengine::gui::GUI_OBJECTS
                .lock()
                .unwrap()
                .insert("fps".to_string(), format!("{:.2}", fps));
            frame_count = 0;
            last_time = Instant::now();
        }

        if !running {
            break;
        }
    }

    println!("Hello, world!");
}