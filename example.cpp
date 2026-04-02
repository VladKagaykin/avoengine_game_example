#include"avoengine_opengl/avoengine.h"
#include"avoengine_opengl/avoextension.h"
#include <GL/glu.h>
#include <GL/glut.h>
#include <iostream>
#include <vector>
bool settings_mode=0;
int stage=0;
float turn_speed=0.58;
float pitch = 0;
float yaw =0;
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
pseudo_3d_entity* radio = new pseudo_3d_entity(5, -0.5, 5, 0, 0, textures, 8, verts);
glb_model* glb = new glb_model(-5, -0.5, 5);
Light flashlight(0);
float verts_square[] = {-1,-1, 1,-1, 1,1, -1,1};
void intro(const char* text){
    begin_2d(window_w, window_h);
    char buf[100]; 
    if (stage == 0) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) {
            absolute_tick = 1;
            stage = 1;        
        }
    }
    if (stage == 1) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        delay_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64) {
            absolute_tick = 1; 
            stage = 2;
        }
    }
    if (stage == 2) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        delay_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){
            stage = 3;
            absolute_tick = 1;
        } 
    }
    if (stage == 3) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        draw_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        draw_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
        delay_text(text, 10.0f, float(window_h) - 74.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 32, false);
        if (absolute_tick >= 64){
            stage = 4;
            absolute_tick = 1;
        } 
    }
    if (stage == 4) {
        snprintf(buf, sizeof(buf), "CPU: %s ", cpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 20.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "GPU: %s ", gpu_name.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 38.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        snprintf(buf, sizeof(buf), "RAM: %s ", ram_v.c_str());
        disappearing_text(buf, 10.0f, float(window_h) - 56.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        disappearing_text(text, 10.0f, float(window_h) - 74.0f, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f, 64, false);
        if (absolute_tick >= 64){
            stage = 5;
            absolute_tick = 1;
        } 
    }
    end_2d();
}
int choise=1;
int sound_choise=choise;
void main_menu(){
    begin_2d(window_w, window_h);
    if(!settings_mode) draw_text(">", 9.0f, 18*(4-choise), GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Play", 18.0f, 18*3, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Settings", 18.0f, 18*2, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit", 18.0f, 18*1, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    end_2d();
}
void settings(){
    begin_2d(window_w,window_h);
    char buf[100];
    draw_text(">", 9.0f, 18*(4-choise), GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Fov", 18.0f, 18*3, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", camera.fov);
    draw_text(buf, 144.0f, 18*3, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Sensentivity", 18.0f, 18*2, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    snprintf(buf, sizeof(buf), "%.2f", turn_speed);
    draw_text(buf, 144.0f, 18*2, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    draw_text("Quit settings", 18.0f, 18*1, GLUT_BITMAP_HELVETICA_18, 1.0f, 1.0f, 1.0f, 1.0f);
    end_2d();
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,yaw);
}
float panorama_move=0;
void main_panorama(){
    end_2d();
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    if(absolute_tick%1==0){panorama_move+=turn_speed;}
    setup_camera(camera.fov,camera.eye_x,camera.eye_y,camera.eye_z,pitch,panorama_move);
    yaw=panorama_move;
}
void demo(){
    bool plita=false;
    // glDisable(GL_LIGHT0);
    draw_panorama(camera.eye_x,camera.eye_y,camera.eye_z);
    flashlight.setPosition(camera.eye_x, camera.eye_y, camera.eye_z);
    flashlight.setDirectionFromPitchYaw(pitch, yaw);
    for(float i=-10;i<=10;i+=2){
        for(float j=-10;j<=10;j+=2){
            if(plita){
                plane(i,-1,j,1,1,1,"src/wall_y.jpeg",{1,0,1,
                                                    1,0,-1,
                                                    -1,0,-1,
                                                    -1,0,1});
                plita=false;
            }else{
                plane(i,-1,j,0.5,0.5,0.5,nullptr,{1,0,1,
                                            1,0,-1,
                                            -1,0,-1,
                                            -1,0,1});
                plita=true;
            }
        }
    }
    move_camera(camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);
    play_white_noise_3d(5,-1,5,1);
    radio->draw(yaw,camera.eye_x,camera.eye_y,camera.eye_z);
    draw_performance_hud(window_w,window_h);
    glb->updateAnimation(absolute_tick * 0.01f);
    glb->draw();
    begin_2d(window_w,window_h);
    float size = 10.0f;                      // размер в пикселях
    float centerX = window_w / 2.0f;
    float centerY = window_h / 2.0f;
    square(size, centerX, centerY, 1,1,1, 0, verts_square, "src/penza_low.png");
    end_2d();
}
void display(){
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    if(stage<5)intro("Welcome to avoengine_game_example");
    if(stage==5){main_panorama();}
    if(stage==6){demo();}
    if(settings_mode){
        settings();
    }else{
        if(stage==5){main_menu();}
    }
    glutSwapBuffers();
}
int delay=6;
void update(){
    if(sound_choise!=choise){
        play_sound("src/switch-button.mp3");
        sound_choise=choise;
    }
    if(stage>=0 and stage<5 and keys[27])stage=5;
    if(settings_mode){
        if(skeys[GLUT_KEY_UP] and absolute_tick%delay==0)choise--;
        if(skeys[GLUT_KEY_DOWN] and absolute_tick%delay==0)choise++;
        if(choise>3)choise=1;
        if(choise<1)choise=3;
        if(keys[13]and choise==3 and absolute_tick%delay==0 or keys[27] and absolute_tick%delay==0){settings_mode=0;choise=1;}
        if(skeys[GLUT_KEY_RIGHT]and choise==1)camera.fov+=0.05;
        if(skeys[GLUT_KEY_LEFT]and choise==1)camera.fov-=0.05;
        if(skeys[GLUT_KEY_RIGHT]and choise==2)turn_speed+=0.01;
        if(skeys[GLUT_KEY_LEFT]and choise==2)turn_speed-=0.01;
    }else{
        if(stage==5){
            if(skeys[GLUT_KEY_UP] and absolute_tick%delay==0)choise--;
            if(skeys[GLUT_KEY_DOWN] and absolute_tick%delay==0)choise++;
            if(choise>3)choise=1;
            if(choise<1)choise=3;
            if(keys[13]and choise==3 and absolute_tick%delay==0)exit(0);
            if(keys[13]and choise==2 and absolute_tick%delay==0)settings_mode=1;
            if(keys[13]and choise==1 and absolute_tick%delay==0){yaw=0;stage=6;}
        }
        if(stage==6){
            if(keys[27] and absolute_tick%delay==0)settings_mode=1;
            if(keys['q'] and absolute_tick%delay==0 or keys['Q'] and absolute_tick%delay==0)exit(0);
            if(skeys[GLUT_KEY_RIGHT])yaw-=turn_speed;
            if(skeys[GLUT_KEY_LEFT])yaw+=turn_speed;
            if(skeys[GLUT_KEY_UP])pitch+=turn_speed;
            if(skeys[GLUT_KEY_DOWN])pitch-=turn_speed;
            float yr = yaw * float(M_PI) / 180.0f;
            float mv = 0.1;
            if(keys['w']or keys['W']){camera.eye_x+=sinf(yr)*mv;camera.eye_z+=cosf(yr)*mv;if(absolute_tick%16==0)play_sound_3d("src/footstep.wav",camera.eye_x,camera.ctr_y-1,camera.eye_z);}
            if(keys['s']or keys['S']){camera.eye_x-=sinf(yr)*mv;camera.eye_z-=cosf(yr)*mv;if(absolute_tick%16==0)play_sound_3d("src/footstep.wav",camera.eye_x,camera.ctr_y-1,camera.eye_z);}
            if(keys['a']or keys['A']){camera.eye_x+=cosf(yr)*mv;camera.eye_z-=sinf(yr)*mv;if(absolute_tick%16==0)play_sound_3d("src/footstep.wav",camera.eye_x,camera.ctr_y-1,camera.eye_z);}
            if(keys['d']or keys['D']){camera.eye_x-=cosf(yr)*mv;camera.eye_z+=sinf(yr)*mv;if(absolute_tick%16==0)play_sound_3d("src/footstep.wav",camera.eye_x,camera.ctr_y-1,camera.eye_z);}
        }
    }
    glutPostRedisplay();
}
int main(int argc, char** argv) {
    setup_display(&argc, argv, 0.0f, 0.0f, 0.0f, 1.0f, "avoengine_example_game", 1280, 720);
    glEnable(GL_NORMALIZE);
    set_icon("avoengine_opengl/logo.png");
    
    enable_light();
    set_ambient_light(0.05f, 0.05f, 0.05f); // тёмная комната
    // Настраиваем глобальный фонарик
    flashlight.setRadius(15.0f);
    flashlight.setColor(1.0f, 0.95f, 0.8f);
    flashlight.setIntensity(1.2f);
    flashlight.setAttenuation(1.0f, 0.1f, 0.01f);  // добавить затухание
    flashlight.enable();
    
    setup_camera(camera.fov, camera.eye_x, camera.eye_y, camera.eye_z, pitch, yaw);
    set_panorama("src/stargazer.png");
    enable_fog(0.05,0.1,0.1,0.7,5,15);
    
    if (glb->load("src/core_fanmade.glb")) {
        glb->setScale(1.0f);
    }
    
    init_tick_system();
    glutDisplayFunc(display);
    glutIdleFunc(update);
    glutKeyboardFunc(keyboard_down);
    glutKeyboardUpFunc(keyboard_up);
    glutSpecialFunc(special_down);
    glutSpecialUpFunc(special_up);
    glutMainLoop();
return 0;
} 