use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

use num::complex::Complex;

pub struct Mandelbrot {
    w: usize,
    h: usize,
    r: Rect,
    buf: Vec<i32>,
    colour_palette: Vec<Vec4>,

    stale: bool,
}

const MAX_ITERATIONS: i32 = 160;

impl Mandelbrot {
    pub fn new(w: usize, h: usize) -> Mandelbrot {
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

        let mut x = Mandelbrot {
            w,
            h,
            r: Rect::new(-2.0, -2.0, 4.0, 4.0),
            stale: true,
            buf: Vec::new(),
            colour_palette: colour_palette,
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

                let x0 = self.r.left() + (i as f32 + 0.5) * self.r.w / self.w as f32;
                let y0 = -self.r.bot() + (j as f32 + 0.5) * self.r.h / self.h as f32;


                let mut z = Complex::new(0.0, 0.0);
                let c = Complex::new(x0, y0);
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

impl DoFrame for Mandelbrot {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    
        if inputs.key_falling(VirtualKeyCode::R) {
            *self = Mandelbrot::new(self.w, self.h);
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

        if self.stale {
            self.compute();
            let tw = self.w;
            let th = self.h;
            let mut t = TextureBuffer::new(tw, th);
            for i in 0..tw {
                for j in 1..th {
                    let it = self.buf[i * self.h + j];

                    let colour = self.colour_palette[(MAX_ITERATIONS - it) as usize];

                    t.set(i as i32, j as i32, colour);
                }
            }
            outputs.texture = Some(t);

            self.stale = false;
        }
 
        outputs.texture_rect = Some(inputs.screen_rect);
    }
}