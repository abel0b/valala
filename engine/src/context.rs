use crate::{
    resource::{ResourcePack, ShaderId, TextureId},
    settings::Settings,
    shader::Shader,
    texture::Texture,
};
use glium::{
    glutin::{dpi::LogicalSize, ContextBuilder, Event, WindowBuilder},
    Display,
};
use std::boxed::Box;
use std::error::Error;
use std::f64;
use std::result::Result;

const TEXTURES_DIRECTORY: &str = "res/textures";
const SHADERS_DIRECTORY: &str = "res/shaders";
// const MESHES_DIRECTORY: &str = "res/meshes";

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
        let wb = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(
                f64::from(window_width),
                f64::from(window_height),
            ))
            .with_title("Valala");
        let cb = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(4);
        let display = Display::new(wb, cb, &event_loop)?;
        Ok(GlBackend {
            event_loop,
            display,
        })
    }
}

impl Context {
    pub fn new(settings: Settings, resource_pack: ResourcePack) -> Context {
        let backend = GlBackend::new(
            settings.graphics.window_width,
            settings.graphics.window_height,
        )
        .unwrap();
        Context {
            backend,
            settings,
            resource_pack,
            mouse: Mouse { position: None },
        }
    }

    pub fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.backend.event_loop.poll_events(|e| {
            events.push(e);
        });
        events
    }

    pub fn load_texture(&mut self, name: &'static str, filename: &str) {
        self.resource_pack.register_texture(
            TextureId(name),
            Texture::new(
                &self.backend,
                format!("{}/{}", TEXTURES_DIRECTORY, filename),
            ),
        );
    }

    pub fn load_shader(&mut self, name: &'static str, filename: &str) {
        self.resource_pack.register_shader(
            ShaderId(name),
            Shader::new(&self.backend, format!("{}/{}", SHADERS_DIRECTORY, filename)),
        );
    }
}
