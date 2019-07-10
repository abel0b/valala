use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use glium::{
    Display,
    glutin::{
        WindowBuilder,
        ContextBuilder,
        dpi::LogicalSize,
        Event,
    },
};
use crate::{
    resource::ResourcePack,
    settings::Settings,
};

pub struct Mouse {
    pub position: Option<(i32, i32)>,
}

pub struct GlBackend {
    pub event_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
}

pub struct Context {
    pub backend: GlBackend,
    pub settings: Settings,
    pub resource_pack: ResourcePack,
    pub mouse: Mouse,
}

impl GlBackend {
    fn new(window_width: u32, window_height: u32) -> Result<GlBackend, Box<dyn Error>> {
        let event_loop = glium::glutin::EventsLoop::new();
        let wb = WindowBuilder::new().with_dimensions(LogicalSize::new(window_width as f64, window_height as f64)).with_title("Valala");
        let cb = ContextBuilder::new().with_depth_buffer(24).with_multisampling(4);
        let display = Display::new(wb, cb, &event_loop)?;
        Ok(
            GlBackend {
                event_loop,
                display,
            }
        )
    }
}

impl Context {
    pub fn new(settings: Settings, resource_pack: ResourcePack) -> Context {
            let backend = GlBackend::new(settings.graphics.window_width, settings.graphics.window_height).unwrap();
            Context {
                backend,
                settings,
                resource_pack,
                mouse: Mouse {
                    position: None,
                }
            }
    }

    pub fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.backend.event_loop.poll_events(|e| {
            events.push(e);
        });
        events
    }
}
