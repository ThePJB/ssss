use crate::scene::*;
use crate::kinput::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use glutin::event::VirtualKeyCode;

pub struct Game {
    player_x: f64,
    player_y: f64,

    enemies_pos: Vec<Vec2>,

    t: f64,
    t_last_spawn: f64,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player_x: 0.1,
            player_y: 0.5,
            enemies_pos: Vec::new(),
            t: 0.0,
            t_last_spawn: 0.0,
        }
    }
}

impl Demo for Game {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        let player_w = 0.05;
        let player_h = 0.05;
        let player_speed = 0.5;
        let enemy_speed = 0.1;

        let laser_h = 0.01;

        let sky_colour = Vec4::new(0.6, 0.6, 1.0, 1.0);
        let player_colour = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let laser_colour = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let enemy_colour = Vec4::new(0.0, 0.0, 1.0, 1.0);

        let enemy_w = 0.04;
        let enemy_h = 0.04;

        self.t += inputs.dt;

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

        // outputs.canvas.put_rect(inputs.screen_rect, 1.0, sky_colour);
        outputs.canvas.put_rect(player_rect, 2.0, player_colour);

        let laser_rect =
            if inputs.key_held(VirtualKeyCode::Space) {
                let laser_rect = Rect::new(self.player_x, self.player_y - laser_h/2.0, 100.0, laser_h);
                outputs.canvas.put_rect(laser_rect, 1.5, laser_colour);
                Some(laser_rect)
            } else {
                None
            };

        if self.t - self.t_last_spawn > 0.5 {
            self.enemies_pos.push(Vec2::new(inputs.screen_rect.w + enemy_w/2.0, krand(inputs.seed)));
            self.t_last_spawn = self.t;
        }

        for mut enemy_pos in self.enemies_pos.iter_mut() {
            enemy_pos.x -= enemy_speed * inputs.dt;
        }

        for enemy_pos in self.enemies_pos.iter() {
            let r = enemy_pos.rect_centered(enemy_w, enemy_h);
            if r.overlaps(player_rect).is_some() {
                println!("dead");
            }
            outputs.canvas.put_rect(r, 1.5, enemy_colour);
        }
        if let Some(laser_rect) = laser_rect {
            self.enemies_pos.retain(|v| v.rect_centered(enemy_w, enemy_h).overlaps(laser_rect).is_none())
        }
        
        let tb = background(self.t, inputs.screen_rect.aspect(), 1);
        outputs.set_texture.push((tb, 0));
        outputs.draw_texture.push((inputs.screen_rect, 0));

        // pixel background mountains
            
    }
}

fn background(t: f64, a: f64, seed: u32) -> TextureBuffer {
    let h = 100 as usize;
    let w = (h as f64 * a) as usize;

    let mut tb = TextureBuffer::new(w, h);

    let sky = Vec4::new(0.6, 0.6, 1.0, 1.0);
    let cmtn_fg = Vec4::new(0.0, 0.6, 0.4, 1.0).hsv_to_rgb();
    let cmtn_mg = Vec4::new(0.0, 0.6, 0.7, 1.0).hsv_to_rgb();
    let cmtn_bg = Vec4::new(0.0, 0.6, 1.0, 1.0).hsv_to_rgb();

    // t to px
    let px_t = 0.05;


    for i in 0..w {
        // 1d fractal noise for mountains
        let bg_t = 0.5 * t + px_t * i as f64;
        let mg_t = 1.0 * t + px_t * i as f64;
        let fg_t = 4.0 * t + px_t * i as f64;

        let mtn_bg = f1d(bg_t, seed);
        let mtn_mg = f1d(mg_t, seed * 1241247);
        let mtn_fg = f1d(fg_t, seed * 123351561);

        let mtn_fg = mtn_fg/4.0 - 0.1;
        let mtn_mg = mtn_mg/4.0;
        let mtn_bg = mtn_bg/4.0 + 0.1;
        
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