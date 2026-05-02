#include "avoengine_opengl/avoengine.h"
#include "avoengine_opengl/avoextension.h"
#include <GL/glu.h>
#include <GLFW/glfw3.h>
#include <GL/glut.h>
#include <iostream>
#include <vector>
#include <filesystem>
#include <algorithm>

float pitch,yaw;

GLFWwindow* window = nullptr;
bool settings_mode = 0;
int stage = 0;
float turn_speed = 0.58;

std::vector<const char*> textures = {
    "src/radio/render_000_ring00_az000.png", "src/radio/render_001_ring00_az045.png",
    "src/radio/render_002_ring00_az090.png", "src/radio/render_003_ring00_az135.png",
    "src/radio/render_004_ring00_az180.png", "src/radio/render_005_ring00_az225.png",
    "src/radio/render_006_ring00_az270.png", "src/radio/render_007_ring00_az315.png",
    "src/radio/render_008_ring01_az000.png", "src/radio/render_009_ring01_az045.png",
    "src/radio/render_010_ring01_az090.png", "src/radio/render_011_ring01_az135.png",
    "src/radio/render_012_ring01_az180.png", "src/radio/render_013_ring01_az225.png",
    "src/radio/render_014_ring01_az270.png", "src/radio/render_015_ring01_az315.png",
    "src/radio/render_016_ring02_az000.png", "src/radio/render_017_ring02_az045.png",
    "src/radio/render_018_ring02_az090.png", "src/radio/render_019_ring02_az135.png",
    "src/radio/render_020_ring02_az180.png", "src/radio/render_021_ring02_az225.png",
    "src/radio/render_022_ring02_az270.png", "src/radio/render_023_ring02_az315.png",
    "src/radio/render_024_ring03_az000.png", "src/radio/render_025_ring03_az045.png",
    "src/radio/render_026_ring03_az090.png", "src/radio/render_027_ring03_az135.png",
    "src/radio/render_028_ring03_az180.png", "src/radio/render_029_ring03_az225.png",
    "src/radio/render_030_ring03_az270.png", "src/radio/render_031_ring03_az315.png",
    "src/radio/render_032_ring04_az000.png", "src/radio/render_033_ring04_az045.png",
    "src/radio/render_034_ring04_az090.png", "src/radio/render_035_ring04_az135.png",
    "src/radio/render_036_ring04_az180.png", "src/radio/render_037_ring04_az225.png",
    "src/radio/render_038_ring04_az270.png", "src/radio/render_039_ring04_az315.png",
    "src/radio/render_040_ring05_az000.png", "src/radio/render_041_ring05_az045.png",
    "src/radio/render_042_ring05_az090.png", "src/radio/render_043_ring05_az135.png",
    "src/radio/render_044_ring05_az180.png", "src/radio/render_045_ring05_az225.png",
    "src/radio/render_046_ring05_az270.png", "src/radio/render_047_ring05_az315.png",
    "src/radio/render_048_ring06_az000.png", "src/radio/render_049_ring06_az045.png",
    "src/radio/render_050_ring06_az090.png", "src/radio/render_051_ring06_az135.png",
    "src/radio/render_052_ring06_az180.png", "src/radio/render_053_ring06_az225.png",
    "src/radio/render_054_ring06_az270.png", "src/radio/render_055_ring06_az315.png",
    "src/radio/render_056_ring07_az000.png", "src/radio/render_057_ring07_az045.png",
    "src/radio/render_058_ring07_az090.png", "src/radio/render_059_ring07_az135.png",
    "src/radio/render_060_ring07_az180.png", "src/radio/render_061_ring07_az225.png",
    "src/radio/render_062_ring07_az270.png", "src/radio/render_063_ring07_az315.png"
};
float verts[] = { -0.5f, -0.5f, 0.5f, -0.5f, 0.5f, 0.5f, -0.5f, 0.5f };

std::vector<std::string> textures_str(textures.begin(), textures.end());
pseudo_3d_entity* radio = new pseudo_3d_entity(0, -0.5, 0, 0, 0, 0.0f, textures_str, 8, std::vector<float>(verts, verts + 8));

Light projector_1;
Light projector_2;
Light projector_3;
Light projector_4;
Light flashlight;
float verts_square[] = {-1,-1, 1,-1, 1,1, -1,1};

float edge = 10.0f;
float height = 5.0f;

bool map_menu_active = false;
std::vector<std::string> map_files;
int selected_map_index = 0;
std::vector<Light*> dynamicLights;

Portal* portals = nullptr;

void refresh_map_list() {
    map_files.clear();
    if (!std::filesystem::exists("maps")) {
        std::filesystem::create_directory("maps");
        return;
    }
    for (const auto& entry : std::filesystem::directory_iterator("maps")) {
        if (entry.is_regular_file() && entry.path().extension() == ".avomap") {
            map_files.push_back(entry.path().filename().string());
        }
    }
    selected_map_index = (map_files.empty() ? 0 : std::min(selected_map_index, (int)map_files.size()-1));
}

void apply_loaded_map(const MapData& map) {
    stop_all_looping_sounds();

    for (auto* e : allEntities) {
        e->setCastShadow(false);
        delete e;
    }
    allEntities.clear();

    projector_1.disable();
    projector_2.disable();
    projector_3.disable();
    projector_4.disable();
    flashlight.disable();

    for (Light* l : dynamicLights) {
        l->disable();
        delete l;
    }
    dynamicLights.clear();

    for (const auto& ent : map.entities) {
        pseudo_3d_entity* e = mapDataToEntity(ent);
        if (ent.castShadow) e->setCastShadow(true);
        registerEntity(e);
    }

    for (const auto& ldata : map.lights) {
        Light* newLight = new Light();
        mapDataToLight(ldata, *newLight);
        newLight->enable();
        dynamicLights.push_back(newLight);
    }

    if (map.fog_enabled) {
        enable_fog(map.fog_density, map.fog_color[0], map.fog_color[1], map.fog_color[2],
                   map.fog_start, map.fog_end);
    } else {
        disable_fog();
    }

    camera.eye_x = map.camera_eye[0];
    camera.eye_y = map.camera_eye[1];
    camera.eye_z = map.camera_eye[2];
    pitch = map.camera_pitch;
    yaw = map.camera_yaw;
    setup_camera(camera.fov, camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);

    if (!map.panorama_path.empty()) {
        set_panorama(map.panorama_path.c_str());
    } else {
        remove_panorama();
    }

    set_ambient_light(map.ambient[0], map.ambient[1], map.ambient[2]);
}

void quick_save() {
    if (!std::filesystem::exists("maps"))
        std::filesystem::create_directory("maps");
    save_current_scene("maps/quicksave.avomap");
    refresh_map_list();
}

void draw_map_menu() {
    begin_2d(window_w, window_h);

    draw_text("~ Map Menu ~", 20.0f, float(window_h) - 30.0f,
              GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 0.0f);
    draw_text("S: Save (quicksave.avomap)   Enter: Load   Esc: Close",
              20.0f, 20.0f, GLUT_BITMAP_HELVETICA_12, 0.8f, 0.8f, 0.8f);

    if (map_files.empty()) {
        draw_text("(no maps in maps/ folder)", 20.0f, float(window_h) - 60.0f,
                  GLUT_BITMAP_HELVETICA_12, 0.7f, 0.7f, 0.7f);
    } else {
        float y = float(window_h) - 60.0f;
        for (int i = 0; i < (int)map_files.size(); ++i) {
            std::string line = (i == selected_map_index) ? "> " : "  ";
            line += map_files[i];
            float r = (i == selected_map_index) ? 1.0f : 0.7f;
            float g = (i == selected_map_index) ? 1.0f : 0.7f;
            draw_text(line.c_str(), 20.0f, y, GLUT_BITMAP_HELVETICA_12, r, g, 0.7f);
            y -= 20.0f;
        }
    }
    end_2d();
}

void intro(const char* text){
    begin_2d(window_w, window_h);
    char buf[100];
    if (stage == 0) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) { absolute_tick = 1; stage = 1; }
    }
    if (stage == 1) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) { absolute_tick = 1; stage = 2; }
    }
    if (stage == 2) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        delay_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){ stage = 3; absolute_tick = 1; }
    }
    if (stage == 3) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        draw_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        delay_text(text, 10.0f, float(window_h) - 74.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){ stage = 4; absolute_tick = 1; }
    }
    if (stage == 4) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        disappearing_text(text, 10.0f, float(window_h) - 74.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        if (absolute_tick >= 64){ stage = 5; absolute_tick = 1; }
    }
    end_2d();
}

int choise = 1;
int sound_choise = choise;

void main_menu(){
    begin_2d(window_w, window_h);
    if(!settings_mode) draw_text(">", 9.0f, 18*(4-choise), GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Play", 18.0f, 18*3, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Settings", 18.0f, 18*2, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit", 18.0f, 18*1, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    end_2d();
}

void settings(){
    begin_2d(window_w,window_h);
    char buf[100];
    draw_text(">", 9.0f, 18*(4-choise), GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Fov", 18.0f, 18*3, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", camera.fov);
    draw_text(buf, 144.0f, 18*3, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Sensentivity", 18.0f, 18*2, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", turn_speed);
    draw_text(buf, 144.0f, 18*2, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit settings", 18.0f, 18*1, GLUT_BITMAP_HELVETICA_12, 1.0f, 1.0f, 1.0f, 1.0f);
    end_2d();
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,yaw);
}

float panorama_move = 0;

void main_panorama(){
    end_2d();
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    if(absolute_tick%1==0){panorama_move+=turn_speed;}
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,panorama_move);
    camera.yaw=panorama_move;
}

void demo_scene(){
    bool plita=false;

    projector_1.setPosition(-edge, height, edge);
    projector_1.setDirectionFromPitchYaw(-35, 135);
    projector_2.setPosition(edge, height, edge);
    projector_2.setDirectionFromPitchYaw(-35, -135);
    projector_3.setPosition(edge, height, -edge);
    projector_3.setDirectionFromPitchYaw(-35, -45);
    projector_4.setPosition(-edge, height, -edge);
    projector_4.setDirectionFromPitchYaw(-35, 45);

    flashlight.setPosition(camera.eye_x, camera.eye_y, camera.eye_z);
    flashlight.setDirectionFromPitchYaw(camera.pitch, camera.yaw);
    useShader(defaultLightingShader);
    applyAllLights();
    applyAllShadows();

    for(float i=-10;i<=10;i+=2){
        for(float j=-10;j<=10;j+=2){
            if(plita){
                plane(i,-1,j,1,1,1,"src/wall_y.jpeg",{1,0,1, 1,0,-1, -1,0,-1, -1,0,1});
                plita=false;
            }else{
                plane(i,-1,j,0.5,0.5,0.5,nullptr,{1,0,1, 1,0,-1, -1,0,-1, -1,0,1});
                plita=true;
            }
        }
    }
    play_white_noise_3d(5,-1,5,1);
    radio->draw(camera.eye_x, camera.eye_y, camera.eye_z);
}

void demo(){
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    portals->checkTeleport();
    if(camera.pitch!=pitch){pitch=camera.pitch;}
    if(camera.yaw!=yaw){yaw=camera.yaw;}
    move_camera(camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);
    demo_scene();
    stopShader();

    portals->draw(2);

    draw_performance_hud(window_w,window_h);
    begin_2d(window_w,window_h);
    float size = 10.0f;
    float centerX = window_w / 2.0f;
    float centerY = window_h / 2.0f;
    square(size, centerX, centerY, 1,1,1, 0, verts_square, "src/penza_low.png");
    end_2d();
}

void display(){
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    if(stage<5)intro("Welcome to avoengine_game_example");
    if(stage==5){main_panorama();}
    if(stage==6){
        demo();
        if (map_menu_active) draw_map_menu();
    }
    if(settings_mode){
        settings();
    }else{
        if(stage==5){main_menu();}
    }
}

int delay = 6;
int last_footstep = 0;
int delay_map = 10;

void update() {
    update_mouse();

    if (stage == 6 && keys[GLFW_KEY_M] && absolute_tick % delay_map == 0) {
        map_menu_active = !map_menu_active;
        if (map_menu_active) {
            refresh_map_list();
            selected_map_index = 0;
        }
    }

    if (map_menu_active) {
        if (skeys[GLFW_KEY_UP] && absolute_tick % delay == 0) {
            if (!map_files.empty())
                selected_map_index = (selected_map_index - 1 + map_files.size()) % map_files.size();
        }
        if (skeys[GLFW_KEY_DOWN] && absolute_tick % delay == 0) {
            if (!map_files.empty())
                selected_map_index = (selected_map_index + 1) % map_files.size();
        }
        if (skeys[GLFW_KEY_ENTER] && absolute_tick % delay == 0) {
            if (!map_files.empty()) {
                std::string path = "maps/" + map_files[selected_map_index];
                MapData map;
                if (load_map(path.c_str(), map)) {
                    apply_loaded_map(map);
                    map_menu_active = false;
                } else {
                    std::cerr << "Failed to load map: " << path << std::endl;
                }
            }
        }
        if (keys[GLFW_KEY_S] && absolute_tick % delay == 0) {
            quick_save();
        }
        if (skeys[GLFW_KEY_ESCAPE] && absolute_tick % delay == 0) {
            map_menu_active = false;
        }
        return;
    }

    if (sound_choise != choise) {
        play_sound("src/switch-button.mp3");
        sound_choise = choise;
    }

    if (stage >= 0 && stage < 5 && skeys[GLFW_KEY_ESCAPE])
        stage = 5;

    if (settings_mode) {
        if (skeys[GLFW_KEY_UP] && absolute_tick % delay == 0) choise--;
        if (skeys[GLFW_KEY_DOWN] && absolute_tick % delay == 0) choise++;
        if (choise > 3) choise = 1;
        if (choise < 1) choise = 3;

        if ((skeys[GLFW_KEY_ENTER] && choise == 3) || (skeys[GLFW_KEY_ESCAPE]) && absolute_tick % delay == 0) {
            settings_mode = 0;
            choise = 1;
        }

        if (skeys[GLFW_KEY_RIGHT] && choise == 1) camera.fov += 0.05f;
        if (skeys[GLFW_KEY_LEFT] && choise == 1) camera.fov -= 0.05f;
        if (skeys[GLFW_KEY_RIGHT] && choise == 2) turn_speed += 0.01f;
        if (skeys[GLFW_KEY_LEFT] && choise == 2) turn_speed -= 0.01f;
    } else {
        if (stage == 5) {
            if (skeys[GLFW_KEY_UP] && absolute_tick % delay == 0) choise--;
            if (skeys[GLFW_KEY_DOWN] && absolute_tick % delay == 0) choise++;
            if (choise > 3) choise = 1;
            if (choise < 1) choise = 3;

            if (skeys[GLFW_KEY_ENTER] && choise == 3 && absolute_tick % delay == 0)
                exit(0);
            if (skeys[GLFW_KEY_ENTER] && choise == 2 && absolute_tick % delay == 0)
                settings_mode = 1;
            if (skeys[GLFW_KEY_ENTER] && choise == 1 && absolute_tick % delay == 0) {
                camera.yaw = 0;
                stage = 6;
            }
        }

        if (stage == 6) {
            if (skeys[GLFW_KEY_ESCAPE] && absolute_tick % delay == 0)
                settings_mode = 1;
            if (keys[GLFW_KEY_Q] && absolute_tick % delay == 0)
                exit(0);

            if (skeys[GLFW_KEY_RIGHT]) camera.yaw -= turn_speed;
            if (skeys[GLFW_KEY_LEFT])  camera.yaw += turn_speed;
            if (skeys[GLFW_KEY_UP])    camera.pitch += turn_speed;
            if (skeys[GLFW_KEY_DOWN])  camera.pitch -= turn_speed;

            float yr = camera.yaw * float(M_PI) / 180.0f;
            float mv = 0.1f;

            if (keys[GLFW_KEY_W]) {
                camera.eye_x += sinf(yr) * mv;
                camera.eye_z += cosf(yr) * mv;
                if (absolute_tick % 5 == 0 && last_footstep != absolute_tick) {
                    play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                    last_footstep = absolute_tick;
                }
            }
            if (keys[GLFW_KEY_S]) {
                camera.eye_x -= sinf(yr) * mv;
                camera.eye_z -= cosf(yr) * mv;
                if (absolute_tick % 5 == 0 && last_footstep != absolute_tick) {
                    play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                    last_footstep = absolute_tick;
                }
            }
            if (keys[GLFW_KEY_A]) {
                camera.eye_x += cosf(yr) * mv;
                camera.eye_z -= sinf(yr) * mv;
                if (absolute_tick % 5 == 0 && last_footstep != absolute_tick) {
                    play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                    last_footstep = absolute_tick;
                }
            }
            if (keys[GLFW_KEY_D]) {
                camera.eye_x -= cosf(yr) * mv;
                camera.eye_z += sinf(yr) * mv;
                if (absolute_tick % 5 == 0 && last_footstep != absolute_tick) {
                    play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                    last_footstep = absolute_tick;
                }
            }
        }
    }
}

int main(int argc, char** argv){
    glutInit(&argc, argv);
    setup_display(&argc, argv, 0.0f, 0.0f, 0.0f, 1.0f, "avoengine_example_game", 1280, 720);
    window = glfwGetCurrentContext();

    glEnable(GL_NORMALIZE);
    set_icon("avoengine_opengl/logo.png");
    useShader(defaultLightingShader);
    enable_light();
    set_ambient_light(0.05f, 0.05f, 0.05f);
    flashlight.setRadius(15.0f);
    flashlight.setColor(1.0f, 0.95f, 0.8f);
    flashlight.setIntensity(1.2f);
    flashlight.setAttenuation(1.0f, 0.1f, 0.01f);
    flashlight.enable();

    projector_1.setPosition(-edge, height, edge);
    projector_1.setDirectionFromPitchYaw(-45,-135);
    projector_1.setColor(1.0f, 0.0f, 0.0f);
    projector_1.enable();

    projector_2.setPosition(edge, height, edge);
    projector_2.setDirectionFromPitchYaw(-45,135);
    projector_2.setColor(0.0f, 1.0f, 0.0f);
    projector_2.enable();

    projector_3.setPosition(edge, height, -edge);
    projector_3.setDirectionFromPitchYaw(-45,45);
    projector_3.setColor(0.0f, 0.0f, 1.0f);
    projector_3.enable();

    projector_4.setPosition(-edge, height, -edge);
    projector_4.setDirectionFromPitchYaw(-45,-45);
    projector_4.setColor(1, 1, 1);
    projector_4.enable();

    Light* projs[] = { &projector_1, &projector_2, &projector_3, &projector_4 };
    for(int i = 0; i < 4; i++) {
        projs[i]->setRadius(15.0f);
        projs[i]->setIntensity(1.5f);
    }

    std::vector<float> portalVerts = { -1,-1,0, -1,1,0, 1,1,0, 1,-1,0 };
    portals = new Portal(5.0f, 0.0f, 5.0f,   -5.0f, 0.0f, -5.0f,
                     portalVerts,
                     45.0f, 0.0f, 0.0f,
                     0.0f, 0.0f, 0.0f);

    portals->setSceneDrawCallback([&]() {demo_scene();});

    setup_camera(camera.fov, camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);
    set_panorama("src/stargazer.png");
    enable_fog(0.05, 0.1, 0.1, 0.7, 5, 15);
    radio->setCastShadow(true);
    init_tick_system();
    init_keyboard(window);
    init_mouse(window);
    stopShader();
    while (!glfwWindowShouldClose(window)){
        update_ticks();
        update();
        display();
        glfwSwapBuffers(window);
        glfwPollEvents();
    }
    delete portals;
    portals = nullptr;
    stop_all_looping_sounds();
    glfwTerminate();
    return 0;
}