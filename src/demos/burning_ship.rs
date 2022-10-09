use crate::scene::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

// c such that |(((c^2 + c)^2 + c)^2 + c)^2 + ...| <= 2.0
// so 1 - 1 + 1 - 1 + 1 - 1 for example
// intuition for squaring a complex number?
// recursive definition of the points? eg a point is in the mandelbrot set if the point it goes to is in the mandelbrot set
//  but actually thats wrong because points can be out of the mandelbrot set. maybe if p^2 - c is in the mandelbrot set

pub struct BurningShip {
    w: usize,
    h: usize,
    r: Rect,
    buf: Vec<i32>,
    colour_palette: Vec<Vec4>,

    stale: bool,

    path_c: Option<Vec2>,
}

const MAX_ITERATIONS: i32 = 160;

impl BurningShip {
    pub fn new(w: usize, h: usize) -> BurningShip {
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

        let mut x = BurningShip {
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

                let x0 = self.r.left() + (i as f32 + 0.5) * self.r.w / self.w as f32;
                let y0 = -self.r.bot() + (j as f32 + 0.5) * self.r.h / self.h as f32;
                let y0 = -y0;


                let c = Vec2::new(x0, y0);
                let mut z = Vec2::new(0.0, 0.0);
                while z.x * z.x + z.y * z.y < 4.0 && it < MAX_ITERATIONS {
                    z = Vec2::new(z.x.abs(), z.y.abs());
                    z = z.complex_mul(z) + c;
                    it += 1;
                }

                self.buf[i * self.h + j] = it;
            }
        }

        println!("compute took {:?}", tstart.elapsed().unwrap());
    }
}

impl DoFrame for BurningShip {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {    
        if inputs.key_falling(VirtualKeyCode::R) {
            *self = BurningShip::new(self.w, self.h);
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
            self.path_c = Some(inputs.mouse_pos.transform(inputs.screen_rect, self.r));
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

        if let Some(path_c) = self.path_c {
            let mut zold = Vec2::new(0.0, 0.0);
            for i in 0..100 {
                let znew = Vec2::new(zold.x.abs(), zold.y.abs());
                let znew = znew.complex_mul(znew) + path_c;
    
                // line start and end - transform to where canvas is. from self.r to screen rect
                let start = zold.transform(self.r, inputs.screen_rect);
                let end = znew.transform(self.r, inputs.screen_rect);
    
    
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
 
        outputs.texture_rect = Some(inputs.screen_rect);
    }
}