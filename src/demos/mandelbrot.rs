use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

use num::complex::Complex;

// c such that |(((c^2 + c)^2 + c)^2 + c)^2 + ...| <= 2.0
// so 1 - 1 + 1 - 1 + 1 - 1 for example
// intuition for squaring a complex number?
// recursive definition of the points? eg a point is in the mandelbrot set if the point it goes to is in the mandelbrot set
//  but actually thats wrong because points can be out of the mandelbrot set. maybe if p^2 - c is in the mandelbrot set

pub struct Mandelbrot {
    w: usize,
    h: usize,
    r: Rect,
    buf: Vec<i32>,
    colour_palette: Vec<Vec4>,

    stale: bool,

    path_c: Option<Complex<f64>>,
}

const MAX_ITERATIONS: i32 = 800;

// have a rule like colour changes every doubling

impl Default for Mandelbrot {
    fn default() -> Mandelbrot {
        Mandelbrot::new(800, 800)
    }
}

impl Mandelbrot {
    pub fn new(w: usize, h: usize) -> Mandelbrot {
        let mut colour_palette = Vec::new();
        let mut period = 16;
        let mut pc = 0;
        let mut i = 0;
        colour_palette.push(Vec4::new(0.0, 0.0, 0.0, 1.0));
        while colour_palette.len() < MAX_ITERATIONS as usize {
            let colour_start = Vec4::new(137.5 * i as f64, 1.0, 1.0, 1.0).hsv_to_rgb();
            let colour_end = Vec4::new(137.5 * (i+1) as f64, 1.0, 1.0, 1.0).hsv_to_rgb();
            colour_palette.push(colour_start.lerp(colour_end, pc as f64 / period as f64));
            pc += 1;
            if pc == period {
                period *= 2;
                pc = 0;
                i += 1;
            }
        }

        // colour_palette.push(Vec4::new(0.0, 0.0, 0.0, 1.0));
        // let start = Vec4::new(1.0, 0.4, 0.0, 1.0);
        // let end = Vec4::new(0.9, 0.7, 0.0, 1.0);
        // for i in 0..MAX_ITERATIONS/2 {
        //     colour_palette.push(start.lerp(end, i as f64/(MAX_ITERATIONS/2) as f64));
        // };

        // let start = Vec4::new(0.9, 0.7, 0.0, 1.0);
        // let end = Vec4::new(0.2, 0.7, 0.5, 1.0);
        // for i in 0..MAX_ITERATIONS/2 {
        //     colour_palette.push(start.lerp(end, i as f64/(MAX_ITERATIONS/2) as f64));
        // };

        let mut x = Mandelbrot {
            w,
            h,
            r: Rect::new(-2.0, -1.5, 3.0, 3.0),
            stale: true,
            buf: Vec::new(),
            colour_palette: colour_palette,
            path_c: None,
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

                let x0 = self.r.left() as f64 + (i as f64 + 0.5) * self.r.w as f64 / self.w as f64;
                let y0 = -self.r.bot() as f64 + (j as f64 + 0.5) * self.r.h as f64 / self.h as f64;


                let c = Complex::new(x0, y0);
                let mut z = Complex::new(0.0, 0.0);
                // let mut z = 2.0c;
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

impl Demo for Mandelbrot {
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
        } else if (inputs.lmb == KeyStatus::Pressed && !inputs.key_held(VirtualKeyCode::LShift) && !inputs.key_held(VirtualKeyCode::LControl)) || inputs.lmb == KeyStatus::JustPressed {
            let v = inputs.mouse_pos.transform(inputs.screen_rect, self.r);
            self.path_c = Some(Complex::new(v.x, v.y));
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
            outputs.set_texture.push((t, 0));

            self.stale = false;
        }

        if let Some(path_c) = self.path_c {
            let mut zold = Complex::new(0.0, 0.0);
            for _ in 0..100 {
                let znew = zold*zold + path_c;
    
                // line start and end - transform to where canvas is. from self.r to screen rect
                let start = Vec2::new(zold.re, zold.im).transform(self.r, inputs.screen_rect);
                let end = Vec2::new(znew.re, znew.im).transform(self.r, inputs.screen_rect);
    
    
                outputs.canvas.put_line(start, end, 0.002, 2.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
                zold = znew;
            }
        }
        
        // axes
        let xstart = Vec2::new(-2.0, 0.0).transform(self.r, inputs.screen_rect);
        let xend = Vec2::new(1.0, 0.0).transform(self.r, inputs.screen_rect);
        outputs.canvas.put_line(xstart, xend, 0.001, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
        let ystart = Vec2::new(0.0, -1.0).transform(self.r, inputs.screen_rect);
        let yend = Vec2::new(0.0, 1.0).transform(self.r, inputs.screen_rect);
        outputs.canvas.put_line(ystart, yend, 0.001, 2.0, Vec4::new(0.8, 0.8, 0.8, 1.0));
 
        outputs.draw_texture.push((inputs.screen_rect, 0));
    }
}