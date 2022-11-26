use glow::*;
use crate::renderers::ct_renderer::*;
use crate::renderers::mesh_renderer;
use crate::renderers::mesh_renderer::*;
use crate::renderers::simple_renderer::*;
use crate::renderers::texture_renderer::*;
use crate::renderers::font_rendering::*;
use crate::scene::*;

pub struct Video {
    pub gl: glow::Context,
    pub window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub xres: f64,
    pub yres: f64,

    pub simple_renderer: SimpleRenderer,
    pub texture_renderer: TextureRenderer,
    pub ct_renderer: CTRenderer,
    pub mesh_renderer: MeshRenderer,
}

impl Video {
    pub fn new(title: &str, xres: f64, yres: f64, event_loop: &glutin::event_loop::EventLoop<()>) -> Video {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));

        let window = unsafe {
            glutin::ContextBuilder::new()
                // .with_depth_buffer(0)
                // .with_srgb(true)
                // .with_stencil_buffer(0)
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let gl = unsafe {
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            gl.enable(DEPTH_TEST);
            // gl.enable(CULL_FACE);
            gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            gl.enable(BLEND);
            gl.debug_message_callback(|a, b, c, d, msg| {
                println!("{} {} {} {} msg: {}", a, b, c, d, msg);
            });
            gl
        };

        let simple_renderer = SimpleRenderer::new(&gl);
        let texture_renderer = TextureRenderer::new(&gl);
        let ct_renderer = CTRenderer::new(&gl, "font.png");
        let mesh_renderer = MeshRenderer::new(&gl);

        Video {
            gl,
            window,
            xres,
            yres,
            simple_renderer,
            texture_renderer,
            ct_renderer,
            mesh_renderer,
        }
    }

    pub fn render(&mut self, outputs: &FrameOutputs, a: f64) {
        unsafe {
            if let Some(mb) = &outputs.set_mesh {
                self.mesh_renderer.update_mesh(&self.gl, mb)
            }
            if let Some(tb) = &outputs.set_mesh_texture {
                self.mesh_renderer.update_texture(&self.gl, tb)
            }

            for (buf, idx) in &outputs.set_texture {
                self.texture_renderer.update(&self.gl, buf, *idx);
            }

            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 


            if let Some((pv, mm, cp, cd)) = &outputs.draw_mesh {
                self.mesh_renderer.render(&self.gl, *pv, *mm, *cp, *cd);
            }


            for (r, idx) in &outputs.draw_texture {
                self.texture_renderer.render(&self.gl, *r, a, *idx);
            }

            self.gl.clear(glow::DEPTH_BUFFER_BIT); 
            self.simple_renderer.render(&self.gl, &outputs.canvas);

            let font_ct_canvas = glyph_buffer_to_canvas(&outputs.glyphs, a);
            self.ct_renderer.render(&self.gl, &font_ct_canvas);


            self.window.swap_buffers().unwrap();
        }
    }
}

