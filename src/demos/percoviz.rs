use crate::scene::*;
use crate::widgets::*;
use crate::kmath::*;
use crate::texture_buffer::*;
use crate::kinput::*;
use glutin::event::VirtualKeyCode;

pub struct Percoviz {
    w: usize,
    h: usize,

    seed: u32,

    components: Vec<i32>,
    edges_above: Vec<f64>,
    edges_left: Vec<f64>,

    edge_chance: f64,
    edge_chance_slider: FloatSlider,
    edge_chance_fine_slider: FloatSlider,

    stale: bool,
}

impl Default for Percoviz {
    fn default() -> Percoviz {
        Percoviz::new(400, 400)
    }
}

impl Percoviz {
    pub fn new(w: usize, h: usize) -> Percoviz {
        let mut x = Percoviz {
            w,
            h,
            seed: 0,
            components: Vec::new(),
            edges_above: Vec::new(),
            edges_left: Vec::new(),
            edge_chance: 0.5,
            edge_chance_slider: FloatSlider::new(0.5, 0.0, 1.0, "coarse".to_string()),
            edge_chance_fine_slider: FloatSlider::new(0.0, -0.01, 0.01, "fine".to_string(),),
            stale: true,
        };
        x.run_perc();
        x
    }

    pub fn run_perc(&mut self) {
        let tstart = std::time::SystemTime::now();

        // initialize edges
        self.edges_above = vec![0.0; (self.w) * (self.h)];
        self.edges_left = vec![0.0; (self.w) * (self.h)];
        for i in 0..self.w {
            for j in 0..self.h {
                let cell_seed = self.seed + i as u32 * 213129087 + j as u32 * 1239141567;
                self.edges_above[i * (self.h) + j] = kuniform(cell_seed, 0.0, 1.0);
                self.edges_left[i * (self.h) + j] = kuniform(cell_seed * 239487237, 0.0, 1.0);
            }
        }

        self.components = vec![0; self.w*self.h];
        for i in 0..self.w {
            for j in 0..self.h {
                self.components[i * self.h + j] = (i * self.h + j) as i32;
            }
        }

        let mut ff_stack = Vec::new();
        
        for i in 0..self.w {
            for j in 0..self.h {
                // cc starting at i,j
                let start_idx = i * self.h + j;
                let start_team = start_idx as i32;
                if self.components[start_idx] >= start_team {
                    ff_stack.push(start_idx);
                    while let Some(idx) = ff_stack.pop() {
                        for n in 0..4 {
                            let n_idx;
                            let edge;
                            if n == 0 {
                                if idx % self.w == 0 {
                                    continue;
                                }
                                n_idx = idx - 1;
                                edge = self.edges_left[idx];
                            } else if n == 1 {
                                if idx < self.w {
                                    continue;
                                }
                                n_idx = idx - self.w;
                                edge = self.edges_above[idx];
                            } else if n == 2 {
                                if idx % self.w == self.w - 1 {
                                    continue;
                                }
                                n_idx = idx + 1;
                                edge = self.edges_left[idx + 1];
                            } else {
                                if idx >= (self.w * self.h) - self.w {
                                    continue;
                                }
                                n_idx = idx + self.w;
                                edge = self.edges_above[idx + 1];
                            }
                            
                            if edge < self.edge_chance && self.components[n_idx] > self.components[idx] {
                                self.components[n_idx] = self.components[idx];
                                ff_stack.push(n_idx);
                            }
                        }
                    }
                }
            }
        }

        println!("run perc took {:?}", tstart.elapsed().unwrap());
    }
}

impl Demo for Percoviz {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        let mut change = self.edge_chance_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(8, 0, 10, 3));
        change |= self.edge_chance_fine_slider.frame(inputs, outputs, inputs.screen_rect.grid_child(9, 0, 10, 3));
        if change {
            self.edge_chance = self.edge_chance_slider.curr() + self.edge_chance_fine_slider.curr();
            self.stale = true;
        }

        if inputs.key_rising(VirtualKeyCode::R) {
            self.seed += 1;
            self.stale = true;
        }
        
        if self.stale {
            self.run_perc();
            let tw = self.w;
            let th = self.h;
            let mut t = TextureBuffer::new(tw, th);
            for i in 0..tw {
                for j in 0..th {
                    let cc = self.components[i * self.h + j];
                    let colour = Vec4::new(cc as f64 * 137.5, 1.0, 1.0, 1.0).hsv_to_rgb();
                    t.set(i as i32, j as i32, colour);
                }
            }
            outputs.set_texture.push((t, 0));

            self.stale = false;
        }
 
        outputs.draw_texture.push((inputs.screen_rect, 0));
    }
}

#[test]
fn test_perco() {
    use crate::kimg::*;

    let w = 800;
    let h = 800;

    let mut img = ImageBuffer::new(w, h);
    let mut perc = Percoviz::new(w, h);
    perc.run_perc();
    for i in 0..w {
        for j in 0..h {
            let cc = perc.components[i * h + j];
            // colour = ?
            let colours = vec![
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 1.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 1.0, 1.0),
                Vec4::new(1.0, 1.0, 0.0, 1.0),
            ];
            let colour = colours[cc as usize % colours.len()];
            
            img.set_px(i, j, ((colour.x * 255.) as u8, (colour.y * 255.) as u8, (colour.z * 255.) as u8));
        }
    }
    img.dump_to_file("perc.png");
}