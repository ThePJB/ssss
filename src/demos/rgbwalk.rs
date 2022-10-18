use crate::{scene::{Demo, FrameOutputs}, kinput::FrameInputState};
use crate::kmath::*;
use crate::texture_buffer::*;

pub struct RGBWalk {
    grid: Vec<Vec4>,
    heat: Vec<u64>,
    highest_heat: u64,
    lowest_heat: u64,
    step: u64,
    w: usize,
    h: usize,

    head_colour: [u8; 3],
    head_x: i32,
    head_y: i32,

    steps_per_frame: usize,

    evil_head: bool,
    frame: i32,

    rand_state: u32,
    states: Vec<u32>,
    states_back: usize,
    states_max: usize,
    states_to_go: usize,
    first_new: bool,
}
// clamp to edge
// black and white
// opposite

// another thats opposite in terms of colour and position

// if the rule was decided by the difference between the last time and now you visited

impl RGBWalk {
    pub fn new(w: usize, h: usize, seed: u32) -> RGBWalk {
        let head_colour = [0, 255, 255];

        let head_x = (w/2) as i32;
        let head_y = (h/2) as i32;

        let grid = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); w*h];
        let heat = vec![0; w*h];

        RGBWalk {
            grid,
            heat,
            step: 0,
            highest_heat: 0,
            lowest_heat: 0,
            w,
            h,
            head_colour,
            head_x,
            head_y,
            steps_per_frame: 100,
            evil_head: true,
            frame: 0,

            rand_state: seed,
            states: vec![],
            states_back: 0,
            states_max: 4096,
            states_to_go: 0,
            first_new: true,
        }
    }
}

impl Default for RGBWalk {
    fn default() -> Self {
        Self::new(200, 200, 2)
    }
}

impl Demo for RGBWalk {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        self.frame += 1;

        let steps_per_frame = (((self.frame as f64 * 0.01).sin() + 1.0) * self.steps_per_frame as f64) as usize;

        // head colour
        // de grey ifying: bottom one gets zerod

        for i in 0..steps_per_frame {
            if self.step % 10000 == 0 {
                for i in 0..self.w {
                    for j in 0..self.h {
                        if self.heat[(self.w * j + i)] < self.lowest_heat {
                            self.heat[(self.w * j + i)] = self.lowest_heat;
                        }
                    }
                }
            }
            self.step += 1;
            let hc_min = self.head_colour.iter().enumerate().min_by_key(|x| x.1).map(|x| x.0).unwrap();
            let mut head_colour = self.head_colour;
            // let head_colour = [255, 255, 255];
            // head_colour[hc_min] = 0;

            self.grid[(self.w as i32 * self.head_y + self.head_x) as usize] = Vec4::new(head_colour[0] as f64 / 255.0, head_colour[1] as f64 / 255.0, head_colour[2] as f64 / 255.0, 1.0);
            self.heat[(self.w as i32 * self.head_y + self.head_x) as usize] += 1;
            if self.heat[(self.w as i32 * self.head_y + self.head_x) as usize] > self.highest_heat {
                self.highest_heat = self.heat[(self.w as i32 * self.head_y + self.head_x) as usize];
            }

            if self.evil_head {
                // self.grid[(self.w as i32 * (self.h as i32 - self.head_y - 1) + (self.w as i32 - self.head_x - 1)) as usize] = Vec4::new(1.0 - head_colour[0] as f64 / 255.0, 1.0 - head_colour[1] as f64 / 255.0, 1.0 - head_colour[2] as f64 / 255.0, 1.0);
                self.grid[(self.w as i32 * (self.h as i32 - self.head_y - 1) + (self.w as i32 - self.head_x - 1)) as usize] = Vec4::new(head_colour[0] as f64 / 255.0, head_colour[1] as f64 / 255.0, head_colour[2] as f64 / 255.0, 1.0);
                self.heat[(self.w as i32 * (self.h as i32 - self.head_y - 1) + (self.w as i32 - self.head_x - 1)) as usize] += 1;
            }
            if self.states_to_go > 0 {
                self.rand_state = self.states[self.states.len() - self.states_to_go];
                self.states_to_go -= 1;
                self.first_new = true;
            } else {
                if self.states.len() < self.states_max {
                    self.states.push(self.rand_state);
                } else {
                    self.states[self.states_back] = self.rand_state;
                    self.states_back = (self.states_back + 1) % self.states_max;
                }
                self.rand_state = khash(self.rand_state);
                if !self.first_new {
                    // skip back
                    // ok I like 1/n but lets just have like a small chance to go back a bit
                    // also just powers of 2 would be cool
                    // and actually  it would be ideal if the sequence generator was genuine so if i have like a few small do 2s they get repeated again in the larger cycles. duh
                    // just wow such logic. probably make a small thing

                    // simplification: actual seed and applied seed.
                    // seed changes happen off actual seed

                    // you could givei t a stack that fills and empties (and does something when it overflows)

                    // for i in 0..self.states_max {
                    //     if khash
                    // }
                    // self.states_back
                }
                self.first_new = false;
            }


            let (dx, dy) = [(-1, 0), (1, 0), (0, -1), (0, 1)][(khash(self.rand_state + 132847177*i as u32) % 4) as usize];
            self.head_x = (self.head_x + dx + self.w as i32) % self.w as i32;
            self.head_y = (self.head_y + dy + self.h as i32) % self.h as i32;
            let hc_idx = khash(self.rand_state + 10347177 * i as u32) as usize % 3;
            self.head_colour[hc_idx] = if self.head_colour[hc_idx] == 0 {
                self.head_colour[hc_idx] + 1
            } else if self.head_colour[hc_idx] == 255 {
                self.head_colour[hc_idx] - 1
            } else {
                [self.head_colour[hc_idx] + 1, self.head_colour[hc_idx] - 1][(khash(self.rand_state + 11319567 * i as u32)) as usize % 2]
            };
        }

        // render
        let tw = self.w;
        let th = self.h;
        let mut t = TextureBuffer::new(tw, th);
        for i in 0..tw {
            for j in 0..th {
                t.set(i as i32, j as i32, self.grid[j * self.w + i]);
                // let heat = (self.heat[j * self.w + i] - self.lowest_heat) as f64 / self.highest_heat as f64;
                // t.set(i as i32, j as i32, heat * Vec4::new(1.0, 0.0, 0.0, 1.0));
            }
        }
        outputs.set_texture.push((t, 0));
        outputs.draw_texture.push((inputs.screen_rect, 0));

    }
}