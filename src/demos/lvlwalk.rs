use crate::{scene::{DoFrame, FrameOutputs}, kinput::FrameInputState, kmath::{kuniform}};
use crate::kmath::*;
use crate::texture_buffer::*;
use glutin::event::VirtualKeyCode;


// this could so easily be a map for a game. different walkers, walkers have rules when they meet certain conditions eg walk on another walkers tile, you die, or swap to going straight
// what about a UTM with made up rules, like from a given colour you set the colour to something and set the tile to something and move in a certain direction. might start getting weird patterns
// seed = r ^ g ^ b or r << 16 | g << 8 | b and dir is khash(seed), head colour is khash(seed) and tile colour is head colour. id expect it to find cycles
// does this look like percolation? percolation looks like a map for a game
// give the walker a tendency to maintain its velocity, or let it move 8ways

// invent a stochastic process to make sick levels
// main walker: 1x1
//  open carver: 3x3, sterile
//  bonus path: 1x1, sterile

pub struct Walker {
    bias: Vec2,
    x: i32,
    y: i32,
    gen: i32,
    kind: i32,
    age: i32,
}

pub struct LevelWalk {
    grid: Vec<Vec4>,
    w: usize,
    h: usize,

    walkers: Vec<Walker>,

    steps_per_frame: usize,
}

impl LevelWalk {
    pub fn new(w: usize, h: usize) -> LevelWalk {
        let grid = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); w*h];

        LevelWalk {
            grid,
            w,
            h,
            walkers: vec![Walker {
                bias: Vec2::new(0.0, -0.2),
                x: w as i32/2,
                y: h as i32/2,
                gen: 0,
                kind: 0,
                age: 0,
            }],
            steps_per_frame: 10,
        }
    }
}

fn random_direction_with_bias(seed: u32, bias: Vec2) -> (i32, i32) {
    let weight_l = 1.0 - bias.x/2.0;
    let weight_r = 1.0 + bias.x/2.0;
    let weight_u = 1.0 - bias.y/2.0;
    let weight_d = 1.0 + bias.y/2.0;

    let mut cum = 0.0;
    let target = kuniform(seed, 0.0, weight_l + weight_u + weight_r + weight_d);
    cum += weight_l;
    if cum > target {
        return (-1, 0);
    }
    cum += weight_r;
    if cum > target {
        return (1, 0);
    }
    cum += weight_u;
    if cum > target {
        return (0, -1);
    }
    cum += weight_d;
    if cum > target {
        return (0, 1);
    }
    return (0, 1);
    // panic!("random direction bug");
}

impl DoFrame for LevelWalk {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {

        if inputs.key_rising(VirtualKeyCode::R) {
            *self = LevelWalk::new(self.w, self.h);
        }

        for i in 0..self.steps_per_frame {


            for wi in 0..self.walkers.len() {
                let (dx, dy) = random_direction_with_bias(inputs.seed + wi as u32 * 12317757 + i as u32 * 58972377, self.walkers[wi].bias);

                self.walkers[wi].x = (self.walkers[wi].x + dx + self.w as i32) % self.w as i32;
                self.walkers[wi].y = (self.walkers[wi].y + dy + self.h as i32) % self.h as i32;

                let colour = Vec4::new(self.walkers[wi].gen as f32 * 137.5, 1.0, 1.0, 1.0).hsv_to_rgb();
                self.grid[(self.walkers[wi].y * self.w as i32 + self.walkers[wi].x) as usize] = colour;

                if self.walkers[wi].age > 0 && self.walkers[wi].age % (1000) == 0 && self.walkers[wi].gen < 5 {
                    self.walkers.push(Walker {
                        x: self.walkers[wi].x,
                        y: self.walkers[wi].y,
                        bias: Vec2::new(self.walkers[wi].bias.y, self.walkers[wi].bias.x) * 1.05,
                        gen: self.walkers[wi].gen + 1,
                        age: 0,
                        kind: self.walkers[wi].kind,
                    });
                }
                
                self.walkers[wi].age += 1;
            }

            self.walkers.retain(|w| w.age < 2001);
        }

        // render
        let tw = self.w;
        let th = self.h;
        let mut t = TextureBuffer::new(tw, th);
        for i in 0..tw {
            for j in 0..th {
                t.set(i as i32, j as i32, self.grid[j * self.w + i]);
            }
        }
        outputs.texture = Some(t);
        outputs.texture_rect = Some(inputs.screen_rect);
    }
}

// what about a random walk where the vibe is incremented each time the square is visited
// or derivative random walk