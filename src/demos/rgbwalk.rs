use crate::{scene::{DoFrame, FrameOutputs}, kinput::FrameInputState, kmath::{chance, kuniform}};
use crate::kmath::*;
use crate::texture_buffer::*;

// this could so easily be a map for a game. different walkers, walkers have rules when they meet certain conditions eg walk on another walkers tile, you die, or swap to going straight
// what about a UTM with made up rules, like from a given colour you set the colour to something and set the tile to something and move in a certain direction. might start getting weird patterns
// seed = r ^ g ^ b or r << 16 | g << 8 | b and dir is khash(seed), head colour is khash(seed) and tile colour is head colour. id expect it to find cycles
// does this look like percolation? percolation looks like a map for a game
// give the walker a tendency to maintain its velocity, or let it move 8ways

pub struct RGBWalk {
    grid: Vec<Vec4>,
    w: usize,
    h: usize,

    head_colour: [u8; 3],
    head_x: i32,
    head_y: i32,

    steps_per_frame: usize,
}

impl RGBWalk {
    pub fn new(w: usize, h: usize) -> RGBWalk {
        let seed = 69;
        let head_colour = [0, 255, 255];

        let head_x = (w/2) as i32;
        let head_y = (h/2) as i32;

        let grid = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); w*h];

        RGBWalk {
            grid,
            w,
            h,
            head_colour,
            head_x,
            head_y,
            steps_per_frame: 10000,
        }
    }
}

impl DoFrame for RGBWalk {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {

        for i in 0..self.steps_per_frame {
            self.grid[(self.w as i32 * self.head_y + self.head_x) as usize] = Vec4::new(self.head_colour[0] as f32 / 255.0, self.head_colour[1] as f32 / 255.0, self.head_colour[2] as f32 / 255.0, 1.0);
            let (dx, dy) = [(-1, 0), (1, 0), (0, -1), (0, 1)][(khash(inputs.seed + 132847177*i as u32) % 4) as usize];
            self.head_x = (self.head_x + dx + self.w as i32) % self.w as i32;
            self.head_y = (self.head_y + dy + self.h as i32) % self.h as i32;
            let hc_idx = khash(inputs.seed + 10347177 * i as u32) as usize % 3;
            self.head_colour[hc_idx] = if self.head_colour[hc_idx] == 0 {
                self.head_colour[hc_idx] + 1
            } else if self.head_colour[hc_idx] == 255 {
                self.head_colour[hc_idx] - 1
            } else {
                [self.head_colour[hc_idx] + 1, self.head_colour[hc_idx] - 1][(khash(inputs.seed + 11319567 * i as u32)) as usize % 2]
            };
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