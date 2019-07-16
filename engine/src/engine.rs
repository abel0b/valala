use log::info;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::result::Result;
use std::time::Instant;

use crate::{
    context::Context,
    gamestate::{Action, GameState, GameStateMachine},
    picking::Picker,
    resource::{ShaderId, TextureId},
    shader::Shader,
    texture::Texture,
    ui::Ui,
};

pub struct Engine {
    pub context: Context,
    pub game_state_machine: GameStateMachine,
    pub picker: Picker,
    pub ui: Ui,
}

impl Engine {
    pub fn new(ctx: Context) -> Result<Engine, Box<dyn Error>> {
        let picker = Picker::new(&ctx.backend.display);
        Ok(Engine {
            game_state_machine: GameStateMachine::default(),
            context: ctx,
            picker,
            ui: Ui::default(),
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn GameState>) {
        info!(
            "GL Version {}",
            self.context.backend.display.get_opengl_version_string()
        );
        info!(
            "GL Implementation {}",
            self.context.backend.display.get_opengl_vendor_string()
        );
        info!(
            "GL Renderer {}",
            self.context.backend.display.get_opengl_renderer_string()
        );
        info!("Engine started");
        self.context.resource_pack.register_shader(
            ShaderId("default"),
            Shader::from_source(
                &self.context.backend,
                include_str!("../res/shaders/default.vert"),
                include_str!("../res/shaders/default.frag"),
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

        self.game_state_machine.push(&self.context, initial_state);

        self.picker.initialize_picking_attachments(
            &self.context.backend.display,
            (
                self.context.settings.graphics.window_width,
                self.context.settings.graphics.window_height,
            ),
        );

        loop {
            match self.update() {
                Action::Continue => {
                    let now = Instant::now();
                    let _fps = 1_000_000_000 / (now - previous_clock).as_nanos();
                    // println!("{}", _fps);
                    previous_clock = now;
                }
                Action::Push(gamestate) => {
                    self.game_state_machine.push(&self.context, gamestate);
                }
                Action::Switch(gamestate) => {
                    self.game_state_machine.pop(&self.context);
                    self.game_state_machine.push(&self.context, gamestate);
                }
                Action::Pop => {
                    self.game_state_machine.pop(&self.context);
                }
                Action::Quit => {
                    break;
                }
            }
        }
    }

    fn update(&mut self) -> Action {
        let mut action = self.game_state_machine.update(&self.context);

        let _picked_object = self.picker.get_picked_object();

        self.game_state_machine.render(&self.context);

        self.picker.commit(self.context.mouse.position);

        for e in self.context.events().iter() {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => action = Action::Quit,
                    glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize {
                        width,
                        height,
                    }) => {
                        self.picker.initialize_picking_attachments(
                            &self.context.backend.display,
                            (*width as u32, *height as u32),
                        );
                        // self.game_state_machine
                        //     .scene()
                        //     .camera
                        //     .scale((height / width) as f32);
                    }
                    glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                        if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                            // self.game_state_machine.scene().camera.zoom(*y);
                        }
                    }
                    glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                        self.context.mouse.position = Some((position.x as i32, position.y as i32));
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
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
        )
        .unwrap(),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("valala.log").unwrap(),
        ),
    ])
    .unwrap();
}
