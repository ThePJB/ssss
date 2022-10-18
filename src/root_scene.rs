use glutin::event::VirtualKeyCode;
use crate::scene::*;
use crate::kmath::*;

use crate::demos::bsj::*;
use crate::demos::mdj::*;
use crate::demos::burning_ship::*;
use crate::demos::percoviz::*;
use crate::demos::mandelbrot::*;
use crate::demos::julia::*;
use crate::demos::rgbwalk::*;
use crate::demos::rgbutm::*;
use crate::demos::noise_test::*;
use crate::demos::voronoinoi::*;

pub struct RootScene {
    curr_scene: Option<Box<dyn Demo>>,
    demo_table: Vec<(&'static str, fn() -> Box<dyn Demo>)>,
    idx: usize,
    show: bool,
}

impl RootScene {
    pub fn new() -> RootScene {
        let mut demo_table: Vec<(&str, fn() -> Box<dyn Demo>)> = Vec::new();
        
        // Fractals
        demo_table.push(("Mandel", init_demo::<Mandelbrot>));
        demo_table.push(("Julia", init_demo::<Julia>));
        demo_table.push(("Mandel-Julia", init_demo::<MDJ>));
        demo_table.push(("Burning", init_demo::<BurningShip>));
        demo_table.push(("Burn-Julia", init_demo::<BSJ>));
        
        // Random walk
        demo_table.push(("RGBWalk", init_demo::<RGBWalk>));
        demo_table.push(("RGBUTM", init_demo::<RGBUTM>));

        // Noise
        demo_table.push(("Recnoise", init_demo::<NoiseTest>));

        // Others
        demo_table.push(("Percolation", init_demo::<Percoviz>));
        demo_table.push(("Delauney", init_demo::<Voronoinoi>));

        RootScene {
            curr_scene: None,
            demo_table,
            idx: 0,
            show: false,
        }
    }
}

impl Demo for RootScene {
    fn frame(&mut self, inputs: &crate::kinput::FrameInputState, outputs: &mut FrameOutputs) {
        if let Some(curr) = self.curr_scene.as_mut() {
            curr.frame(inputs, outputs);
        } else {
            self.show = true;
        }

        if inputs.key_rising(VirtualKeyCode::Space) {
            self.show = !self.show;
        }

        if inputs.key_rising(VirtualKeyCode::Return) {
            self.curr_scene = Some(self.demo_table[self.idx].1());
        }

        if inputs.key_rising(VirtualKeyCode::K) {
            if self.idx > 0 {
                self.idx = self.idx - 1;
            }
        }
        if inputs.key_rising(VirtualKeyCode::J) {
            if self.idx < self.demo_table.len() - 1 {
                self.idx = self.idx + 1;
            }
        }

        if self.show {
            outputs.canvas.put_rect(inputs.screen_rect.child(0.0, 0.0, 0.15, 1.0), 5.0, Vec4::new(0.5, 0.5, 0.5, 0.5));
            for i in 0..self.demo_table.len() {
                let x = 0.005;
                let y_initial = 0.005;
                let glyph_h = inputs.screen_rect.h * 0.02;
                let glyph_w = glyph_h * (12.0/14.0);
                let colour = if i == self.idx {
                    Vec4::new(1.0, 1.0, 0.0, 1.0)
                } else {
                    Vec4::new(1.0, 1.0, 1.0, 1.0)
                };
                outputs.glyphs.push_str(self.demo_table[i].0, x, y_initial + i as f64 * glyph_h, glyph_w, glyph_h * 0.8, 5.5, colour);
            }
        }
    }
}