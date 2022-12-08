use crate::kinput::*;
use crate::root_scene::RootScene;
use crate::scene::*;
use crate::video::*;
use crate::audio::*;
use cpal::Stream;
use cpal::traits::*;
use ringbuf::*;
use glow::HasContext;
use glutin::{event_loop::*, event::{Event, WindowEvent}};

pub struct Application {
    video: Video,
    stream: Stream,
    channel: Producer<u32>,

    root_scene: RootScene,
    event_handler: EventAggregator,
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Application {
        let xres = 1600;
        let yres = 1600;
    
        let video = Video::new("ssss", xres as f32, yres as f32, event_loop);
        let rb = RingBuffer::<u32>::new(50);
        let (mut prod, mut cons) = rb.split();
        
        
        let mut app = Application {
            video,
            root_scene: RootScene::new(),
            event_handler: EventAggregator::new(xres as f32, yres as f32),
            stream: stream_setup_for(sample_next, cons).expect("no can make stream"),

            channel: prod,

        };

        app.stream.play().expect("no can play stream");
        app
    }

    pub fn handle_event(&mut self, event: Event<()>) {
        match event {
            Event::LoopDestroyed => self.exit(),
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => self.exit(),
            Event::WindowEvent {event: WindowEvent::Resized(physical_size), .. } => {
                    self.video.window.resize(physical_size);
                    self.video.xres = physical_size.width as f32;
                    self.video.yres = physical_size.height as f32;
                    unsafe {self.video.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32)};
                    self.event_handler.handle_event(&event);
                    // this is expected in the event handler but we need to also handle it here
                    // why does this detach from screen size then
            },
            _ => {
                if let Some(input_state) = self.event_handler.handle_event(&event) {
                    let mut new_outputs = FrameOutputs::new(input_state.screen_rect.aspect());
                    self.root_scene.frame(&input_state, &mut new_outputs);
                    self.video.render(&new_outputs, input_state.screen_rect.aspect());
                    for e in new_outputs.audio_events {
                        self.channel.push(e).ok();
                    }
                }
            }
        }
    }

    pub fn exit(&mut self) {
        println!("exiting");
        std::process::exit(0);
    }
}