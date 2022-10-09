use crate::renderers::font_rendering::*;
use crate::renderers::simple_renderer::*;
use crate::kinput::*;
use crate::texture_buffer::*;
use crate::kmath::*;

pub struct FrameOutputs {
    pub canvas: SimpleCanvas,
    pub texture: Option<TextureBuffer>,
    pub texture_rect: Option<Rect>,
    pub glyphs: GlyphBuffer,
}

impl FrameOutputs {
    pub fn new(a: f32) -> FrameOutputs {
        FrameOutputs {
            glyphs: GlyphBuffer::new(),
            canvas: SimpleCanvas::new(a),
            texture: None,
            texture_rect: None,
        }
    }
}

pub trait DoFrame {
    fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs);
}