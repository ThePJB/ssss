use crate::kmath::*;
use crate::renderers::ct_renderer::*;

pub fn glyph_buffer_to_canvas(buf: &GlyphBuffer, a: f32) -> CTCanvas {
    let clip_fn = |c: u8| {
        let idx = c - ' ' as u8;
        let w = '@' as u8 - ' ' as u8;
        let x = idx % w;
        let y = idx / w;
        Rect::new(0.0, 0.0, 1.0, 1.0).grid_child(x as i32, y as i32, w as i32, 3)
    };

    let mut c = CTCanvas::new(a);
    for g in &buf.buf {
        c.put_rect(g.1, clip_fn(g.0 as u8), g.2, g.3);
    }
    c
}

pub struct GlyphBuffer {
    pub buf: Vec<(char, Rect, f32, Vec4)>,
}

impl GlyphBuffer {
    pub fn new() -> GlyphBuffer {
        GlyphBuffer { buf: Vec::new() }
    }

    pub fn push_glyph(&mut self, c: char, r: Rect, d: f32, colour: Vec4) {
        self.buf.push((c, r, d, colour));
    }

    pub fn push_str(&mut self, s: &str, mut x: f32, y: f32, w: f32, h: f32, d: f32, colour: Vec4) {
        for c in s.chars() {
            self.push_glyph(c, Rect::new(x, y, w, h), d, colour);
            x += w;
        }
    }
}