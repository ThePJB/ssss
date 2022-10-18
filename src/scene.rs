use crate::renderers::font_rendering::*;
use crate::renderers::simple_renderer::*;
use crate::kinput::*;
use crate::texture_buffer::*;
use crate::kmath::*;

pub fn init_demo<T: Demo + Default + 'static>() -> Box<dyn Demo> {
    Box::new(T::default())
}

pub struct FrameOutputs {
    pub canvas: SimpleCanvas,
    pub set_texture: Vec<(TextureBuffer, usize)>,
    pub draw_texture: Vec<(Rect, usize)>,
    pub glyphs: GlyphBuffer,
}

impl FrameOutputs {
    pub fn new(a: f64) -> FrameOutputs {
        FrameOutputs {
            glyphs: GlyphBuffer::new(),
            canvas: SimpleCanvas::new(a),
            set_texture: Vec::new(),
            draw_texture: Vec::new(),
        }
    }
}

pub trait Demo {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs);
}