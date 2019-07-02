use glium;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use glium::glutin::dpi::LogicalSize;
use std::time::Instant;
use glium::{uniform,Surface};
use std::collections::HashMap;

use crate::gamestate;
use crate::picking::Picker;
use crate::ui;
use crate::world;
use crate::resource;
use crate::mesh;

const WINDOW_WIDTH: f64 = 1080.0;
const WINDOW_HEIGHT: f64 = 720.0;

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
    pub world: world::World,
    pub lobby: gamestate::Lobby,
    pub ui: ui::Ui,
}

#[derive(Eq, PartialEq)]
enum Action {
    Continue,
    Close,
}

impl Engine {
    pub fn new() -> Result<Engine, Box<dyn Error>> {
        let event_loop = glium::glutin::EventsLoop::new();

        let wb = glium::glutin::WindowBuilder::new().with_dimensions(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)).with_title("Valala");
        let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24).with_multisampling(4);
        let mut display = glium::Display::new(wb, cb, &event_loop)?;
        let resource_pack =  resource::ResourcePack::new(&display);
        let world = world::World::new(&display);
        let lobby = gamestate::Lobby::new();
        let picker = Picker::new(&display);
        let ui = ui::Ui::new(&mut display);

        Ok(
            Engine {
                event_loop,
                display,
                resource_pack,
                picker,
                cursor_state: CursorState {
                    position: None,
                },
                world,
                lobby,
                ui,
            }
        )
    }

    pub fn run(&mut self) {
        let mut action = Action::Continue;
        let mut previous_clock = Instant::now();

        self.picker.initialize_picking_attachments(&self.display, (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));

        while action == Action::Continue {
            action = self.update();

            let now = Instant::now();
            let _fps = 1_000_000_000/(now - previous_clock).as_nanos();
            // println!("{}", _fps);
            previous_clock = now;
        }
    }

    fn update(&mut self) -> Action {
        let mut target = self.display.draw();

        let mut action = Action::Continue;

        gamestate::GameState::update(&self.lobby, &mut self.world);

        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        let picked_object = self.picker.get_picked_object();

        let mut picking_target_opt = self.picker.target(&self.display);


        for entity in self.world.scene.iter_entities() {

            let uniforms = match &entity.texture_id {
                Some(tex) => {
                    glium::uniform! {
                        tex: self.resource_pack.get_texture(&tex),
                        view: self.world.camera.view,
                        model: self.world.camera.model,
                        perspective: self.world.camera.perspective,
                    }
                },
                None => {
                    glium::uniform! {
                        tex: self.resource_pack.get_texture(&resource::TextureId::Terrain),
                        view: self.world.camera.view,
                        model: self.world.camera.model,
                        perspective: self.world.camera.perspective,
                    }
                }
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
                    target.draw(&entity.vertices, triangles, self.resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
                    if entity.pickable {
                        if let Some(picking_target) = picking_target_opt.as_mut() {
                            picking_target.draw(&entity.vertices, triangles, self.resource_pack.get_shader(&resource::ShaderId::Picking), &uniforms, &params).unwrap();
                        }
                    }
                }
                if let Some(lines) = entity.lines.as_ref() {
                    target.draw(&entity.vertices, lines, self.resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
                    if entity.pickable {
                        if let Some(picking_target) = picking_target_opt.as_mut() {
                            picking_target.draw(&entity.vertices, lines, self.resource_pack.get_shader(&resource::ShaderId::Picking), &uniforms, &params).unwrap();
                        }
                    }
                }
                if let Some(mesh_id) = entity.mesh_id.as_ref() {
                    target.draw(&self.resource_pack.get_mesh(mesh_id).vertices, &self.resource_pack.get_mesh(mesh_id).indices, self.resource_pack.get_shader(&resource::ShaderId::Character), &uniforms, &params).unwrap();
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
                        glium::glutin::WindowEvent::CloseRequested => action = Action::Close,
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
