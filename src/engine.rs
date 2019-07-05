use glium;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use glium::glutin::dpi::LogicalSize;
use std::time::Instant;
use glium::{uniform,Surface};
use std::collections::HashMap;
use ron;
use std::fs::File;

use crate::gamestate::{GameState, Action};
use crate::lobby::Lobby;
use crate::picking::Picker;
use crate::ui;
use crate::world;
use crate::resource;
use crate::mesh;
use crate::settings::Settings;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);

struct CursorState {
    position: Option<(i32, i32)>,
}

pub struct Engine {
    event_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub picker: Picker,
    pub resource_pack: resource::ResourcePack,
    cursor_state: CursorState,
    pub states: Vec<Box<dyn GameState>>,
    pub world: world::World,
    pub ui: ui::Ui,
    pub settings: Settings,
}

impl Engine {
    pub fn new() -> Result<Engine, Box<dyn Error>> {
        let event_loop = glium::glutin::EventsLoop::new();
        let f = File::open("settings.ron").expect("Failed opening file");
        let settings: Settings = match ron::de::from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load settings: {}", e);
                std::process::exit(1);
            }
        };

        println!("{:?}", settings);

        let wb = glium::glutin::WindowBuilder::new().with_dimensions(LogicalSize::new(settings.graphics.window_width as f64, settings.graphics.window_height as f64)).with_title("Valala");
        let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24).with_multisampling(4);
        let mut display = glium::Display::new(wb, cb, &event_loop)?;
        let resource_pack =  resource::ResourcePack::new(&display);
        let world = world::World::new(&display);
        let lobby = Lobby::new();
        let lobby: Box<dyn GameState> = Box::new(lobby);
        lobby.enter();
        let states = vec![lobby];
        let picker = Picker::new(&display);
        let ui = ui::Ui::new(&mut display);

        Ok(
            Engine {
                event_loop,
                display,
                resource_pack,
                picker,
                settings,
                cursor_state: CursorState {
                    position: None,
                },
                world,
                states,
                ui,
            }
        )
    }

    pub fn run(&mut self) {
        let mut action = Action::Continue;
        let mut previous_clock = Instant::now();

        self.picker.initialize_picking_attachments(&self.display, (self.settings.graphics.window_width, self.settings.graphics.window_height));

        loop {
            match self.update() {
                Action::Continue => {
                    let now = Instant::now();
                    let _fps = 1_000_000_000/(now - previous_clock).as_nanos();
                    // println!("{}", _fps);
                    previous_clock = now;
                },
                Action::Push(gamestate) => {
                    if let Some(prevstate) = self.states.last() {
                        prevstate.pause();
                    }
                    self.states.push(gamestate);
                },
                Action::Pop => {
                    self.states.pop().unwrap().leave();
                    if let Some(gamestate) = self.states.last() {
                        gamestate.resume();
                    }
                },
                Action::Quit => {
                    break;
                },
            }
        }
    }

    fn update(&mut self) -> Action {
        let mut target = self.display.draw();

        let mut action = match self.states.last() {
            Some(gamestate) => gamestate.update(&mut self.world),
            None => Action::Quit,
        };

        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        let picked_object = self.picker.get_picked_object();

        let mut picking_target_opt = self.picker.target(&self.display);

        for entity in self.world.scene.iter_entities() {
            let uniforms = glium::uniform! {
                tex: match &entity.texture_id {
                    Some(tex) => self.resource_pack.get_texture(&tex),
                    None => self.resource_pack.get_texture(&resource::TextureId::Terrain),
                },
                u_light: [-1.0, 0.4, 0.9f32],
                view: self.world.camera.view,
                model: self.world.camera.model,
                perspective: self.world.camera.perspective,
            };

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };

            if entity.visible {
                if let Some(triangles) = entity.triangles.as_ref() {
                    match entity.normals.as_ref() {
                        Some(normals) => {
                            target.draw((&entity.vertices, normals), triangles, self.resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
                        },
                        None => {}
                    };
                    if entity.pickable {
                        if let Some(picking_target) = picking_target_opt.as_mut() {
                            picking_target.draw(&entity.vertices, triangles, self.resource_pack.get_shader(&resource::ShaderId::Picking), &uniforms, &params).unwrap();
                        }
                    }
                }
                if let Some(lines) = entity.lines.as_ref() {
                    target.draw(&entity.vertices, lines, self.resource_pack.get_shader(&resource::ShaderId::Line), &uniforms, &params).unwrap();
                }

                if let Some(mesh_id) = entity.mesh_id.as_ref() {
                    target.draw((&self.resource_pack.get_mesh(mesh_id).vertices, &self.resource_pack.get_mesh(mesh_id).normals), &self.resource_pack.get_mesh(mesh_id).indices, self.resource_pack.get_shader(&resource::ShaderId::Character), &uniforms, &params).unwrap();
                }
            }
        }

        target.finish().unwrap();
        self.picker.commit(self.cursor_state.position);

        let cursor_state = &mut self.cursor_state;
        let camera = &mut self.world.camera;
        let picker = &mut self.picker;
        let display = &mut self.display;
        self.event_loop.poll_events(|e| {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested => action = Action::Quit,
                        glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize{width, height}) => {
                            picker.initialize_picking_attachments(display, (width as u32, height as u32));
                            camera.scale((height/width) as f32);
                        },
                        glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                            if let glium::glutin::MouseScrollDelta::LineDelta(x, y) = delta {
                                camera.zoom(y);
                            }
                        },
                        glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                            cursor_state.position = Some(position.into());
                        },
                        _ => (),
                    }
                },
                glium::glutin::Event::DeviceEvent { .. } => (),
                _ => (),
            }
        });

        action
    }


}
