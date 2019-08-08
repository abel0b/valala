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
    store::World,
    texture::Texture,
};

pub struct Engine<'a, W: World> {
    pub stage_machine: StageMachine<W>,
    pub store: Store<'a, W>,
}

impl<'a, W> Engine<'a, W>
where
    W: World,
{
    pub fn new(store: Store<'a, W>) -> Result<Engine<'a, W>, Box<dyn Error>> {
        Ok(Engine {
            stage_machine: StageMachine::default(),
            store,
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn Stage<W>>) {
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

    fn update(&mut self) -> Transition<W> {
        let mut action = self.stage_machine.update(&mut self.store);
        self.stage_machine.render(&mut self.store);

        if action.is_continue() {
            for e in self.store.context.events().iter() {
                action = self.stage_machine.handle(&mut self.store, e)
            }
        }

        action
    }
}

pub fn initialize() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
