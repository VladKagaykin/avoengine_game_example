use avoengine::*;
use std::collections::HashMap;
use std::sync::Arc;
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
    if a.abs() < 1e-8 { return None; }
    let f = 1.0 / a;
    let s = sub(origin, v0);
    let u = f * dot(s, h);
    if !(0.0..=1.0).contains(&u) { return None; }
    let q = cross(s, e1);
    let v = f * dot(dir, q);
    if v < 0.0 || u + v > 1.0 { return None; }
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
                let v0 = [obj.vertices[i] + obj.x, obj.vertices[i+1] + obj.y, obj.vertices[i+2] + obj.z];
                let v1 = [obj.vertices[i+3] + obj.x, obj.vertices[i+4] + obj.y, obj.vertices[i+5] + obj.z];
                let v2 = [obj.vertices[i+6] + obj.x, obj.vertices[i+7] + obj.y, obj.vertices[i+8] + obj.z];
                if let Some(t) = ray_tri(origin, dir, v0, v1, v2) {
                    if t < best_t {
                        best_t = t;
                        let e1 = sub(v1, v0);
                        let e2 = sub(v2, v0);
                        let nrm = cross(e1, e2);
                        let len = (nrm[0]*nrm[0] + nrm[1]*nrm[1] + nrm[2]*nrm[2]).sqrt();
                        let normal = if len > 1e-8 { [nrm[0]/len, nrm[1]/len, nrm[2]/len] } else { [0.0, 1.0, 0.0] };
                        let hp = [origin[0] + dir[0]*t, origin[1] + dir[1]*t, origin[2] + dir[2]*t];
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
        (Some((st, sp, sn)), Some((dt, dp, dn))) => if st <= dt { Some((sp, sn)) } else { Some((dp, dn)) },
        (Some((_, sp, sn)), None) => Some((sp, sn)),
        (None, Some((_, dp, dn))) => Some((dp, dn)),
        (None, None) => None,
    }
}

fn offset_toward_triangle(hit: [f32; 3], tri_centroid: [f32; 3]) -> [f32; 3] {
    let d = sub(tri_centroid, hit);
    let dl = (d[0]*d[0] + d[1]*d[1] + d[2]*d[2]).sqrt();
    if dl > 1e-6 {
        let k = 0.01 / dl;
        [hit[0] + d[0]*k, hit[1] + d[1]*k, hit[2] + d[2]*k]
    } else { hit }
}

const PLAYER_NAME: &str = "__player";
const MODEL_NAME: &str = "mark";
const PLAYER_HEIGHT: f32 = 1.9;
const PLAYER_WIDTH: f32 = 0.5;
const PLAYER_LENGTH: f32 = 0.5;
const CAMERA_FORWARD_OFFSET: f32 = 0.5;
const WALK_FORCE: f32 = 7.5;
const JUMP_FORCE: f32 = 300.0;
const SPEED_THRESHOLD: f32 = 0.05;
const EXPLOSION_FORCE: f32 = 800.0;

fn main() {
    avoengine::Engine_setup();

    let tile_vertices: Vec<f32> = vec![
        1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, -1.0,
        1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0, 1.0,
    ];
    let cube_vertices: Vec<f32> = vec![
        -1.0,-1.0, 1.0,  1.0,-1.0, 1.0,  1.0, 1.0, 1.0,
        -1.0,-1.0, 1.0,  1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        -1.0,-1.0,-1.0, -1.0, 1.0,-1.0,  1.0, 1.0,-1.0,
        -1.0,-1.0,-1.0,  1.0, 1.0,-1.0,  1.0,-1.0,-1.0,
        -1.0,-1.0,-1.0, -1.0,-1.0, 1.0, -1.0, 1.0, 1.0,
        -1.0,-1.0,-1.0, -1.0, 1.0, 1.0, -1.0, 1.0,-1.0,
         1.0,-1.0,-1.0,  1.0, 1.0,-1.0,  1.0, 1.0, 1.0,
         1.0,-1.0,-1.0,  1.0, 1.0, 1.0,  1.0,-1.0, 1.0,
        -1.0, 1.0,-1.0, -1.0, 1.0, 1.0,  1.0, 1.0, 1.0,
        -1.0, 1.0,-1.0,  1.0, 1.0, 1.0,  1.0, 1.0,-1.0,
        -1.0,-1.0,-1.0,  1.0,-1.0,-1.0,  1.0,-1.0, 1.0,
        -1.0,-1.0,-1.0,  1.0,-1.0, 1.0, -1.0,-1.0, 1.0,
    ];

    let hw = PLAYER_WIDTH * 0.5;
    let hl = PLAYER_LENGTH * 0.5;
    let player_vertices: Vec<f32> = vec![
        -hw, 0.0,          -hl,
         hw, 0.0,          -hl,
         hw, 0.0,           hl,
        -hw, 0.0,           hl,
        -hw, PLAYER_HEIGHT, -hl,
         hw, PLAYER_HEIGHT, -hl,
         hw, PLAYER_HEIGHT,  hl,
        -hw, PLAYER_HEIGHT,  hl,
    ];

    {
        let mut scene = avoengine::Static_scene.lock().unwrap();
        for x in (-10..=10).step_by(2) {
            for z in (-10..=10).step_by(2) {
                let ix = (x + 10) / 2;
                let iz = (z + 10) / 2;
                let color = if (ix + iz) % 2 == 0 { [58u8,58,58,255] } else { [3u8,78,37,255] };
                scene.push(avoengine::Draw_components {
                    x: x as f32, y: 0.0, z: z as f32,
                    pitch: 0.0, yaw: 0.0, roll: 0.0,
                    vertices: tile_vertices.clone(), RGBA_color: color,
                    properties: HashMap::new(), special_name: String::new(),
                });
            }
        }
        for (px,py,pz,col) in [(7.0,1.1,7.0,[255,0,0,255]), (-9.0,1.1,-9.0,[0,0,255,255]), (-5.0,1.1,5.0,[255,0,255,128])] {
            scene.push(avoengine::Draw_components {
                x: px, y: py, z: pz, pitch: 0.0, yaw: 0.0, roll: 0.0,
                vertices: cube_vertices.clone(), RGBA_color: col,
                properties: HashMap::new(), special_name: String::new(),
            });
        }
    }

    {
        let mut physics = avoengine::Physics_scene.lock().unwrap();
        let mut fp = HashMap::new();
        fp.insert("collision".to_string(), "1".to_string());
        fp.insert("resistance".to_string(), "1.0".to_string());
        fp.insert("soundproofing".to_string(), "0.9".to_string());
        physics.push(avoengine::Physics_components {
            x:0.0,y:0.0,z:0.0, pitch:0.0,yaw:0.0,roll:0.0,
            vertices: vec![110.0,0.0,110.0, 110.0,0.0,-110.0, -110.0,0.0,-110.0, 110.0,0.0,110.0, -110.0,0.0,-110.0, -110.0,0.0,110.0],
            properties: fp, special_name: "floor".to_string(),
        });
        for (px,py,pz,name) in [(7.0,1.0,7.0,"cube_red"),(-9.0,1.0,-9.0,"cube_blue"),(-5.0,1.0,5.0,"cube_magenta")] {
            let mut props = HashMap::new(); props.insert("collision".to_string(),"1".to_string());
            physics.push(avoengine::Physics_components {
                x:px,y:py,z:pz, pitch:0.0,yaw:0.0,roll:0.0,
                vertices: cube_vertices.clone(), properties: props, special_name: name.to_string(),
            });
        }
    }

    avoengine::gui::GUI_SCENE.lock().unwrap().push(avoengine::gui::GuiElement::Text { template:"FPS: {fps}".into(), color:[255,255,255,255], font:"proportional".into(), size:15.0, anchor:avoengine::gui::Anchor::TopLeft, offset:(10.0,10.0) });
    avoengine::gui::GUI_SCENE.lock().unwrap().push(avoengine::gui::GuiElement::Text { template:"X:{cam_x} Y:{cam_y} Z:{cam_z}".into(), color:[255,255,255,255], font:"proportional".into(), size:15.0, anchor:avoengine::gui::Anchor::TopLeft, offset:(10.0,32.0) });
    avoengine::gui::GUI_SCENE.lock().unwrap().push(avoengine::gui::GuiElement::Text { template:"Yaw:{yaw} Pitch:{pitch}".into(), color:[255,255,255,255], font:"proportional".into(), size:15.0, anchor:avoengine::gui::Anchor::TopLeft, offset:(10.0,54.0) });
    avoengine::gui::GUI_SCENE.lock().unwrap().push(avoengine::gui::GuiElement::Text { template:"{player_speed}".into(), color:[255,255,255,255], font:"proportional".into(), size:15.0, anchor:avoengine::gui::Anchor::TopRight, offset:(-10.0,10.0) });
    {
        let mut o = avoengine::gui::GUI_OBJECTS.lock().unwrap();
        o.insert("fps".into(),"0".into()); o.insert("cam_x".into(),"0".into()); o.insert("cam_y".into(),"0".into());
        o.insert("cam_z".into(),"0".into()); o.insert("yaw".into(),"0".into()); o.insert("pitch".into(),"0".into());
        o.insert("player_speed".into(),"0".into());
    }

    avoengine::render::Bake_static_scene();
    {
        let mut lm = avoengine::Pseudo_light_map.lock().unwrap();
        lm.push(avoengine::Draw_components {
            x:10.0,y:0.0,z:0.0, pitch:0.0,yaw:0.0,roll:0.0,
            vertices: vec![-10.0,-10.0,10.0, 10.0,-10.0,10.0, 10.0,10.0,10.0, -10.0,-10.0,10.0, 10.0,10.0,10.0, -10.0,10.0,10.0, -10.0,-10.0,-10.0, -10.0,10.0,-10.0, 10.0,10.0,-10.0, -10.0,-10.0,-10.0, 10.0,10.0,-10.0, 10.0,-10.0,-10.0, -10.0,-10.0,-10.0, -10.0,-10.0,10.0, -10.0,10.0,10.0, -10.0,-10.0,-10.0, -10.0,10.0,10.0, -10.0,10.0,-10.0, 10.0,-10.0,-10.0, 10.0,10.0,-10.0, 10.0,10.0,10.0, 10.0,-10.0,-10.0, 10.0,10.0,10.0, 10.0,-10.0,10.0, -10.0,10.0,-10.0, -10.0,10.0,10.0, 10.0,10.0,10.0, -10.0,10.0,-10.0, 10.0,10.0,10.0, 10.0,10.0,-10.0, -10.0,-10.0,-10.0, 10.0,-10.0,-10.0, 10.0,-10.0,10.0, -10.0,-10.0,-10.0, 10.0,-10.0,10.0, -10.0,-10.0,10.0],
            RGBA_color:[255,255,255,255], properties:HashMap::new(), special_name:String::new(),
        });
    }
    avoengine::physics::Bake_physics_scene();
    avoengine::window::Mouse_capture();

    {
        let mut p = avoengine::physics::Dynamic_physics_components::new(PLAYER_NAME.to_string());
        p.x=0.0; p.y=5.0; p.z=0.0;
        p.vertices = player_vertices.clone();
        p.properties.insert("collision".into(),"1".into());
        p.properties.insert("has_gravity".into(),"1".into());
        p.properties.insert("resistance".into(),"0.5".into());
        p.properties.insert("mass".into(),"7.5".into());
        avoengine::physics::Physics_queue.lock().unwrap().push(p);
    }

    if let Some(ir) = avoengine::gltf_parser::Import_gltf("data/mark(31_10_2026).gltf".to_string()) {
        let mut m = avoengine::render::Dynamic_components::new(MODEL_NAME.to_string());
        m.Load_from_import(&ir);
        m.Set_default_animation("default".to_string());
        avoengine::render::Draw_queue.lock().unwrap().push(m);
    }

    let mut office_triangles: Vec<(usize, usize, [f32; 3])> = Vec::new();
    let office_import_point: [f32; 3] = [0.0, 0.0, 0.0];

    if let Some(full_import) = avoengine::gltf_parser::Import_gltf("data/office.gltf".to_string()) {
        let num_tris = full_import.vertices.len() / 9;
        if num_tris > 0 {
            let mut dq = avoengine::render::Draw_queue.lock().unwrap();
            let mut pq = avoengine::physics::Physics_queue.lock().unwrap();
            let ds = dq.len();
            let ps = pq.len();
            office_triangles.reserve(num_tris);

            for i in 0..num_tris {
                let vs = i * 9;
                let us = i * 6;
                let js = i * 3;
                
                let tri_verts = full_import.vertices[vs..vs+9].to_vec();
                let tri_uvs = if us+6 <= full_import.uvs.len() { full_import.uvs[us..us+6].to_vec() } else { vec![0.0;6] };
                let tri_ji = if js+3 <= full_import.joint_indices.len() { full_import.joint_indices[js..js+3].to_vec() } else { vec![[0,0,0,0];3] };
                let tri_jw = if js+3 <= full_import.joint_weights.len() { full_import.joint_weights[js..js+3].to_vec() } else { vec![[1.0,0.0,0.0,0.0];3] };

                let uv_str = format!("{},{} {},{} {},{}", tri_uvs[0],tri_uvs[1], tri_uvs[2],tri_uvs[3], tri_uvs[4],tri_uvs[5]);
                
                let pname = format!("office_tri_{}", i);
                let mut phys = avoengine::physics::Dynamic_physics_components::new(pname.clone());
                phys.x = office_import_point[0]; phys.y = office_import_point[1]; phys.z = office_import_point[2];
                phys.vertices = tri_verts.clone();
                phys.properties.insert("collision".into(), "1".into());
                phys.properties.insert("has_gravity".into(), "0".into());
                phys.properties.insert("mass".into(), "1.0".into());
                phys.properties.insert("resistance".into(), "0.05".into());
                phys.properties.insert("bounce".into(), "0.3".into());
                phys.properties.insert("uv".into(), uv_str.clone());
                phys.properties.insert("texture".into(), full_import.texture_path.clone());
                pq.push(phys);

                let mini_result = avoengine::gltf_parser::Import_result {
                    texture_path: full_import.texture_path.clone(),
                    uv: uv_str,
                    uvs: tri_uvs,
                    vertices: tri_verts.clone(),
                    joint_indices: tri_ji,
                    joint_weights: tri_jw,
                    bone_names: vec![],
                    animation_names: vec![],
                    skeleton: Arc::new(avoengine::gltf_parser::Skeleton_data { bones: vec![], animations: HashMap::new() }),
                };

                let mut draw = avoengine::render::Dynamic_components::new(format!("{}_draw", pname));
                draw.Load_from_import(&mini_result);
                draw.x = office_import_point[0]; draw.y = office_import_point[1]; draw.z = office_import_point[2];
                dq.push(draw);

                let center = [
                    (tri_verts[0]+tri_verts[3]+tri_verts[6]) * 0.33333334,
                    (tri_verts[1]+tri_verts[4]+tri_verts[7]) * 0.33333334,
                    (tri_verts[2]+tri_verts[5]+tri_verts[8]) * 0.33333334,
                ];
                office_triangles.push((ds + i, ps + i, center));
            }
            println!("[game] Imported {} office triangles", num_tris);
        }
    }

    { let mut c = Camera.lock().unwrap(); c.camera_x=0.0; c.camera_y=5.0+PLAYER_HEIGHT; c.camera_z=0.0; }

    let mut frame_count: u32 = 0;
    let mut last_time = Instant::now();
    let mut running = true;
    let mut noclip = true;
    let mut lmb_was_pressed = false;
    let mut last_tick = avoengine::tick_system::Get_current_tick();
    let mut t_was_pressed = false;
    let mut v_was_pressed = false;
    let mut c_was_pressed = false;
    let mut e_was_pressed = false;
    let mut b_was_pressed = false;
    let mut prev_player_pos: Option<[f32; 3]> = None;
    let mut prev_speed_time = Instant::now();
    let mut mouse_captured = true;
    let mut prev_on_ground: bool = false;
    let mut current_anim_state: String = "default".to_string();

    avoengine::render::Bake_dynamic_scene();

    loop {
        let keys = avoengine::window::Get_pressed_keys();
        let mouse = avoengine::window::Get_mouse();

        if keys.iter().any(|k| *k == 20) { running = false; }

        let e_pressed = keys.iter().any(|k| *k == 8);
        if e_pressed && !e_was_pressed {
            if mouse_captured { avoengine::window::Return_mouse(); mouse_captured = false; }
            else { avoengine::window::Mouse_capture(); mouse_captured = true; }
        }
        e_was_pressed = e_pressed;

        if mouse_captured {
            let mut cam = Camera.lock().unwrap();
            cam.camera_yaw += mouse[2] as f32 * 0.15;
            cam.camera_pitch -= mouse[3] as f32 * 0.15;
        }

        {
            let mut cam = Camera.lock().unwrap();
            for k in &keys { match *k { 82=>cam.camera_pitch+=5.8, 81=>cam.camera_pitch-=5.8, 80=>cam.camera_yaw-=5.8, 79=>cam.camera_yaw+=5.8, _=>{} } }
        }

        let lmb = (mouse[4] & 1) != 0;
        if lmb && !lmb_was_pressed && mouse_captured {
            noclip = !noclip;
            let cp = { let c=Camera.lock().unwrap(); [c.camera_x,c.camera_y,c.camera_z] };
            let mut q = avoengine::physics::Physics_queue.lock().unwrap();
            if let Some(p) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                if noclip {
                    p.properties.insert("collision".into(),"0".into()); p.properties.insert("has_gravity".into(),"0".into());
                    p.Clear_forces(); p.x=cp[0]; p.y=cp[1]-PLAYER_HEIGHT; p.z=cp[2];
                } else {
                    p.properties.insert("collision".into(),"1".into()); p.properties.insert("has_gravity".into(),"1".into());
                    p.Clear_forces();
                }
            }
        }
        lmb_was_pressed = lmb;

        if noclip {
            let (cx,cy,cz) = {
                let mut cam = Camera.lock().unwrap();
                let sp=0.58; let yr=cam.camera_yaw.to_radians(); let pr=cam.camera_pitch.to_radians();
                let fx=pr.cos()*yr.sin(); let fy=pr.sin(); let fz=pr.cos()*yr.cos();
                let rx=yr.cos(); let rz=-yr.sin();
                for k in &keys { match *k {
                    26=>{cam.camera_x+=fx*sp;cam.camera_y+=fy*sp;cam.camera_z+=fz*sp;}
                    22=>{cam.camera_x-=fx*sp;cam.camera_y-=fy*sp;cam.camera_z-=fz*sp;}
                    4=>{cam.camera_x-=rx*sp;cam.camera_z-=rz*sp;}
                    7=>{cam.camera_x+=rx*sp;cam.camera_z+=rz*sp;}
                    _=>{}
                }}
                (cam.camera_x,cam.camera_y,cam.camera_z)
            };
            let mut q = avoengine::physics::Physics_queue.lock().unwrap();
            if let Some(p) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) { p.x=cx; p.y=cy-PLAYER_HEIGHT; p.z=cz; p.Clear_forces(); }
        } else {
            let ct = avoengine::tick_system::Get_current_tick();
            if ct != last_tick {
                last_tick = ct;
                let yr = Camera.lock().unwrap().camera_yaw.to_radians();
                let fwd=[yr.sin(),0.0,yr.cos()]; let rgt=[yr.cos(),0.0,-yr.sin()];
                let mut q = avoengine::physics::Physics_queue.lock().unwrap();
                if let Some(p) = q.iter_mut().find(|p| p.special_name == PLAYER_NAME) {
                    for k in &keys { match *k {
                        26=>p.Add_force([fwd[0]*WALK_FORCE,0.0,fwd[2]*WALK_FORCE]),
                        22=>p.Add_force([-fwd[0]*WALK_FORCE,0.0,-fwd[2]*WALK_FORCE]),
                        4=>p.Add_force([-rgt[0]*WALK_FORCE,0.0,-rgt[2]*WALK_FORCE]),
                        7=>p.Add_force([rgt[0]*WALK_FORCE,0.0,rgt[2]*WALK_FORCE]),
                        _=>{}
                    }}
                }
            }
            if mouse[5] > 0 {
                let hc = { let q=avoengine::physics::Physics_queue.lock().unwrap(); q.iter().find(|p|p.special_name==PLAYER_NAME).map(|p|!p.Get_impacts().is_empty()).unwrap_or(false) };
                if hc { let mut q=avoengine::physics::Physics_queue.lock().unwrap(); if let Some(p)=q.iter_mut().find(|p|p.special_name==PLAYER_NAME){p.Add_force([0.0,JUMP_FORCE,0.0]);} }
            }
        }

        physics::Update_physics();
        avoengine::sound::Play_sound();

        if !office_triangles.is_empty() {
            let mut dq = avoengine::render::Draw_queue.lock().unwrap();
            let pq = avoengine::physics::Physics_queue.lock().unwrap();
            for (di, pi, _) in &office_triangles {
                if let (Some(d), Some(p)) = (dq.get_mut(*di), pq.get(*pi)) {
                    d.x=p.x; d.y=p.y; d.z=p.z; d.pitch=p.pitch; d.yaw=p.yaw; d.roll=p.roll;
                }
            }
        }

        let bp = keys.iter().any(|k| *k == 5);
        if bp && !b_was_pressed && !office_triangles.is_empty() {
            let mut pq = avoengine::physics::Physics_queue.lock().unwrap();
            for (_, pi, ctr) in &office_triangles {
                if let Some(p) = pq.get_mut(*pi) {
                    p.properties.insert("has_gravity".into(), "1".into());
                    let wx=p.x+ctr[0]; let wy=p.y+ctr[1]; let wz=p.z+ctr[2];
                    let dx=wx-office_import_point[0]; let dy=wy-office_import_point[1]; let dz=wz-office_import_point[2];
                    let dsq=dx*dx+dy*dy+dz*dz;
                    if dsq > 1e-6 {
                        let d=dsq.sqrt(); let fm=EXPLOSION_FORCE/(d+0.5); let id=1.0/d;
                        p.Add_force([dx*id*fm, dy*id*fm, dz*id*fm]);
                    } else { p.Add_force([0.0, EXPLOSION_FORCE*0.5, 0.0]); }
                }
            }
            println!("[game] Explosion triggered!");
        }
        b_was_pressed = bp;

        let ppn = { let q=avoengine::physics::Physics_queue.lock().unwrap(); q.iter().find(|p|p.special_name==PLAYER_NAME).map(|p|(p.x,p.y,p.z)) };
        let og = { let q=avoengine::physics::Physics_queue.lock().unwrap(); q.iter().find(|p|p.special_name==PLAYER_NAME).map(|p|!p.Get_impacts().is_empty()).unwrap_or(false) };
        let (cx,cy,cz,cyaw,cpitch) = { let c=Camera.lock().unwrap(); (c.camera_x,c.camera_y,c.camera_z,c.camera_yaw,c.camera_pitch) };
        
        let ppa: Option<[f32;3]> = ppn.map(|(x,y,z)|[x,y,z]);
        let now = Instant::now();
        let sv: Option<f32> = match (ppa, prev_player_pos) {
            (Some(c),Some(p)) => { let dx=c[0]-p[0]; let dy=c[1]-p[1]; let dz=c[2]-p[2]; Some((dx*dx+dy*dy+dz*dz).sqrt() / now.duration_since(prev_speed_time).as_secs_f32().max(1e-6)) }
            _ => None,
        };
        if ppa.is_some() { prev_player_pos=ppa; prev_speed_time=now; }
        let hv = sv.map(|s|s>SPEED_THRESHOLD).unwrap_or(false);

        if !noclip {
            if let Some((px,py,pz)) = ppn {
                let yr=cyaw.to_radians(); let pr=cpitch.to_radians();
                let fx=pr.cos()*yr.sin(); let fy=pr.sin(); let fz=pr.cos()*yr.cos();
                let mut c=Camera.lock().unwrap();
                c.camera_x=px+fx*CAMERA_FORWARD_OFFSET; c.camera_y=py+PLAYER_HEIGHT+fy*CAMERA_FORWARD_OFFSET; c.camera_z=pz+fz*CAMERA_FORWARD_OFFSET;
            }
        }

        {
            let mut q = avoengine::render::Draw_queue.lock().unwrap();
            if let Some(m) = q.iter_mut().find(|m|m.special_name==MODEL_NAME) {
                if let Some((px,py,pz)) = ppn { m.x=px; m.y=py; m.z=pz; }
                m.yaw = cyaw;
                if !noclip {
                    if og {
                        if !prev_on_ground { m.Stop_all_animations(); m.Play_animation("end_jump".into()); m.properties.insert("default_animation".into(),"default".into()); current_anim_state="end_jump".into(); }
                        else if hv { if current_anim_state!="walk" { m.Stop_all_animations(); m.properties.insert("default_animation".into(),"walk".into()); current_anim_state="walk".into(); } }
                        else { if current_anim_state!="default" && current_anim_state!="end_jump" { m.Stop_all_animations(); m.properties.insert("default_animation".into(),"default".into()); current_anim_state="default".into(); } else if current_anim_state=="end_jump" && m.animations_in_play.is_empty() { m.properties.insert("default_animation".into(),"default".into()); current_anim_state="default".into(); } }
                    } else {
                        if prev_on_ground { m.Stop_all_animations(); m.Play_animation("start_jump".into()); m.properties.insert("default_animation".into(),"jump_default".into()); current_anim_state="start_jump".into(); }
                        else { if current_anim_state=="start_jump" && !m.animations_in_play.contains(&"start_jump".into()) { current_anim_state="jump_default".into(); } if current_anim_state!="jump_default" && current_anim_state!="start_jump" { m.Stop_all_animations(); m.properties.insert("default_animation".into(),"jump_default".into()); current_anim_state="jump_default".into(); } }
                    }
                } else { if current_anim_state!="default" { m.Stop_all_animations(); m.properties.insert("default_animation".into(),"default".into()); current_anim_state="default".into(); } }
            }
        }
        prev_on_ground = og;

        {
            let mut o = avoengine::gui::GUI_OBJECTS.lock().unwrap();
            o.insert("cam_x".into(),format!("{:.2}",cx)); o.insert("cam_y".into(),format!("{:.2}",cy)); o.insert("cam_z".into(),format!("{:.2}",cz));
            o.insert("yaw".into(),format!("{:.2}",cyaw)); o.insert("pitch".into(),format!("{:.2}",cpitch));
            if let Some(s)=sv { o.insert("player_speed".into(),format!("{:.2}",s)); }
        }

        let tp = keys.iter().any(|k|*k==23);
        if tp && !t_was_pressed && mouse_captured {
            let (pos,fwd) = { let c=Camera.lock().unwrap(); let y=c.camera_yaw.to_radians(); let p=c.camera_pitch.to_radians(); ([c.camera_x,c.camera_y,c.camera_z],[p.cos()*y.sin(),p.sin(),p.cos()*y.cos()]) };
            if let Some((h,n)) = raycast_scene(pos,fwd) {
                let ho = offset_toward_triangle(h,[h[0]-n[0],h[1]-n[1],h[2]-n[2]]);
                avoengine::render::Add_spray_at("avoengine_rust/src/logo.png".into(),ho,n,[255,255,255,255]);
            }
        }
        t_was_pressed = tp;

        let vp = keys.iter().any(|k|*k==25);
        if vp && !v_was_pressed {
            let mut s = avoengine::Sounds.lock().unwrap(); s.retain(|x|x.special_name!="voyager");
            s.push(avoengine::Sound_components{path:"data/voyager.wav".into(),x:0.0,y:1.0,z:0.0,time:0,properties:HashMap::new(),special_name:"voyager".into()});
        }
        v_was_pressed = vp;

        let cp = keys.iter().any(|k|*k==6);
        if cp && !c_was_pressed { avoengine::Sounds.lock().unwrap().clear(); }
        c_was_pressed = cp;

        avoengine::render::Render_frame();
        window::Update_frame();

        frame_count += 1;
        let el = last_time.elapsed();
        if el >= Duration::from_millis(1000) {
            avoengine::gui::GUI_OBJECTS.lock().unwrap().insert("fps".into(),format!("{:.2}",frame_count as f64/el.as_secs_f64()));
            frame_count=0; last_time=Instant::now();
        }
        if !running { break; }
    }
    println!("Hello, world!");
}