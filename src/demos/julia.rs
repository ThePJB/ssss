use std::f64::consts::PI;

use crate::scene::*;
use crate::kmath::*;
use crate::widgets::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

use num::complex::Complex;

pub struct Julia {
    w: usize,
    h: usize,
    r: Rect,
    buf: Vec<i32>,
    colour_palette: Vec<Vec4>,

    a_slider: FloatSlider,
    b_slider: FloatSlider,

    r_slider: FloatSlider,
    theta_slider: FloatSlider,

    stale: bool,
}

const MAX_ITERATIONS: i32 = 400;

impl Default for Julia {
    fn default() -> Julia {
        Julia::new(400, 400)
    }
}

impl Julia {
    pub fn new(w: usize, h: usize) -> Julia {
        let mut colour_palette = Vec::new();

        colour_palette.push(Vec4::new(0.0, 0.0, 0.0, 1.0));
        let start = Vec4::new(1.0, 0.4, 0.0, 1.0);
        let end = Vec4::new(0.9, 0.7, 0.0, 1.0);
        for i in 0..MAX_ITERATIONS/2 {
            colour_palette.push(start.lerp(end, i as f64/(MAX_ITERATIONS/2) as f64));
        };

        let start = Vec4::new(0.9, 0.7, 0.0, 1.0);
        let end = Vec4::new(0.2, 0.7, 0.5, 1.0);
        for i in 0..MAX_ITERATIONS/2 {
            colour_palette.push(start.lerp(end, i as f64/(MAX_ITERATIONS/2) as f64));
        };

        let mut x = Julia {
            w,
            h,
            r: Rect::new(-2.0, -2.0, 4.0, 4.0),
            stale: true,
            buf: Vec::new(),
            colour_palette: colour_palette,
            a_slider: FloatSlider::new(0.25, -(2.0f64.sqrt()), 2.0f64.sqrt(), "re".to_string()),
            b_slider: FloatSlider::new(0.0, -(2.0f64.sqrt()), 2.0f64.sqrt(), "im".to_string(),),
            r_slider: FloatSlider::new(0.25, 0.0, 2.0, "mag".to_string()),
            theta_slider: FloatSlider::new(0.0, -PI, PI, "angle".to_string(),),
        };
        x.compute();
        x
    }

    pub fn compute(&mut self) {
        let tstart = std::time::SystemTime::now();

        self.buf = vec![0; self.w*self.h];

        for i in 0..self.w {
            for j in 0..self.h {
                // convert to float (im) for each pixel coordinate
                let mut it = 0;

                let jre = self.a_slider.curr;
                let jim = self.b_slider.curr;

                let x0 = self.r.left() + (i as f64 + 0.5) * self.r.w / self.w as f64;
                let y0 = -self.r.bot() + (j as f64 + 0.5) * self.r.h / self.h as f64;

                let mut z = Complex::new(x0, y0);
                let c = Complex::new(jre, jim);
                // let c = Complex::new(x0, y0);    // the mandelbrot set is the julia set but c at z = z
                while z.re * z.re + z.im * z.im < 4.0 && it < MAX_ITERATIONS {
                    z = z*z + c;
                    it += 1;
                }

                self.buf[i * self.h + j] = it;
            }
        }

        println!("compute took {:?}", tstart.elapsed().unwrap());
    }
}

impl Demo for Julia {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    
        if inputs.key_falling(VirtualKeyCode::R) {
            *self = Julia::new(self.w, self.h);
        }

        if inputs.lmb == KeyStatus::JustPressed && inputs.key_held(VirtualKeyCode::LShift){
            let rp = inputs.mouse_pos.transform(inputs.screen_rect, self.r);
            self.r = Rect::new_centered(rp.x, rp.y, self.r.w*0.5, self.r.h*0.5);

            self.stale = true;
        } else if inputs.lmb == KeyStatus::JustPressed && inputs.key_held(VirtualKeyCode::LControl){
            let rp = inputs.mouse_pos.transform(inputs.screen_rect, self.r);
            self.r = Rect::new_centered(rp.x, rp.y, self.r.w*2.0, self.r.h*2.0);

            self.stale = true;
        }

        let mut ab_change = self.a_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(8, 0, 10, 3));
        ab_change |= self.b_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(9, 0, 10, 3));
        if ab_change {
            self.stale = true;
            self.r_slider.curr = (self.a_slider.curr*self.a_slider.curr + self.b_slider.curr*self.b_slider.curr).sqrt();
            self.theta_slider.curr = self.b_slider.curr.atan2(self.a_slider.curr);
        }

        let mut rt_change = self.r_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(8, 1, 10, 3));
        rt_change |= self.theta_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(9, 1, 10, 3));
        if rt_change {
            self.stale = true;
            self.a_slider.curr = self.r_slider.curr * self.theta_slider.curr.cos();
            self.b_slider.curr = self.r_slider.curr * self.theta_slider.curr.sin();
        }

        if self.stale {
            self.compute();
            let tw = self.w;
            let th = self.h;
            let mut t = TextureBuffer::new(tw, th);
            for i in 0..tw {
                for j in 0..th {
                    let it = self.buf[i * self.h + j];

                    let colour = self.colour_palette[(MAX_ITERATIONS - it) as usize];

                    t.set(i as i32, j as i32, colour);
                }
            }
            outputs.set_texture.push((t, 0));

            self.stale = false;
        }
 
        outputs.draw_texture.push((inputs.screen_rect, 0));
    }
}