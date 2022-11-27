use std::time::SystemTime;

use crate::scene::*;
use crate::kinput::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use glutin::event::VirtualKeyCode;

pub struct Game {
    player_x: f64,
    player_y: f64,

    enemies_pos: Vec<Vec2>,
    enemies_v: Vec<Vec2>,
    enemies_t_last_shoot: Vec<f64>,
    enemies_type: Vec<usize>,
    enemies_dob: Vec<f64>,
    bullets_pos: Vec<Vec2>,

    t: f64,
    t_last_spawn: [f64; 3],
    seed: u32,

    alive: bool,
    score: i32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            alive: true,
            player_x: 0.1,
            player_y: 0.5,
            enemies_pos: Vec::new(),
            enemies_t_last_shoot: Vec::new(),
            enemies_v: Vec::new(),
            enemies_type: Vec::new(),
            enemies_dob: Vec::new(),
            bullets_pos: Vec::new(),
            t: 0.0,
            t_last_spawn: [0.0; 3],
            seed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_nanos() as u32).unwrap_or(1),
            score: 0,
        }
    }
}

impl Demo for Game {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        if inputs.key_rising(VirtualKeyCode::R) {
            *self = Game::default()
        }

        let player_w = 0.08;
        let player_h = 0.05;
        let player_speed = 0.5;
        
        let laser_h = 0.005;
        
        let player_border_colour = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let player_inner_colour = Vec4::new(120.0, 0.5, 1.0, 1.0).hsv_to_rgb();
        let laser_colour = Vec4::new(1.0, 0.0, 0.0, 1.0);
        
        let enemy_w = [0.04, 0.04, 0.03];
        let enemy_h = [0.04, 0.04, 0.03];
        let enemy_shoot_interval = [1.01, 0.36, 0.3];
        let enemy_spawn_interval = [1.0, 0.2, 0.1];
        let enemy_spawn_after = [0.0, 10.0, 20.0];
        let enemy_spawn_duty_cycle = [1.0, 0.3, 0.1];
        let enemy_spawn_len = [1.0, 3.0, 1.0];
        let enemy_speed = [0.1, 0.15, 0.2];
        let enemy_colour = [Vec4::new(0.0, 0.0, 1.0, 1.0), Vec4::new(1.0, 0.0, 0.0, 1.0), Vec4::new(0.0, 1.0, 0.0, 1.0),];

        let bullet_w = 0.03;
        let bullet_h = 0.01;
        let bullet_colour = Vec4::new(1.0, 1.0, 0.0, 1.0);
        let bullet_speed = 0.3;

        let mut dt = inputs.dt;

        if self.alive {
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
        if self.alive {
            outputs.canvas.put_triangle_struct(ptri, 1.6, player_border_colour);
            outputs.canvas.put_triangle_struct(ptri_inner, 1.7, player_inner_colour);
        }

        let laser_rect =
            if inputs.key_held(VirtualKeyCode::Space) {
                let laser_rect = Rect::new(self.player_x, self.player_y - laser_h/2.0, 100.0, laser_h);
                outputs.canvas.put_rect(laser_rect, 1.5, laser_colour);
                Some(laser_rect)
            } else {
                None
            };

        for et in 0..3 {
            if self.t > enemy_spawn_after[et] && self.t - self.t_last_spawn[et] > enemy_spawn_interval[et] && ((self.t - enemy_spawn_after[et]) % (enemy_spawn_len[et]/enemy_spawn_duty_cycle[et])) < enemy_spawn_len[et] {

                let sp = [
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, krand(inputs.seed)),
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, 0.5),
                    Vec2::new(inputs.screen_rect.w + enemy_w[et]/2.0, krand(inputs.seed)),
                ];

                self.enemies_pos.push(sp[et]);
                self.enemies_t_last_shoot.push(self.t);
                self.enemies_v.push(Vec2::new(-enemy_speed[et], 0.0));
                self.enemies_type.push(et);
                self.enemies_dob.push(self.t);
                self.t_last_spawn[et] = self.t;
            }
        }

        for i in 0..self.enemies_v.len() {
            let et = self.enemies_type[i];
            let vy = [
                0.1 * (noise1d(self.t, self.seed + i as u32 * 12312487) - 0.5) * 2.0,
                -0.3 * (self.t - self.enemies_dob[i]).cos(),
                0.3 * (noise1d(self.t, self.seed + i as u32 * 12312487) - 0.5) * 2.0,
            ];
            self.enemies_v[i].y = vy[et];
            self.enemies_pos[i] = self.enemies_pos[i] + dt * self.enemies_v[i];
            let r = self.enemies_pos[i].rect_centered(enemy_w[et], enemy_h[et]);
            if r.overlaps(player_rect).is_some() {
                self.alive = false;
            }
            outputs.canvas.put_rect(r, 1.5, enemy_colour[et]);
        }

        if let Some(laser_rect) = laser_rect {
            let mut i = self.enemies_pos.len();
            loop {
                if i == 0 {
                    break;
                }
                i -= 1;

                let et = self.enemies_type[i];
                let er = self.enemies_pos[i].rect_centered(enemy_w[et], enemy_h[et]);

                if er.overlaps(laser_rect).is_some() {
                    self.enemies_pos.swap_remove(i);
                    self.enemies_t_last_shoot.swap_remove(i);
                    self.enemies_v.swap_remove(i);
                    self.enemies_dob.swap_remove(i);
                    self.enemies_type.swap_remove(i);
                    self.score += 1;
                }
            }
        }

        for i in 0..self.enemies_pos.len() {
            let et = self.enemies_type[i];

            if self.t - self.enemies_t_last_shoot[i] > enemy_shoot_interval[et] {
                self.bullets_pos.push(self.enemies_pos[i]);
                self.enemies_t_last_shoot[i] = self.t;
            }
        }

        for bullet_pos in self.bullets_pos.iter_mut() {
            bullet_pos.x -= dt * bullet_speed;
        }

        for bullet_pos in self.bullets_pos.iter() {
            outputs.canvas.put_rect(bullet_pos.rect_centered(bullet_w, bullet_h), 2.0, bullet_colour);
        }

        for bullet_pos in self.bullets_pos.iter() {
            if ptri.dilate(bullet_h).contains(*bullet_pos) {
                self.alive = false;
            }
        }

        if self.alive {
            outputs.glyphs.push_str(format!("score: {}", self.score).as_str(), 0.02, 0.02, 0.03, 0.03, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
        } else {
            let x = inputs.screen_rect.w/2.0;
            let mut y = inputs.screen_rect.h * 0.3;
            outputs.glyphs.push_center_str("you died", x, y, 0.08, 0.08, 2.1, Vec4::new(1.0, 0.0, 0.0, 1.0));
            y += 0.1;
            outputs.glyphs.push_center_str(format!("score: {}", self.score).as_str(), x, y, 0.04, 0.08, 2.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
            y += 0.1;
            outputs.glyphs.push_center_str("press r to play again", x, y, 0.04, 0.04, 2.1, Vec4::new(1.0, 1.0, 1.0, 1.0));
        }

        self.bullets_pos.retain(|x| !ptri.dilate(bullet_h).contains(*x) && x.x > -1.0);
        
        let tb = background(self.t, self.seed, 400, (inputs.screen_rect.aspect() * 400.0) as usize);
        // let tb = background(self.t, self.seed, 400, 400);
        outputs.set_texture.push((tb, 0));
        outputs.draw_texture.push((inputs.screen_rect, 0));

        // pixel background mountains
            
    }
}

fn background(t: f64, seed: u32, w: usize, h: usize) -> TextureBuffer {

    let mut tb = TextureBuffer::new(w, h);

    let sky = Vec4::new(0.6, 0.6, 1.0, 1.0);
    let cmtn_fg = Vec4::new(0.0, 0.6, 0.4, 1.0).hsv_to_rgb();
    let cmtn_mg = Vec4::new(0.0, 0.6, 0.7, 1.0).hsv_to_rgb();
    let cmtn_bg = Vec4::new(0.0, 0.6, 1.0, 1.0).hsv_to_rgb();

    // t to px
    let px_t = 0.01;


    for i in 0..w {
        // 1d fractal noise for mountains
        let bg_t = 0.5 * t + px_t * i as f64;
        let mg_t = 1.0 * t + px_t * i as f64;
        let fg_t = 4.0 * t + px_t * i as f64;

        let mtn_bg = f1d(bg_t, seed);
        let mtn_mg = f1d(mg_t, seed * 1241247);
        let mtn_fg = f1d(fg_t, seed * 123351561);

        let mtn_fg = mtn_fg/4.0 - 0.15;
        let mtn_mg = mtn_mg/4.0;
        let mtn_bg = mtn_bg/4.0 + 0.15;
        
        for j in 0..h {
            let vertical_pos = j as f64 / h as f64;
            
            let c = if vertical_pos < mtn_fg {
                cmtn_fg
            } else if vertical_pos < mtn_mg {
                cmtn_mg
            } else if vertical_pos < mtn_bg {
                cmtn_bg
            } else {
                let cloudness_vert = (vertical_pos-0.5).max(0.0);
                let cloudness_noise = cloud_noise(i as f64 + t * 10.0, j as f64, seed * 15417);
                sky.lerp(Vec4::new(1.0, 1.0, 1.0, 1.0), cloudness_vert * cloudness_noise)
            };

            tb.set(i as i32, j as i32, c);
            // tb.set(i as i32, j as i32, Vec4::new(0.0, 0.0, 0.0, 1.0));
        }
    }
    tb
}

fn f1d(t: f64, seed: u32) -> f64 {
    1.000 * noise1d(t, seed) + 
    0.500 * noise1d(t, seed * 14147) + 
    0.250 * noise1d(t, seed * 141879177) + 
    0.125 * noise1d(t, seed * 13212487) /
    1.5875
}

fn cloud_noise(x: f64, y: f64, seed: u32) -> f64 {
    let n1 = noise2d(x/100.0, y/10.0, seed);
    // let n2 = noise2d(x/5.0, y/5.0, seed * 154171234);
    // n1 * n2
    n1
}