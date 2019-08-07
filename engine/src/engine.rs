use glium_glyph::glyph_brush::{rusttype::Scale, Section};
use std::boxed::Box;
use std::error::Error;
use std::result::Result;

use crate::{
    log::LOGGER,
    resource::{ShaderId, TextureId},
    shader::Shader,
    stage::{Stage, StageMachine, Transition},
    store::Store,
    texture::Texture,
};

pub struct Engine<'a, S, A> {
    pub stage_machine: StageMachine<S, A>,
    pub store: Store<'a, S, A>,
}

impl<'a, S, A> Engine<'a, S, A> {
    pub fn new(store: Store<'a, S, A>) -> Result<Engine<'a, S, A>, Box<dyn Error>> {
        Ok(Engine {
            stage_machine: StageMachine::default(),
            store,
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn Stage<S, A>>) {
        self.store.context.resource_pack.register_shader(
            ShaderId("default"),
            Shader::from_source(
                &self.store.context.backend,
                include_str!("../res/shaders/default.vert"),
                include_str!("../res/shaders/default.frag"),
            ),
        );

        self.store.context.resource_pack.register_shader(
            ShaderId("color"),
            Shader::from_source(
                &self.store.context.backend,
                include_str!("../res/shaders/color.vert"),
                include_str!("../res/shaders/color.frag"),
            ),
        );

        self.store.context.resource_pack.register_shader(
            ShaderId("model"),
            Shader::from_source(
                &self.store.context.backend,
                include_str!("../res/shaders/model.vert"),
                include_str!("../res/shaders/model.frag"),
            ),
        );

        self.store.context.resource_pack.register_shader(
            ShaderId("picking"),
            Shader::from_source(
                &self.store.context.backend,
                include_str!("../res/shaders/picking.vert"),
                include_str!("../res/shaders/picking.frag"),
            ),
        );

        self.store.context.resource_pack.register_texture(
            TextureId("default"),
            Texture::from_raw(
                &self.store.context.backend,
                include_bytes!("../res/textures/default.png"),
            ),
        );

        self.stage_machine.push(&mut self.store, initial_state);

        self.store.context.picker.initialize_picking_attachments(
            &self.store.context.backend.display,
            (
                self.store.context.settings.graphics.window_width,
                self.store.context.settings.graphics.window_height,
            ),
        );

        loop {
            match self.update() {
                Transition::Continue => {
                    self.store.context.clock.tick();
                    self.store.context.backend.glyph_brush.queue(Section {
                        text: &format!("{} fps", self.store.context.clock.fps)[..],
                        screen_position: (8.0, 8.0),
                        bounds: (
                            self.store.context.window.width as f32,
                            self.store.context.window.height as f32 / 2.0,
                        ),
                        scale: Scale::uniform(24.0),
                        color: [1.0, 0.84, 0.27, 1.0],
                        ..Section::default()
                    });
                }
                Transition::Push(stage) => {
                    self.stage_machine.push(&mut self.store, stage);
                }
                Transition::Switch(stage) => {
                    self.stage_machine.pop(&mut self.store);
                    self.stage_machine.push(&mut self.store, stage);
                }
                Transition::Pop => {
                    self.stage_machine.pop(&mut self.store);
                }
                Transition::Quit => {
                    break;
                }
            }
        }
    }

    fn update(&mut self) -> Transition<S, A> {
        let mut action = self.stage_machine.update(&mut self.store);
        self.stage_machine.render(&mut self.store);

        for e in self.store.context.events().iter() {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => action = Transition::Quit,
                    glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize {
                        width,
                        height,
                    }) => {
                        self.store.context.picker.initialize_picking_attachments(
                            &self.store.context.backend.display,
                            (*width as u32, *height as u32),
                        );
                        // self.stage_machine
                        //     .scene()
                        //     .camera
                        //     .scale((height / width) as f32);
                    }
                    // glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                    //     if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                    //         self.stage_machine.scene().camera.zoom(*y);
                    //     }
                    // },
                    glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                        self.store.context.mouse.position =
                            Some((position.x as i32, position.y as i32));
                    }
                    _ => (),
                },
                glium::glutin::Event::DeviceEvent { .. } => (),
                _ => (),
            }
        }

        action
    }
}

pub fn initialize() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
