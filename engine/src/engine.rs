use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use std::time::Instant;

use crate::{
    context::Context,
    gamestate::{Action, GameState, GameStateMachine},
    picking::Picker,
    resource::{ShaderId, TextureId},
    scene::Scene,
    shader::Shader,
    texture::Texture,
    ui,
};

pub struct Engine {
    pub context: Context,
    pub game_state_machine: GameStateMachine,
    pub scene: Scene,
    pub picker: Picker,
    // pub ui: ui::Ui,
}

impl Engine {
    pub fn new(ctx: Context) -> Result<Engine, Box<dyn Error>> {
        let picker = Picker::new(&ctx.backend.display);
        Ok(Engine {
            game_state_machine: GameStateMachine::new(),
            scene: Scene::new(),
            context: ctx,
            picker,
            // ui,
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn GameState>) {
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

        self.game_state_machine
            .push(&mut self.context, &mut self.scene, initial_state);

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
                    self.game_state_machine
                        .push(&self.context, &mut self.scene, gamestate);
                }
                Action::Switch(gamestate) => {
                    self.game_state_machine.pop(&self.context);
                    self.game_state_machine
                        .push(&self.context, &mut self.scene, gamestate);
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
        let mut action = match self.game_state_machine.current() {
            Some(gamestate) => gamestate.frame(&self.context, &mut self.scene),
            None => Action::Quit,
        };

        let _picked_object = self.picker.get_picked_object();

        self.scene.render(&self.context);

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
                        self.scene.camera.scale((height / width) as f32);
                    }
                    glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                        if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                            self.scene.camera.zoom(*y);
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
