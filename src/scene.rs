use crate::renderers::font_rendering::*;
use crate::renderers::mesh_renderer::MeshBuilder;
use crate::renderers::simple_renderer::*;
use crate::texture_buffer::*;
use crate::kmath::*;
use crate::audio::*;
use crate::kinput::*;

pub fn init_demo<T: Demo + Default + 'static>() -> Box<dyn Demo> {
    Box::new(T::default())
}

pub struct FrameOutputs {
    pub canvas: SimpleCanvas,
    pub set_texture: Vec<(TextureBuffer, usize)>,
    pub draw_texture: Vec<(Rect, usize)>,
    pub glyphs: GlyphBuffer,

    pub set_mesh: Option<MeshBuilder>,
    pub set_mesh_texture: Option<TextureBuffer>,
    pub draw_mesh: Option<([f32;16], [f32;16], Vec3, Vec3)>,

    pub audio_events: Vec<u32>,
}

impl FrameOutputs {
    pub fn new(a: f32) -> FrameOutputs {
        FrameOutputs {
            glyphs: GlyphBuffer::new(),
            canvas: SimpleCanvas::new(a),
            set_texture: Vec::new(),
            draw_texture: Vec::new(),
            audio_events: Vec::new(),
            set_mesh: None,
            set_mesh_texture: None,
            draw_mesh: None,
        }
    }
}

pub trait Demo {
    fn frame(&mut self, inputs: &FrameInputs, outputs: &mut FrameOutputs);
}