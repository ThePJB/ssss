use std::time::SystemTime;

use crate::scene::*;
use crate::kinput::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::audio::*;
use crate::sound_instance::*;
use glutin::event::VirtualKeyCode;

pub struct Game {
    player_x: f32,
    player_y: f32,
    player_laser_heat: f32,
    player_laser_discharge: bool,
    player_alive: bool,

    enemies_pos: Vec<Vec2>,
    enemies_v: Vec<Vec2>,
    enemies_t_last_shoot: Vec<f32>,
    enemies_type: Vec<usize>,
    enemies_dob: Vec<f32>,
    bullets_pos: Vec<Vec2>,
    
    powerup_t_last: f32,
    powerup_collect_t_last: f32,
    powerup_pos: Option<Vec2>,
    powerup_number: u32,

    t: f32,
    t_last_spawn: [f32; 4],
    seed: u32,

    score: i32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player_x: 0.1,
            player_y: 0.5,
            player_alive: true,
            player_laser_heat: 0.0,
            player_laser_discharge: false,
            enemies_pos: Vec::new(),
            enemies_t_last_shoot: Vec::new(),
            enemies_v: Vec::new(),
            enemies_type: Vec::new(),
            enemies_dob: Vec::new(),
            bullets_pos: Vec::new(),
            powerup_pos: None,
            powerup_t_last: 0.0,
            powerup_collect_t_last: -10.0,
            powerup_number: 0,
            t: 0.0,
            t_last_spawn: [0.0; 4],
            seed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_nanos() as u32).unwrap_or(1),
            score: 0,
        }
    }
}

impl Demo for Game {
    fn frame(&mut self, inputs: &FrameInputs, outputs: &mut FrameOutputs) {
        if inputs.key_rising(VirtualKeyCode::R) {
            *self = Game::default();
            outputs.audio_events.push(LASER);
            outputs.audio_events.push(ENEMY_DIE);
            outputs.audio_events.push(ENEMY_SHOOT);
            outputs.audio_events.push(PLAYER_DIE);
        }

        let player_w = 0.04;
        let player_h = 0.025;
        let player_speed = 0.5;
        
        let laser_h = 0.005;
        
        let player_border_colour = v4(0.0, 0.0, 0.0, 1.0);
        let player_inner_colour = v4(120.0, 0.5, 1.0, 1.0).hsv_to_rgb();
        let laser_colour = v4(1.0, 0.0, 0.0, 1.0);
        
        let enemy_w = [0.05, 0.04, 0.03, 0.05];
        let enemy_h = [0.05, 0.04, 0.03, 0.05];
        let enemy_shoot_interval = [1.41, 0.8, 0.7, f32::INFINITY];
        let enemy_spawn_interval = [1.0, 0.2, 0.1, 5.0];
        let enemy_spawn_after = [0.0, 10.0, 20.0, 30.0];
        let enemy_spawn_duty_cycle = [1.0, 0.21, 0.1, 1.0];
        let enemy_spawn_len = [1.0, 3.0, 0.6, 0.1];
        let enemy_speed = [0.1, 0.2, 0.25, 0.4];
        let enemy_colour = [v4(0.0, 0.0, 1.0, 1.0), v4(1.0, 0.0, 0.0, 1.0), v4(0.0, 0.9, 0.0, 1.0), v4(0.7, 0.0, 0.7, 1.0)];

        let powerup_speed = 0.4;    // or as a drop from the purple guys
                                    // or a powerup laser that kill bullets too
                                    // or kill all bullets powerup, this is powerup to kill all enemies

        let bullet_w = 0.03;
        let bullet_h = 0.01;
        let bullet_colour = Vec4::new(1.0, 1.0, 0.0, 1.0);
        let bullet_speed = 0.6;

        let mut dt = inputs.dt;


        /////////////////////////////////////////////
        // Player
        /////////////////////////////////////////////
        if self.player_alive {
            self.t += inputs.dt;
        } else {
            dt = 0.0;
        }

        let steer_y = if inputs.key_held(VirtualKeyCode::W) {
            -1.0
        } else if inputs.key_held(VirtualKeyCode::S) {
            1.0
        } else {
            0.0
        };
            
        let steer_x = if inputs.key_held(VirtualKeyCode::D) {
            1.0
        } else if inputs.key_held(VirtualKeyCode::A) {
            -1.0
        } else {
            0.0
        };

        let steer = Vec2::new(steer_x, steer_y).normalize();
        let v = steer * player_speed * inputs.dt;
        self.player_x += v.x;
        self.player_y += v.y;

        let player_rect = Rect::new_centered(self.player_x, self.player_y, player_w, player_h);

        let ptl = Vec2::new(self.player_x - player_w/2.0, self.player_y + player_h/2.0);
        let pbl = Vec2::new(self.player_x - player_w/2.0, self.player_y - player_h/2.0);
        let pr = Vec2::new(self.player_x + player_w/2.0, self.player_y);
        let ptri = Triangle::new(ptl, pr, pbl);
        let ptri_inner = ptri.dilate(-0.3);
        if self.player_alive {
            outputs.canvas.put_triangle_struct(ptri, 1.6, player_border_colour);
            outputs.canvas.put_triangle_struct(ptri_inner, 1.7, player_inner_colour);
        }


        ///////////////////////////////////
        // Laser
        ///////////////////////////////////
        let shoot_laser = inputs.key_held(VirtualKeyCode::Space) && !self.player_laser_discharge && self.player_alive;
        let laser_rect = if shoot_laser {
            Some(Rect::new(self.player_x, self.player_y - laser_h/2.0, 100.0, laser_h))
        } else {
            None
        };
        if let Some(laser_rect) = laser_rect {
            outputs.canvas.put_rect(laser_rect, 1.5, laser_colour);
        }
        if shoot_laser {
            outputs.audio_events.push(SOUND_PLAY | LASER | SOUND_UNIQUE);
            self.player_laser_heat += dt;
            if self.player_laser_heat > 1.0 {
                self.player_laser_heat = 1.0;
                self.player_laser_discharge = true;
                outputs.audio_events.push(LASER_POP | SOUND_PLAY);
            }
        }
        if !shoot_laser {
            outputs.audio_events.push(LASER);
            self.player_laser_heat -= dt;
            if self.player_laser_heat <= 0.0 {
                self.player_laser_heat = 0.0;
                self.player_laser_discharge = false;
            }
        }


        /////////////////////////////////////////
        // Powerups
        /////////////////////////////////////////
        if self.t - self.powerup_t_last > kuniform(1241235417 * self.powerup_number + 1234125417, 10.0, 20.0) && self.powerup_pos.is_none() {
            let y = kuniform(1361723497 * self.powerup_number + 9323717, 0.0, 1.0);
            let x = inputs.screen_rect.w + 0.05;
            self.powerup_pos = Some(Vec2::new(x, y));
            self.powerup_number += 1;
            self.powerup_t_last = self.t;
        }

        let mut pp_off = false;
        let mut pp_collected = false;
        if let Some(ppos) = self.powerup_pos.as_mut() {
            ppos.x -= powerup_speed * dt;
            if ppos.x < -0.1 {
                pp_off = true;
            }
            let r = 0.03;
            let pp_rect = Rect::new_centered(ppos.x, ppos.y, r, r);
            if pp_rect.overlaps(player_rect).is_some() {
                pp_off = true;
                pp_collected = true;
                self.powerup_collect_t_last = self.t;
                outputs.audio_events.push(SOUND_PLAY | POWERUP);
            }
            outputs.canvas.put_triangle(
                r * Vec2::new((inputs.t * TAU/3.0).sin(), (inputs.t * TAU/3.0).cos()) + *ppos, 
                r * Vec2::new((inputs.t * TAU/3.0 + TAU/3.0).sin(), (inputs.t * TAU/3.0  + TAU/3.0).cos()) + *ppos, 
                r * Vec2::new((inputs.t * TAU/3.0 + 2.0 * TAU/3.0).sin(), (inputs.t * TAU/3.0  + 2.0 * TAU/3.0).cos()) + *ppos, 
            3.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
            outputs.canvas.put_triangle(
                r * Vec2::new((-inputs.t * TAU/3.0).sin(), (-inputs.t * TAU/3.0).cos()) + *ppos, 
                r * Vec2::new((-inputs.t * TAU/3.0 + TAU/3.0).sin(), (-inputs.t * TAU/3.0  + TAU/3.0).cos()) + *ppos, 
                r * Vec2::new((-inputs.t * TAU/3.0 + 2.0 * TAU/3.0).sin(), (-inputs.t * TAU/3.0  + 2.0 * TAU/3.0).cos()) + *ppos, 
            2.9, Vec4::new(1.0, 1.0, 1.0, 1.0));
        }

        if pp_off {
            self.powerup_pos = None;
        }

        if self.t - self.powerup_collect_t_last < 0.0707 {
            outputs.canvas.put_rect(inputs.screen_rect, 1.4, v4(1., 1., 1., 1.));
        }
        


        ////////////////////////////////////////////
        // Enemy Spawning
        ////////////////////////////////////////////
        for et in 0..4 {
            if self.t > enemy_spawn_after[et] && self.t - self.t_last_spawn[et] > enemy_spawn_interval[et] && ((self.t - enemy_spawn_after[et]) % (enemy_spawn_len[et]/enemy_spawn_duty_cycle[et])) < enemy_spawn_len[et] {

                let sp = [
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, krand(inputs.seed)),
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, 0.5),
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, krand(inputs.seed)),
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, krand(inputs.seed)),
                ];

                self.enemies_pos.push(sp[et]);
                self.enemies_t_last_shoot.push(self.t);
                self.enemies_v.push(Vec2::new(-enemy_speed[et], 0.0));
                self.enemies_type.push(et);
                self.enemies_dob.push(self.t);
                self.t_last_spawn[et] = self.t;
                outputs.audio_events.push(SOUND_PLAY | ENEMY_SPAWN);
            }
        }


        /////////////////////////////////////////////////
        // Enemy Movement
        /////////////////////////////////////////////////
        for i in 0..self.enemies_v.len() {
            let et = self.enemies_type[i];
            let vy = [
                0.1 * (noise1d(self.t, self.seed + i as u32 * 12312487) - 0.5) * 2.0,
                -0.3 * (self.t - self.enemies_dob[i]).cos(),
                0.3 * (noise1d(self.t, self.seed + i as u32 * 12312487) - 0.5) * 2.0,
                0.3 * (10.0*(self.t - self.enemies_dob[i])).cos(),
            ];
            self.enemies_v[i].y = vy[et];
            self.enemies_pos[i] = self.enemies_pos[i] + dt * self.enemies_v[i];
            let p = self.enemies_pos[i];
            let r = p.rect_centered(enemy_w[et], enemy_h[et]);
            let c = enemy_colour[et];

            match et {
                0 => {
                    let rh = 0.02;
                    outputs.canvas.put_rect(p.rect_centered(0.05, rh), 1.5, c);
                    outputs.canvas.put_circle(p + v2(0.0, -rh/2.0), 0.025, 1.5,  c);
                    outputs.canvas.put_circle(p + v2(-0.05/4.0, rh/2.0), 0.025/2.0, 1.5,  c);
                    outputs.canvas.put_circle(p + v2(0.05/4.0, rh/2.0), 0.025/2.0, 1.5,  c);
                    outputs.canvas.put_circle(p + v2(0., -0.01), 0.025/2.5, 1.51,  v4(0.5, 0.8, 1., 1.));
                },
                1 => {
                    let rh = 0.02;
                    let rw = 0.02;
                    outputs.canvas.put_triangle(p + v2(0., -rh), p + v2(rw, -rh), p + v2(rw, rh), 1.5, v4(1., 1., 0., 1.));
                    outputs.canvas.put_triangle(p + v2(0., rh), p + v2(rw, -rh), p + v2(rw, rh), 1.5, v4(1., 1., 0., 1.));
                    outputs.canvas.put_triangle(p + v2(-rw, 0.0), p + v2(rw, -rh), p + v2(rw, rh), 1.5, v4(1., 0., 0., 1.));
                },
                2 => {
                    outputs.canvas.put_vpill(p, 0.03, 0.01, 1.5, c);
                    outputs.canvas.put_vpill(p, 0.02, 0.08, 1.5, c);
                    outputs.canvas.put_circle(p, 0.007, 1.51, v4(0.3, 0.3, 0.3, 1.0));
                },
                3 => {
                    let r = 0.03;
                    let phase = 2.0 * self.t;
                    outputs.canvas.put_circle(p, r, 1.5, c);
                    outputs.canvas.put_circle(p + 3.*r/2. * v2(phase.sin(), phase.cos()), r/2., 1.5, c);
                    outputs.canvas.put_circle(p + 3.*r/2. * v2((phase + TAU/4.0).sin(), (phase + TAU/4.0).cos()), r/2., 1.5, c);
                    outputs.canvas.put_circle(p + 3.*r/2. * v2((phase + 2.0*TAU/4.0).sin(), (phase + 2.0*TAU/4.0).cos()), r/2., 1.5, c);
                    outputs.canvas.put_circle(p + 3.*r/2. * v2((phase + 3.0*TAU/4.0).sin(), (phase + 3.0*TAU/4.0).cos()), r/2., 1.5, c);
                }
                _ => outputs.canvas.put_rect(r, 1.5, enemy_colour[et]),
            }            
            
            let r = r.dilate_pc(-0.2);
            if r.overlaps(player_rect).is_some() && self.player_alive {
                self.player_alive = false;
                outputs.audio_events.push(SOUND_PLAY | PLAYER_DIE);
            }
        }


        ///////////////////////////////////////////
        // Enemy Death
        ///////////////////////////////////////////
        let mut i = self.enemies_pos.len();
        loop {
            if i == 0 {
                break;
            }
            i -= 1;

            let et = self.enemies_type[i];
            let er = self.enemies_pos[i].rect_centered(enemy_w[et], enemy_h[et]);

            let death_by_laser = laser_rect.is_some() && laser_rect.unwrap().overlaps(er).is_some();

            if er.right() < 0.0 || death_by_laser || pp_collected {
                self.enemies_pos.swap_remove(i);
                self.enemies_t_last_shoot.swap_remove(i);
                self.enemies_v.swap_remove(i);
                self.enemies_dob.swap_remove(i);
                self.enemies_type.swap_remove(i);
                self.score += 1;
                if !pp_collected {
                    outputs.audio_events.push(SOUND_PLAY | ENEMY_DIE);
                }
            }
        }


        ////////////////////////////
        // Shooting
        ////////////////////////////
        for i in 0..self.enemies_pos.len() {
            let et = self.enemies_type[i];

            if self.t - self.enemies_t_last_shoot[i] > enemy_shoot_interval[et] {
                self.bullets_pos.push(self.enemies_pos[i]);
                self.enemies_t_last_shoot[i] = self.t;
                outputs.audio_events.push(SOUND_PLAY | ENEMY_SHOOT);
            }
        }


        ///////////////////////////////
        // Bullets
        ///////////////////////////////
        for bullet_pos in self.bullets_pos.iter_mut() {
            bullet_pos.x -= dt * bullet_speed;
        }
        for bullet_pos in self.bullets_pos.iter() {
            outputs.canvas.put_rect(bullet_pos.rect_centered(bullet_w, bullet_h), 1.8, bullet_colour);
        }
        for bullet_pos in self.bullets_pos.iter() {
            if ptri.dilate(bullet_h).contains(*bullet_pos) {
                self.player_alive = false;
                outputs.audio_events.push(SOUND_PLAY | PLAYER_DIE);
            }
        }
        self.bullets_pos.retain(|x| !ptri.dilate(bullet_h).contains(*x) && x.x > -1.0);
        

        /////////////////////////////////////
        // Interface
        /////////////////////////////////////
        outputs.glyphs.push_str(format!("score: {}", self.score).as_str(), 0.02, 0.02, 0.03, 0.03, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
        if self.player_alive {

            if self.t < 3.0 {
                let x = inputs.screen_rect.w/2.0;
                let mut y = inputs.screen_rect.h * 0.3;
                outputs.glyphs.push_center_str("wasd - move", x, y, 0.04, 0.04, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
                let mut y = inputs.screen_rect.h * 0.6;
                outputs.glyphs.push_center_str("space - shoot", x, y, 0.04, 0.04, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
                
            }
        } else {
            let banner_rect = inputs.screen_rect.child(0.0, 0.33, 1.0, 0.25);
            outputs.canvas.put_rect(banner_rect, 1.9, Vec4::new(0.0, 0.0, 0.0, 1.0));
            outputs.canvas.put_rect(banner_rect.dilate_pc(0.01), 1.85, Vec4::new(1.0, 0.0, 0.0, 1.0));

            let x = inputs.screen_rect.w/2.0;
            let mut y = inputs.screen_rect.h * 0.37;
            outputs.glyphs.push_center_str("you died", x, y, 0.08, 0.08, 2.1, Vec4::new(1.0, 0.0, 0.0, 1.0));
            y += 0.1;
            outputs.glyphs.push_center_str(format!("score: {}", self.score).as_str(), x, y, 0.08, 0.08, 2.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
            y = 0.66;
            if inputs.t % 2.0 > 1.0 {
                outputs.glyphs.push_center_str("press r to play again", x, y, 0.04, 0.04, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
            }
        }

        
        let tb = background(self.t, self.seed, 400, (inputs.screen_rect.aspect() * 400.0) as usize);
        // let tb = background(self.t, self.seed, 400, 400);
        outputs.set_texture.push((tb, 0));
        outputs.draw_texture.push((inputs.screen_rect, 0));

        

        // pixel background mountains
            
    }
}

fn background(t: f32, seed: u32, w: usize, h: usize) -> TextureBuffer {

    let mut tb = TextureBuffer::new(w, h);

    let sky = Vec4::new(0.6, 0.6, 1.0, 1.0);
    let cmtn_fg = Vec4::new(0.0, 0.6, 0.4, 1.0).hsv_to_rgb();
    let cmtn_mg = Vec4::new(0.0, 0.6, 0.7, 1.0).hsv_to_rgb();
    let cmtn_bg = Vec4::new(0.0, 0.6, 1.0, 1.0).hsv_to_rgb();

    // t to px
    let px_t = 0.01;


    for i in 0..w {
        // 1d fractal noise for mountains
        let bg_t = 0.5 * t + px_t * i as f32;
        let mg_t = 1.0 * t + px_t * i as f32;
        let fg_t = 4.0 * t + px_t * i as f32;

        let mtn_bg = -f1d(bg_t, seed).ln();
        let mtn_mg = -f1d(mg_t, seed * 1241247).ln();
        let mtn_fg = -f1d(fg_t, seed * 123351561).ln();

        let mtn_fg = mtn_fg/4.0 - 0.15;
        let mtn_mg = mtn_mg/4.0;
        let mtn_bg = mtn_bg/4.0 + 0.15;
        
        for j in 0..h {
            let vertical_pos = j as f32 / h as f32;
            
            let c = if vertical_pos < mtn_fg {
                cmtn_fg
            } else if vertical_pos < mtn_mg {
                cmtn_mg
            } else if vertical_pos < mtn_bg {
                cmtn_bg
            } else {
                let cloudness_vert = (vertical_pos-0.5).max(0.0);
                let cloudness_noise = cloud_noise(i as f32 + t * 10.0, j as f32, seed * 15417);
                sky.lerp(Vec4::new(1.0, 1.0, 1.0, 1.0), cloudness_vert * cloudness_noise)
            };

            tb.set(i as i32, j as i32, c);
            // tb.set(i as i32, j as i32, Vec4::new(0.0, 0.0, 0.0, 1.0));
        }
    }
    tb
}

fn f1d(t: f32, seed: u32) -> f32 {
    1.000 * noise1d(t, seed) + 
    0.500 * noise1d(t, seed * 14147) + 
    0.250 * noise1d(t, seed * 141879177) + 
    0.125 * noise1d(t, seed * 13212487) /
    1.5875
}

fn cloud_noise(x: f32, y: f32, seed: u32) -> f32 {
    let n1 = noise2d(x/100.0, y/10.0, seed);
    // let n2 = noise2d(x/5.0, y/5.0, seed * 154171234);
    // n1 * n2
    n1
}