use crate::demos::bsj::BSJ;
use crate::demos::mdj::MDJ;
use crate::demos::burning_ship::BurningShip;
use crate::demos::frac_test::FracTest;
use crate::demos::pp_fert::PredatorPreyFert;
use crate::scene::*;
use crate::demos::predator_prey::*;
use crate::demos::percoviz::*;
use crate::demos::mandelbrot::*;
use crate::demos::julia::*;
use crate::demos::rgbwalk::*;
use crate::demos::rgbutm::*;
use crate::demos::lvlwalk::*;
use crate::demos::noise_test::*;
use crate::demos::voronoinoi::*;
use crate::kmath::*;
use glutin::event::VirtualKeyCode;
// scene struct or 
// best way to organize? probably dont want an instance of each one. ideally a fn ptr but its nice to pass params in
// say you wanted to handle sets of configs
// but i dont want boilerplate making a default() for everything
// maybe no boilerplate because it takes a lamda to make it
// could they not all be different and then have a switch statement mapping a number
// also needs somewhere to store it while its running
// maybe this is fine, though they all smash the gpu at once

// i spose now i gotta add the menu
// shit the first one is queening a chessboard



pub struct RootScene {
    curr_scene: Option<Box<dyn DoFrame>>,
    names: Vec<String>,
    idx: usize,
    show: bool,
}

impl RootScene {
    pub fn new() -> RootScene {
        RootScene {
            curr_scene: None,
            names: vec![
                "Percolation".to_owned(),
                "Predator Prey".to_owned(),
                "Mandelbrot".to_owned(),
                "Julia".to_owned(),
                "Burning Ship".to_owned(),
                "Predator Prey Fert".to_owned(),
                "RGBWalk".to_owned(),
                "RGBUTM".to_owned(),
                "lvlwalk".to_owned(),
                "frac test".to_owned(),
                "noise test".to_owned(),
                "voronoinoi".to_owned(),
                "bsj".to_owned(),
                "mdj".to_owned(),
            ],
            idx: 0,
            show: false,
        }
    }

    pub fn switch(&mut self) {
        self.curr_scene = Some(match self.idx {
            0 => Box::new(Percoviz::new(400, 400)),
            1 => Box::new(PredatorPrey::new(400,400)),
            2 => Box::new(Mandelbrot::new(800, 800)),
            3 => Box::new(Julia::new(400, 400)),
            4 => Box::new(BurningShip::new(800, 800)),
            5 => Box::new(PredatorPreyFert::new(400, 400)),
            6 => Box::new(RGBWalk::new(151, 151, 0)),
            7 => Box::new(RGBUTM::new(200, 200, 0)),
            8 => Box::new(LevelWalk::new(800, 800)),
            9 => Box::new(FracTest::new(800, 800, 0)),
            10 => Box::new(NoiseTest::new(800, 800)),
            11 => Box::new(Voronoinoi::new()),
            12 => Box::new(BSJ::new(800, 800)),
            13 => Box::new(MDJ::new(800, 800)),
            _ => panic!("out of range")
        })
    }
}

impl DoFrame for RootScene {
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
            self.switch();
        }

        if inputs.key_rising(VirtualKeyCode::K) {
            if self.idx > 0 {
                self.idx = self.idx - 1;
            }
        }
        if inputs.key_rising(VirtualKeyCode::J) {
            if self.idx < self.names.len() - 1 {
                self.idx = self.idx + 1;
            }
        }

        if self.show {
            outputs.canvas.put_rect(inputs.screen_rect.child(0.0, 0.0, 0.15, 1.0), 5.0, Vec4::new(0.5, 0.5, 0.5, 0.5));
            for i in 0..self.names.len() {
                let x = 0.0;
                let y_initial = 0.0;
                let glyph_h = inputs.screen_rect.h * 0.03;
                let glyph_w = glyph_h/3.0;
                let colour = if i == self.idx {
                    Vec4::new(1.0, 1.0, 0.0, 1.0)
                } else {
                    Vec4::new(1.0, 1.0, 1.0, 1.0)
                };
                outputs.glyphs.push_str(&self.names[i], x, y_initial + i as f32 * glyph_h, glyph_w, glyph_h, 5.5, colour);
            }
        }
    }
}