use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

// how to even make this again: from delauney? I must of did it for that kingdom sim
// bowyer watson is the go
// add triangles one at a time

pub struct Voronoinoi {
    w: usize,
    h: usize,
    r: Rect,

}

const MAX_ITERATIONS: i32 = 160;

impl Voronoinoi {
    pub fn new(w: usize, h: usize) -> Voronoinoi {
        let mut colour_palette = Vec::new();

        colour_palette.push(Vec4::new(0.0, 0.0, 0.0, 1.0));
        let start = Vec4::new(1.0, 0.4, 0.0, 1.0);
        let end = Vec4::new(0.9, 0.7, 0.0, 1.0);
        for i in 0..MAX_ITERATIONS/2 {
            colour_palette.push(start.lerp(end, i as f32/(MAX_ITERATIONS/2) as f32));
        };

        let start = Vec4::new(0.9, 0.7, 0.0, 1.0);
        let end = Vec4::new(0.2, 0.7, 0.5, 1.0);
        for i in 0..MAX_ITERATIONS/2 {
            colour_palette.push(start.lerp(end, i as f32/(MAX_ITERATIONS/2) as f32));
        };

        let mut x = Voronoinoi {
            w,
            h,
            r: Rect::new(-2.0, -1.5, 3.0, 3.0),
        };
        x
    }
}

impl DoFrame for Voronoinoi {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    

        
        // axes
        let xstart = Vec2::new(-2.0, 0.0).transform(self.r, inputs.screen_rect);
        let xend = Vec2::new(1.0, 0.0).transform(self.r, inputs.screen_rect);
        outputs.canvas.put_line(xstart, xend, 0.001, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
        let ystart = Vec2::new(0.0, -1.0).transform(self.r, inputs.screen_rect);
        let yend = Vec2::new(0.0, 1.0).transform(self.r, inputs.screen_rect);
        outputs.canvas.put_line(ystart, yend, 0.001, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
 
        outputs.texture_rect = Some(inputs.screen_rect);
    }
}