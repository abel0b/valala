use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use std::time::Instant;
use glium_glyph::glyph_brush::{
	rusttype::Scale, Section,
};

use crate::{
    log::LOGGER,
    context::Context,
    stage::{Transition, Stage, StageMachine},
    picking::Picker,
    resource::{ShaderId, TextureId},
    shader::Shader,
    texture::Texture,
    ui::Ui,
	store::Store,
};

pub struct Engine<'a,S,A> {
    pub context: Context<'a>,
    pub stage_machine: StageMachine<S,A>,
    pub picker: Picker,
    pub ui: Ui,
	pub store: Store<S, A>,
}

impl<'a,S,A> Engine<'a,S,A> {
    pub fn new(ctx: Context<'a>, store: Store<S, A>) -> Result<Engine<'a, S, A>, Box<dyn Error>> {
        let picker = Picker::new(&ctx.backend.display);
        Ok(Engine {
            stage_machine: StageMachine::default(),
            context: ctx,
            picker,
            ui: Ui::default(),
			store,
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn Stage<S,A>>) {
        self.context.resource_pack.register_shader(
            ShaderId("default"),
            Shader::from_source(
                &self.context.backend,
                include_str!("../res/shaders/default.vert"),
                include_str!("../res/shaders/default.frag"),
            ),
        );

        self.context.resource_pack.register_shader(
            ShaderId("color"),
            Shader::from_source(
                &self.context.backend,
                include_str!("../res/shaders/color.vert"),
                include_str!("../res/shaders/color.frag"),
            ),
        );

        self.context.resource_pack.register_shader(
            ShaderId("model"),
            Shader::from_source(
                &self.context.backend,
                include_str!("../res/shaders/model.vert"),
                include_str!("../res/shaders/model.frag"),
            ),
        );

        self.context.resource_pack.register_texture(
            TextureId("default"),
            Texture::from_raw(
                &self.context.backend,
                include_bytes!("../res/textures/default.png"),
            ),
        );

        let mut previous_clock = Instant::now();

        self.stage_machine.push(&self.context, initial_state);

        self.picker.initialize_picking_attachments(
            &self.context.backend.display,
            (
                self.context.settings.graphics.window_width,
                self.context.settings.graphics.window_height,
            ),
        );

        loop {
            match self.update() {
                Transition::Continue => {
                    let now = Instant::now();
                    let fps = 1_000_000_000 / (now - previous_clock).as_nanos();

                    self.context.backend.glyph_brush.queue(Section {
                        text : &format!("{} fps", fps)[..],
                        screen_position: (8.0, 8.0),
                        bounds : (self.context.window.width as f32, self.context.window.height as f32 / 2.0),
                        scale: Scale::uniform(24.0),
                        color: [1.0, 0.84, 0.27, 1.0],
                        ..Section::default()
                    });
                    previous_clock = now;
                }
                Transition::Push(stage) => {
                    self.stage_machine.push(&self.context, stage);
                }
                Transition::Switch(stage) => {
                    self.stage_machine.pop(&self.context);
                    self.stage_machine.push(&self.context, stage);
                }
                Transition::Pop => {
                    self.stage_machine.pop(&self.context);
                }
                Transition::Quit => {
                    break;
                }
            }
        }
    }

    fn update(&mut self) -> Transition<S,A> {
        let mut action = self.stage_machine.update(&self.context, &mut self.store);

        let _picked_object = self.picker.get_picked_object();

        self.stage_machine.render(&mut self.context);

        self.picker.commit(self.context.mouse.position);

        for e in self.context.events().iter() {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => action = Transition::Quit,
                    glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize {
                        width,
                        height,
                    }) => {
                        self.picker.initialize_picking_attachments(
                            &self.context.backend.display,
                            (*width as u32, *height as u32),
                        );
                        // self.stage_machine
                        //     .scene()
                        //     .camera
                        //     .scale((height / width) as f32);
                    },
                    // glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                    //     if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                    //         self.stage_machine.scene().camera.zoom(*y);
                    //     }
                    // },
                    glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                        self.context.mouse.position = Some((position.x as i32, position.y as i32));
                    },
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
