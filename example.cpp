#include "avoengine_opengl/avoengine.h"
#include "avoengine_opengl/avoextension.h"
#include <GLFW/glfw3.h>
#include <iostream>
#include <vector>
#include <filesystem>
#include <algorithm>
#include "avoengine_opengl/src/miniaudio.h"
#include "avoengine_opengl/portals_rc.h"
#include "avoengine_opengl/pseudo3dentity.h"
#include "avoengine_opengl/light.h"
#include "avoengine_opengl/ambient.h"
#include "avoengine_opengl/audio_not_mini.h"
#include "avoengine_opengl/2d_primitives.h"
#include "avoengine_opengl/3d_primitives.h"
#include "avoengine_opengl/shaders.h"
#include "avoengine_opengl/warp.h"
#include "avoengine_opengl/baking_scene.h"
#include "avoengine_opengl/ray_casting.h"

float pitch,yaw,roll;

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
pseudo_3d_entity* radio = nullptr;

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
Portal* portals_2 = nullptr;

std::vector<Light*> projectors = { &projector_1, &projector_2, &projector_3, &projector_4 };
std::vector<pseudo_3d_entity*> entitys = {radio};

void demo_scene();

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
    // stop_all_looping_sounds();

    // for (auto* e : allEntities) {
    //     delete e;
    // }
    // allEntities.clear();

    // projector_1.disable();
    // projector_2.disable();
    // projector_3.disable();
    // projector_4.disable();
    // flashlight.disable();

    // for (Light* l : dynamicLights) {
    //     l->disable();
    //     delete l;
    // }
    // dynamicLights.clear();

    // if (portals) {
    //     delete portals;
    //     portals = nullptr;
    // }
    // auto portalsCopy = allPortals;
    // for (auto* p : portalsCopy) {
    //     delete p; 
    // }
    // allPortals.clear();

    // for (const auto& ent : map.entities) {
    //     pseudo_3d_entity* e = mapDataToEntity(ent);
    //     registerEntity(e);
    // }

    // for (const auto& ldata : map.lights) {
    //     Light* newLight = new Light();
    //     mapDataToLight(ldata, *newLight);
    //     newLight->enable();
    //     dynamicLights.push_back(newLight);
    // }

    // for (const auto& portalData : map.portals) {
    //     Portal* newPortal = mapDataToPortal(portalData);
    //     if (!portals) {
    //         portals = newPortal;
    //     }
    // }

    // if (map.fog_enabled) {
    //     enable_fog(map.fog_density, map.fog_color[0], map.fog_color[1], map.fog_color[2],
    //                map.fog_start, map.fog_end);
    // } else {
    //     disable_fog();
    // }

    // camera.eye_x = map.camera_eye[0];
    // camera.eye_y = map.camera_eye[1];
    // camera.eye_z = map.camera_eye[2];
    // pitch = map.camera_pitch;
    // yaw = map.camera_yaw;
    // setup_camera(camera.fov, camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);

    // if (!map.panorama_path.empty()) {
    //     set_panorama(map.panorama_path.c_str());
    // } else {
    //     remove_panorama();
    // }

    // set_ambient_light(map.ambient[0], map.ambient[1], map.ambient[2]);
}

void quick_save() {
    // if (!std::filesystem::exists("maps"))
    //     std::filesystem::create_directory("maps");
    // save_current_scene("maps/quicksave.avomap");
    // refresh_map_list();
}

void draw_map_menu(){
    draw_text("~ Map Menu ~", 20.0f, float(window_h) - 30.0f,
              "avoengine_opengl/src/unifont.ttf", 18, 1.0f, 1.0f, 0.0f);
    draw_text("S: Save (quicksave.avomap)   Enter: Load   Esc: Close",
              20.0f, 20.0f, "avoengine_opengl/src/unifont.ttf", 18, 0.8f, 0.8f, 0.8f);

    if (map_files.empty()) {
        draw_text("(no maps in maps/ folder)", 20.0f, float(window_h) - 60.0f,
                  "avoengine_opengl/src/unifont.ttf", 18, 0.7f, 0.7f, 0.7f);
    } else {
        float y = float(window_h) - 60.0f;
        for (int i = 0; i < (int)map_files.size(); ++i) {
            std::string line = (i == selected_map_index) ? "> " : "  ";
            line += map_files[i];
            float r = (i == selected_map_index) ? 1.0f : 0.7f;
            float g = (i == selected_map_index) ? 1.0f : 0.7f;
            draw_text(line.c_str(), 20.0f, y,
                      "avoengine_opengl/src/unifont.ttf", 18, r, g, 0.7f);
            y -= 20.0f;
        }
    }
}

void intro(const char* text){
    char buf[100];
    const char* font = "avoengine_opengl/src/unifont.ttf";
    int fontSize = 18;
    if (stage == 0) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 20.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) { absolute_tick = 1; stage = 1; }
    }
    if (stage == 1) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 38.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) { absolute_tick = 1; stage = 2; }
    }
    if (stage == 2) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        delay_text(buf, 10.0f, float(window_h) - 56.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){ stage = 3; absolute_tick = 1; }
    }
    if (stage == 3) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        draw_text(buf, 10.0f, float(window_h) - 56.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
        delay_text(text, 10.0f, float(window_h) - 74.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){ stage = 4; absolute_tick = 1; }
    }
    if (stage == 4) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 20.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 38.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 56.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        disappearing_text(text, 10.0f, float(window_h) - 74.0f, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        if (absolute_tick >= 64){ stage = 5; absolute_tick = 1; }
    }
}

int choise = 1;
int sound_choise = choise;

void main_menu(){
    const char* font = "avoengine_opengl/src/unifont.ttf";
    int fontSize = 18;
    if(!settings_mode) draw_text(">", 9.0f, 18*(4-choise), font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Play", 18.0f, 18*3, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Settings", 18.0f, 18*2, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit", 18.0f, 18*1, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
}

void settings(){
    const char* font = "avoengine_opengl/src/unifont.ttf";
    int fontSize = 18;
    char buf[100];
    draw_text(">", 9.0f, 18*(4-choise), font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Fov", 18.0f, 18*3, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", camera.fov);
    draw_text(buf, 144.0f, 18*3, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Sensentivity", 18.0f, 18*2, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", turn_speed);
    draw_text(buf, 144.0f, 18*2, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit settings", 18.0f, 18*1, font, fontSize, 1.0f, 1.0f, 1.0f, 1.0f);
    
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,yaw);
}

float panorama_move = 0;

void main_panorama(){
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    if(absolute_tick%1==0){panorama_move+=turn_speed;}
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,panorama_move);
    camera.yaw=panorama_move;
}

WarpPlane warpRing;   

void warp_ring() {
    warpRing.originX = 0.0f;
    warpRing.originY = -0.99f;  
    warpRing.originZ = 0.0f;

    warpRing.yaw   = 0.0f;
    warpRing.pitch = -90.0f;    
    warpRing.roll  = 0.0f;

    warpRing.sizeU = 10.0f;      
    warpRing.sizeV = 10.0f;

    const int texW = 256;
    const int texH = 256;
    std::vector<float> dispData(texW * texH * 3, 0.0f);

    const float radius = 4.0f;
    const float thickness = 3.0f;
    const float innerR = radius - thickness * 0.5f;  
    const float outerR = radius + thickness * 0.5f;   
    const float depth = 1.0;  

    for (int y = 0; y < texH; ++y) {
        for (int x = 0; x < texW; ++x) {
            float u = (x + 0.5f) / texW;
            float v = (y + 0.5f) / texH;

            float lx = (u - 0.5f) * warpRing.sizeU;
            float lz = (v - 0.5f) * warpRing.sizeV;

            float dist = std::sqrt(lx * lx + lz * lz);

            float t = glm::smoothstep(innerR, radius, dist) * (1.0f - glm::smoothstep(radius, outerR, dist));
            float displacement = depth * t;

            int idx = (y * texW + x) * 3;
            dispData[idx + 0] = 0.0f;
            dispData[idx + 1] = -displacement;
            dispData[idx + 2] = 0.0f;
        }
    }

    warpRing.setDisplacementFromData(texW, texH, dispData.data());
    warpRing.enable();  
    set_active_warp_plane(&warpRing); 
}

WarpPlane warpCylinder;

void warp_cylinder() {
    warpCylinder.originX = 0.0f;
    warpCylinder.originY = -1.0f;
    warpCylinder.originZ = 0.0f;

    warpCylinder.yaw   = 0.0f;
    warpCylinder.pitch = -90.0f;
    warpCylinder.roll  = 0.0f;

    warpCylinder.sizeU = 10.0f;
    warpCylinder.sizeV = 10.0f;

    const int texW = 256;
    const int texH = 256;
    std::vector<float> dispData(texW * texH * 3, 0.0f);

    const float radius = 3.0f;
    const float depth  = 2.0f;

    for (int y = 0; y < texH; ++y) {
        float v = (y + 0.5f) / texH;
        float lz = (v - 0.5f) * warpCylinder.sizeV;

        for (int x = 0; x < texW; ++x) {
            float u = (x + 0.5f) / texW;
            float lx = (u - 0.5f) * warpCylinder.sizeU;

            float dist = std::sqrt(lx * lx + lz * lz);
            float displacement = (dist <= radius) ? depth : 0.0f;

            int idx = (y * texW + x) * 3;
            dispData[idx + 0] = 0.0f;
            dispData[idx + 1] = -displacement;
            dispData[idx + 2] = 0.0f;
        }
    }

    warpCylinder.setDisplacementFromData(texW, texH, dispData.data());
    warpCylinder.enable();
    set_active_warp_plane(&warpCylinder); 
}

static glm::vec3 getWarpDisplacement(float u, float v) {
    if (!activeWarpPlane || !activeWarpPlane->enabled || !activeWarpPlane->displacementTex)
        return glm::vec3(0.0f);

    static std::vector<float> dispData;
    static GLuint lastTex = 0;
    static int lastW = 0, lastH = 0;

    if (lastTex != activeWarpPlane->displacementTex) {
        glBindTexture(GL_TEXTURE_2D, activeWarpPlane->displacementTex);
        GLint w, h;
        glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &w);
        glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &h);
        dispData.resize(w * h * 3);
        glGetTexImage(GL_TEXTURE_2D, 0, GL_RGB, GL_FLOAT, dispData.data());
        glBindTexture(GL_TEXTURE_2D, 0);
        lastTex = activeWarpPlane->displacementTex;
        lastW = w; lastH = h;
    }

    if (lastW == 0 || lastH == 0) return glm::vec3(0.0f);

    float x = u * (lastW - 1);
    float y = v * (lastH - 1);
    int x0 = std::max(0, std::min(lastW - 1, (int)floor(x)));
    int y0 = std::max(0, std::min(lastH - 1, (int)floor(y)));
    int x1 = std::max(0, std::min(lastW - 1, x0 + 1));
    int y1 = std::max(0, std::min(lastH - 1, y0 + 1));
    float fx = x - x0, fy = y - y0;

    auto sample = [&](int xi, int yi) -> glm::vec3 {
        int idx = (yi * lastW + xi) * 3;
        return glm::vec3(dispData[idx], dispData[idx+1], dispData[idx+2]);
    };

    glm::vec3 v00 = sample(x0, y0);
    glm::vec3 v10 = sample(x1, y0);
    glm::vec3 v01 = sample(x0, y1);
    glm::vec3 v11 = sample(x1, y1);

    glm::vec3 v0 = glm::mix(v00, v10, fx);
    glm::vec3 v1 = glm::mix(v01, v11, fx);
    return glm::mix(v0, v1, fy);
}

void draw_warp_minimap() {
    if (!activeWarpPlane || !activeWarpPlane->enabled) return;

    glm::mat4 rot = glm::mat4(1.0f);
    rot = glm::rotate(rot, glm::radians(activeWarpPlane->yaw), glm::vec3(0,1,0));
    rot = glm::rotate(rot, glm::radians(activeWarpPlane->pitch), glm::vec3(1,0,0));
    rot = glm::rotate(rot, glm::radians(activeWarpPlane->roll), glm::vec3(0,0,1));
    glm::vec3 uAxis = glm::vec3(rot * glm::vec4(activeWarpPlane->sizeU, 0, 0, 0));
    glm::vec3 vAxis = glm::vec3(rot * glm::vec4(0, activeWarpPlane->sizeV, 0, 0));
    glm::vec3 origin(activeWarpPlane->originX, activeWarpPlane->originY, activeWarpPlane->originZ);

    glm::vec3 corners[4] = {
        origin + uAxis * 0.5f + vAxis * 0.5f,
        origin - uAxis * 0.5f + vAxis * 0.5f,
        origin - uAxis * 0.5f - vAxis * 0.5f,
        origin + uAxis * 0.5f - vAxis * 0.5f
    };

    const float mapSize = 120.0f;
    const float margin = 10.0f;
    const float gap = 5.0f;
    const float startX = window_w - (3 * mapSize + 2 * gap) - margin;
    const float startY = margin;
    const float liftAmount = 50.0f;

    struct Proj { int axis1, axis2; const char* label; };
    Proj projs[3] = { {0,2,"Top"}, {0,1,"Front"}, {2,1,"Side"} };

    auto projectPoint = [&](const glm::vec3& p, int ax1, int ax2) -> glm::vec2 {
        return glm::vec2(p[ax1], p[ax2]);
    };

    for (int p = 0; p < 3; ++p) {
        float offX = startX + p * (mapSize + gap);

        float gridOffY = startY;
        if (p == 1 || p == 2) gridOffY += liftAmount;

        float indicatorOffY = startY; 
        float labelY = indicatorOffY + mapSize + 4.0f;

        int ax1 = projs[p].axis1;
        int ax2 = projs[p].axis2;

        float minX = 1e9f, maxX = -1e9f, minY = 1e9f, maxY = -1e9f;
        const int gridRes = 20;

        if (p == 0) {
            for (int i = 0; i < 4; ++i) {
                glm::vec2 pt = projectPoint(corners[i], ax1, ax2);
                minX = fmin(minX, pt.x); maxX = fmax(maxX, pt.x);
                minY = fmin(minY, pt.y); maxY = fmax(maxY, pt.y);
            }
        } else {
            for (int iu = 0; iu <= gridRes; ++iu) {
                float u = iu / (float)gridRes;
                for (int iv = 0; iv <= gridRes; ++iv) {
                    float v = iv / (float)gridRes;
                    glm::vec3 pos = origin + uAxis * (u - 0.5f) + vAxis * (v - 0.5f) + getWarpDisplacement(u, v);
                    glm::vec2 pt = projectPoint(pos, ax1, ax2);
                    minX = fmin(minX, pt.x); maxX = fmax(maxX, pt.x);
                    minY = fmin(minY, pt.y); maxY = fmax(maxY, pt.y);
                }
            }
        }

        glm::vec3 camPos(camera.eye_x, camera.eye_y, camera.eye_z);
        glm::vec2 camProj = projectPoint(camPos, ax1, ax2);
        minX = fmin(minX, camProj.x); maxX = fmax(maxX, camProj.x);
        minY = fmin(minY, camProj.y); maxY = fmax(maxY, camProj.y);

        float rangeX = maxX - minX;
        float rangeY = maxY - minY;
        if (rangeX < 0.01f) rangeX = 1.0f;
        if (rangeY < 0.01f) rangeY = 1.0f;
        minX -= rangeX * 0.1f;
        maxX += rangeX * 0.1f;
        minY -= rangeY * 0.1f;
        maxY += rangeY * 0.1f;

        auto toScreenGrid = [&](float wx, float wy) -> std::pair<float,float> {
            return { offX + (wx - minX) / (maxX - minX) * mapSize,
                     gridOffY + (wy - minY) / (maxY - minY) * mapSize };
        };

        if (p == 0) {
            const int gridTop = 10;
            for (int iv = 0; iv <= gridTop; ++iv) {
                float v = iv / (float)gridTop;
                glm::vec3 p0 = origin + uAxis * (-0.5f) + vAxis * (v - 0.5f);
                glm::vec3 p1 = origin + uAxis * (0.5f) + vAxis * (v - 0.5f);
                glm::vec2 s0 = projectPoint(p0, ax1, ax2);
                glm::vec2 s1 = projectPoint(p1, ax1, ax2);
                auto scr0 = toScreenGrid(s0.x, s0.y);
                auto scr1 = toScreenGrid(s1.x, s1.y);
                draw_line_2d(0,0, scr0.first,scr0.second, scr1.first,scr1.second, 1,1,1,1, 0.5f);
            }
            for (int iu = 0; iu <= gridTop; ++iu) {
                float u = iu / (float)gridTop;
                glm::vec3 p0 = origin + uAxis * (u - 0.5f) + vAxis * (-0.5f);
                glm::vec3 p1 = origin + uAxis * (u - 0.5f) + vAxis * (0.5f);
                glm::vec2 s0 = projectPoint(p0, ax1, ax2);
                glm::vec2 s1 = projectPoint(p1, ax1, ax2);
                auto scr0 = toScreenGrid(s0.x, s0.y);
                auto scr1 = toScreenGrid(s1.x, s1.y);
                draw_line_2d(0,0, scr0.first,scr0.second, scr1.first,scr1.second, 1,1,1,1, 0.5f);
            }
        } else {
            for (int iv = 0; iv <= gridRes; ++iv) {
                float v = iv / (float)gridRes;
                for (int iu = 0; iu < gridRes; ++iu) {
                    float u = iu / (float)gridRes;
                    glm::vec3 pos = origin + uAxis * (u - 0.5f) + vAxis * (v - 0.5f) + getWarpDisplacement(u, v);
                    float uNext = (iu + 1) / (float)gridRes;
                    glm::vec3 posNext = origin + uAxis * (uNext - 0.5f) + vAxis * (v - 0.5f) + getWarpDisplacement(uNext, v);
                    glm::vec2 pt = projectPoint(pos, ax1, ax2);
                    glm::vec2 ptNext = projectPoint(posNext, ax1, ax2);
                    auto scr = toScreenGrid(pt.x, pt.y);
                    auto scrN = toScreenGrid(ptNext.x, ptNext.y);
                    draw_line_2d(0,0, scr.first,scr.second, scrN.first,scrN.second, 1,1,1,1, 0.5f);
                }
            }
            for (int iu = 0; iu <= gridRes; ++iu) {
                float u = iu / (float)gridRes;
                for (int iv = 0; iv < gridRes; ++iv) {
                    float v = iv / (float)gridRes;
                    glm::vec3 pos = origin + uAxis * (u - 0.5f) + vAxis * (v - 0.5f) + getWarpDisplacement(u, v);
                    float vNext = (iv + 1) / (float)gridRes;
                    glm::vec3 posNext = origin + uAxis * (u - 0.5f) + vAxis * (vNext - 0.5f) + getWarpDisplacement(u, vNext);
                    glm::vec2 pt = projectPoint(pos, ax1, ax2);
                    glm::vec2 ptNext = projectPoint(posNext, ax1, ax2);
                    auto scr = toScreenGrid(pt.x, pt.y);
                    auto scrN = toScreenGrid(ptNext.x, ptNext.y);
                    draw_line_2d(0,0, scr.first,scr.second, scrN.first,scrN.second, 1,1,1,1, 0.5f);
                }
            }
        }

        glm::vec3 rel = camPos - origin;
        glm::mat3 invBasis = glm::inverse(glm::mat3(uAxis, vAxis, glm::cross(uAxis, vAxis)));
        glm::vec3 local = invBasis * rel;
        float u_cam = local.x + 0.5f;
        float v_cam = local.y + 0.5f;
        bool inside = (u_cam >= 0.0f && u_cam <= 1.0f && v_cam >= 0.0f && v_cam <= 1.0f);
        glm::vec3 camPosCorrected = camPos;
        if (inside) {
            glm::vec3 disp = getWarpDisplacement(u_cam, v_cam);
            camPosCorrected += disp;
        }
        glm::vec2 camProjCorrected = projectPoint(camPosCorrected, ax1, ax2);

        auto camScreenGrid = toScreenGrid(camProjCorrected.x, camProjCorrected.y);

        float camDrawY = camScreenGrid.second - (gridOffY - indicatorOffY);

        float ptSize = 4.0f;
        float verts[8] = {
            camScreenGrid.first - ptSize, camDrawY - ptSize,
            camScreenGrid.first + ptSize, camDrawY - ptSize,
            camScreenGrid.first + ptSize, camDrawY + ptSize,
            camScreenGrid.first - ptSize, camDrawY + ptSize
        };
        square(1.0f, 0, 0, 1.0, 0.0, 0.0, 0, verts, nullptr, 1.0f);

        draw_text(projs[p].label, offX, labelY, "avoengine_opengl/src/unifont.ttf",12, 1,1,1, 1);
    }
}

void fixed_demo_scene(){
    bool plita=false;

    projector_1.setPosition(-edge, height, edge);
    projector_1.setDirectionFromPitchYaw(-35, 135);
    projector_2.setPosition(edge, height, edge);
    projector_2.setDirectionFromPitchYaw(-35, -135);
    projector_3.setPosition(edge, height, -edge);
    projector_3.setDirectionFromPitchYaw(-35, -45);
    projector_4.setPosition(-edge, height, -edge);
    projector_4.setDirectionFromPitchYaw(-35, 45);

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
}
int last_tick= absolute_tick;
void demo_scene(){
    if(last_tick!=absolute_tick){warpRing.pitch += 1;last_tick= absolute_tick;}
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    flashlight.setPosition(camera.eye_x,camera.eye_y, camera.eye_z);
    flashlight.dir[0] = camera.dir_x;
    flashlight.dir[1] = camera.dir_y;
    flashlight.dir[2] = camera.dir_z;

    radio->draw(camera.eye_x, camera.eye_y, camera.eye_z);

    portals->draw();
    portals_2->draw();

    draw_line_3d(0, -0.5,0, -10,0,-10, 10,0,10, 1,1,1,0.05, 16, 0.1);

    plane(0,0,0,0.7,0,0,nullptr,{-11,-1,-7, -11,-1,7, -11,1,7, -11,1,-7});
    plane(0,0,0,0.7,0,0,nullptr,{-9,-1,-7, -9,-1,7, -9,1,7, -9,1,-7});

    plane(0,0,0,0,0.7,0,nullptr,{11,-1,-3, 11,-1,3, 11,1,3, 11,1,-3});
    plane(0,0,0,0,0.7,0,nullptr,{9,-1,-3, 9,-1,3, 9,1,3, 9,1,-3});

    plane(-10, 1, 0, 0.7, 0, 0,nullptr, {1,0,7, 1,0,-7, -1,0,-7, -1,0,7});
    plane(10, 1, 0, 0, 0.7, 0,nullptr, {1,0,3, 1,0,-3, -1,0,-3, -1,0,3});

    // fixed_demo_scene();
}
void demo(){
    if(camera.pitch!=pitch){pitch=camera.pitch;}
    if(camera.yaw!=yaw){yaw=camera.yaw;}
    move_camera(camera.eye_x, camera.eye_y, camera.eye_z, pitch, camera.yaw,roll);

    demo_scene();

    draw_performance_hud(window_w,window_h, "avoengine_opengl/src/unifont.ttf");
    float size = 10.0f;
    float centerX = window_w / 2.0f;
    float centerY = window_h / 2.0f;
    // square(size, centerX, centerY, 1,1,1, 0, verts_square, "src/penza_low.png");
    draw_line_2d(centerX, centerY,  20,  20, -20, -20,  1,1,1,1, 2);  
    draw_line_2d(centerX, centerY, -20,  20,  20, -20,  1,1,1,1, 2);

    draw_warp_minimap();
}

void display(){
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    if(stage<5)intro("Welcome to avoengine_game_example");
    if(stage==5){
        draw_performance_hud(window_w,window_h,"avoengine_opengl/src/unifont.ttf");
        main_panorama();
    }
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

int delay = 10;
int last_footstep = 0;
int delay_map = 10;
int menu_cooldown=0;
int prev_mouse_x = 0, prev_mouse_y = 0;
bool mouse_was_captured = false;
int toggle_cooldown = 0;
void update() {
    roll=0;
    camera.eye_y=0;
    if (menu_cooldown > 0) menu_cooldown--;
    if (toggle_cooldown > 0) toggle_cooldown--;

    update_mouse();

    if (stage == 6 && mouse_captured && !settings_mode && !map_menu_active) {
        int dx = mouse_x - prev_mouse_x;
        int dy = mouse_y - prev_mouse_y;
        prev_mouse_x = mouse_x;
        prev_mouse_y = mouse_y;

        const float sensitivity = 0.1f;
        camera.yaw   -= dx * sensitivity;
        camera.pitch -= dy * sensitivity;
    }

    if (stage == 6 && keys[GLFW_KEY_M] && toggle_cooldown <= 0) {
        map_menu_active = !map_menu_active;
        toggle_cooldown = 20;
        if (map_menu_active) {
            if (mouse_was_captured) {
                set_mouse_capture(window, false);
                mouse_was_captured = false;
            }
            refresh_map_list();
            selected_map_index = 0;
        } else {
            if (!settings_mode) {
                set_mouse_capture(window, true);
                prev_mouse_x = mouse_x;
                prev_mouse_y = mouse_y;
                mouse_was_captured = true;
            }
        }
    }

    if (map_menu_active) {
        if (skeys[GLFW_KEY_UP] && menu_cooldown <= 0) {
            if (!map_files.empty())
                selected_map_index = (selected_map_index - 1 + map_files.size()) % map_files.size();
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_DOWN] && menu_cooldown <= 0) {
            if (!map_files.empty())
                selected_map_index = (selected_map_index + 1) % map_files.size();
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_ENTER] && menu_cooldown <= 0) {
            // if (!map_files.empty()) {
            //     std::string path = "maps/" + map_files[selected_map_index];
            //     MapData map;
            //     if (load_map(path.c_str(), map)) {
            //         apply_loaded_map(map);
            //         map_menu_active = false;
            //         toggle_cooldown = 20;
            //         set_mouse_capture(window, true);
            //         prev_mouse_x = mouse_x;
            //         prev_mouse_y = mouse_y;
            //         mouse_was_captured = true;
            //     } else {
            //         std::cerr << "Failed to load map: " << path << std::endl;
            //     }
            // }
            menu_cooldown = 10;
        }
        if (keys[GLFW_KEY_S] && menu_cooldown <= 0) {
            quick_save();
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_ESCAPE] && menu_cooldown <= 0) {
            map_menu_active = false;
            menu_cooldown = 10;
            toggle_cooldown = 20;
            set_mouse_capture(window, true);
            prev_mouse_x = mouse_x;
            prev_mouse_y = mouse_y;
            mouse_was_captured = true;

        }
        return;
    }

    if (sound_choise != choise && last_footstep != absolute_tick) {
        play_sound("src/switch-button.mp3");
        sound_choise = choise;
        last_footstep = absolute_tick;
    }

    if (stage >= 0 && stage < 5 && skeys[GLFW_KEY_ESCAPE]) {
        stage = 5;
    }

    if (settings_mode) {
        if (skeys[GLFW_KEY_UP] && menu_cooldown <= 0) {
            choise--;
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_DOWN] && menu_cooldown <= 0) {
            choise++;
            menu_cooldown = 10;
        }
        if (choise > 3) choise = 1;
        if (choise < 1) choise = 3;

        if ((skeys[GLFW_KEY_ENTER] || skeys[GLFW_KEY_ESCAPE]) && menu_cooldown <= 0 && toggle_cooldown <= 0) {
            settings_mode = 0;
            choise = 1;
            menu_cooldown = 10;
            toggle_cooldown = 20;
            if (stage == 6) {
                set_mouse_capture(window, true);
                prev_mouse_x = mouse_x;
                prev_mouse_y = mouse_y;
                mouse_was_captured = true;
            }
        }

        if (skeys[GLFW_KEY_RIGHT] && choise == 1 && menu_cooldown <= 0) {
            camera.fov += 0.05f;
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_LEFT] && choise == 1 && menu_cooldown <= 0) {
            camera.fov -= 0.05f;
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_RIGHT] && choise == 2 && menu_cooldown <= 0) {
            turn_speed += 0.01f;
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_LEFT] && choise == 2 && menu_cooldown <= 0) {
            turn_speed -= 0.01f;
            menu_cooldown = 10;
        }
    }
    else if (stage == 5) {
        if (skeys[GLFW_KEY_UP] && menu_cooldown <= 0) {
            choise--;
            menu_cooldown = 10;
        }
        if (skeys[GLFW_KEY_DOWN] && menu_cooldown <= 0) {
            choise++;
            menu_cooldown = 10;
        }
        if (choise > 3) choise = 1;
        if (choise < 1) choise = 3;

        if (skeys[GLFW_KEY_ENTER] && menu_cooldown <= 0) {
            menu_cooldown = 10;
            if (choise == 3)
                exit(0);
            else if (choise == 2)
                settings_mode = 1;
            else if (choise == 1) {
                camera.yaw = 0;
                stage = 6;
                set_mouse_capture(window, true);
                prev_mouse_x = mouse_x;
                prev_mouse_y = mouse_y;
                mouse_was_captured = true;
                warp_ring();
                // warp_cylinder();
                clean_scene();
                fixed_scene(fixed_demo_scene);
            }
        }
    }

    if (stage == 6) {
        if (!settings_mode && skeys[GLFW_KEY_ESCAPE] && toggle_cooldown <= 0) {
            settings_mode = 1;
            toggle_cooldown = 20;
            if (mouse_was_captured) {
                set_mouse_capture(window, false);
                mouse_was_captured = false;
            }
        }

        if (keys[GLFW_KEY_Q] && absolute_tick % delay == 0)
            exit(0);

        if (skeys[GLFW_KEY_RIGHT]) camera.yaw -= turn_speed;
        if (skeys[GLFW_KEY_LEFT])  camera.yaw += turn_speed;
        if (skeys[GLFW_KEY_UP])    camera.pitch += turn_speed;
        if (skeys[GLFW_KEY_DOWN])  camera.pitch -= turn_speed;

        if (skeys[GLFW_KEY_LEFT_SHIFT])  camera.eye_y = -0.5;

        float yr = camera.yaw * float(M_PI) / 180.0f;
        float mv = 0.1f;

        if (keys[GLFW_KEY_W]) {
            camera.eye_x += sinf(yr) * mv;
            camera.eye_z += cosf(yr) * mv;
            if (absolute_tick % delay == 0 && last_footstep != absolute_tick) {
                play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                last_footstep = absolute_tick;
            }
        }
        if (keys[GLFW_KEY_S]) {
            camera.eye_x -= sinf(yr) * mv;
            camera.eye_z -= cosf(yr) * mv;
            if (absolute_tick % delay == 0 && last_footstep != absolute_tick) {
                play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                last_footstep = absolute_tick;
            }
        }
        if (keys[GLFW_KEY_A]) {
            camera.eye_x += cosf(yr) * mv;
            camera.eye_z -= sinf(yr) * mv;
            roll = -5;
            if (absolute_tick % delay == 0 && last_footstep != absolute_tick) {
                play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                last_footstep = absolute_tick;
            }
        }
        if (keys[GLFW_KEY_D]) {
            camera.eye_x -= cosf(yr) * mv;
            camera.eye_z += sinf(yr) * mv;
            roll = 5;
            if (absolute_tick % delay == 0 && last_footstep != absolute_tick) {
                play_sound_3d("src/footstep.wav", camera.eye_x, camera.ctr_y - 1, camera.eye_z);
                last_footstep = absolute_tick;
            }
        }
    }
}

int main(int argc, char** argv){
    // glutInit(&argc, argv);
    setup_display(&argc, argv, 0.0f, 0.0f, 0.0f, 1.0f, "avoengine_example_game", 1280, 720);
    window = glfwGetCurrentContext();

    radio = new pseudo_3d_entity(0, -0.5, 0, 0, 0, 0.0f, textures_str, 8, std::vector<float>(verts, verts + 8));

    glEnable(GL_NORMALIZE);
    set_icon("avoengine_opengl/src/logo.png");
    useShader(defaultLightingShader);
    set_ambient_light(0.05f, 0.05f, 0.05f);
    // set_ambient_light(0.7, 0.7, 0.7);
    flashlight.setRadius(20.0f);
    flashlight.setColor(1.0f, 0.95f, 0.8f);
    flashlight.setIntensity(3);
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
    portals = new Portal(10.0f, 0.0f, -3.0f, -10.0f, 0.0f, -7.0f,
                     portalVerts,
                     0.0f, 0.0f, 0.0f,
                     0.0f, 0.0f, 0.0f);

    portals_2 = new Portal(10.0f, 0.0f, 3.0f, -10.0f, 0.0f, 7.0f,
                     portalVerts,
                     0.0f, 0.0f, 0.0f,
                     0.0f, 0.0f, 0.0f);

    setup_camera(camera.fov, camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);
    set_panorama("src/stargazer.png");
    enable_fog(0.05, 0.1, 0.1, 0.7, 5, 15);
    init_tick_system();
    init_keyboard(window);
    init_mouse(window);
    stopShader();
    while (!glfwWindowShouldClose(window)){
        useShader(defaultLightingShader);
        update_ticks();
        update();
        display();
        flushDrawQueue();
        glfwSwapBuffers(window);
        glfwPollEvents();
    }
    delete portals;
    portals = nullptr;
    stop_all_looping_sounds();
    glfwTerminate();
    return 0;
}