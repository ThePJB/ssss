use crate::font_rendering::*;
use crate::simple_renderer::*;
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

pub struct Scene {
    pub name: String,
    pub logic: Box<dyn DoFrame>,
}

// yea traits are a bit dogshit
// scene initialization: pass in a SceneInitState

// I need to do some basic ass shit: a table of function pointers
// damn I want a GC lol.

// so you could use function pointers but you miss out on the T for retained state

// let me clone stuff!!!!
// the point is having an arbitrary sized type