use crate::{
    resource::{ResourcePack, ShaderId, TextureId, ModelId},
    settings::Settings,
    shader::Shader,
    texture::Texture,
    model::Model,
    clock::Clock,
    picking::Picker,
};
use log::info;
use glium::{
    glutin::{dpi::LogicalSize, ContextBuilder, Event, WindowBuilder},
    Display,
};
use std::boxed::Box;
use std::error::Error;
use std::f64;
use std::path::Path;
use std::result::Result;
use glium_glyph::GlyphBrush;
use glium_glyph::glyph_brush::{
	rusttype::Font,
};

const TEXTURES_DIRECTORY: &str = "res/textures";
const SHADERS_DIRECTORY: &str = "res/shaders";
const MODELS_DIRECTORY: &str = "res/models";

pub struct Mouse {
    pub position: Option<(i32, i32)>,
}

pub struct Window {
    pub width: u32,
    pub height: u32,
}

pub struct GlBackend<'a> {
    pub event_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub glyph_brush: GlyphBrush<'a, 'a>,
}

pub struct Context<'a> {
    pub backend: GlBackend<'a>,
    pub settings: Settings,
    pub resource_pack: ResourcePack,
    pub mouse: Mouse,
    pub window: Window,
    pub clock: Clock,
    pub picker: Picker,
}

impl<'a> GlBackend<'a> {
    fn new(window_width: u32, window_height: u32) -> Result<GlBackend<'a>, Box<dyn Error>> {
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

        let consolas: &[u8] = include_bytes!("../res/fonts/Consolas-Regular.ttf");
        let fonts = vec![Font::from_bytes(consolas).unwrap()];

        let glyph_brush = GlyphBrush::new(&display, fonts);
        info!(
            "GL Version {}",
            display.get_opengl_version_string()
        );
        info!(
            "GL Implementation {}",
            display.get_opengl_vendor_string()
        );
        info!(
            "GL Renderer {}",
            display.get_opengl_renderer_string()
        );
        Ok(GlBackend {
            event_loop,
            display,
            glyph_brush,
        })
    }
}

impl<'a> Context<'a> {
    pub fn new(settings: Settings, resource_pack: ResourcePack) -> Context<'a> {
        let backend = GlBackend::new(
            settings.graphics.window_width,
            settings.graphics.window_height,
        )
        .unwrap();
        let window = Window {
            width: settings.graphics.window_width,
            height: settings.graphics.window_height,
        };
        let picker = Picker::new(&backend.display);
        Context {
            backend,
            settings,
            resource_pack,
            window,
            mouse: Mouse { position: None },
            clock: Clock::new(),
            picker,
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
                Path::new(TEXTURES_DIRECTORY)
                    .join(filename)
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );
    }

    pub fn load_model(&mut self, name: &'static str, filename: &str) {
        self.resource_pack.register_model(
            ModelId(name),
            Model::new(
                Path::new(MODELS_DIRECTORY)
                    .join(filename)
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );
    }

    pub fn load_shader(&mut self, name: &'static str, vertex_shader: &str, fragment_shader: &str) {
        self.resource_pack.register_shader(
            ShaderId(name),
            Shader::new(
                &self.backend,
                Path::new(SHADERS_DIRECTORY)
                    .join(vertex_shader)
                    .to_str()
                    .unwrap()
                    .to_string(),
                Path::new(SHADERS_DIRECTORY)
                    .join(fragment_shader)
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );
    }
}
