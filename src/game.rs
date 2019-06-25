use glium;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use glium::glutin::dpi::LogicalSize;
use std::time::Instant;
use glium::{Surface};

use crate::camera::Camera;
use crate::resource::ResourcePack;
use crate::state;
use crate::picking::Picker;

const WINDOW_WIDTH: f64 = 1080.0;
const WINDOW_HEIGHT: f64 = 720.0;
const CLEAR_COLOR: (f32, f32, f32, f32) = (0.9, 0.9, 0.9, 1.0);

struct CursorState {
    position: Option<(i32, i32)>,
}

pub struct Game {
    event_loop: glium::glutin::EventsLoop,
    pub display: glium::Display,
    pub picker: Picker,
    pub camera: Camera,
    pub resource_pack: ResourcePack,
    cursor_state: CursorState,
    pub world: state::World,
}

#[derive(Eq, PartialEq)]
enum Action {
    Continue,
    Close,
}

impl Game {
    pub fn new() -> Result<Game, Box<dyn Error>> {
        let event_loop = glium::glutin::EventsLoop::new();

        let wb = glium::glutin::WindowBuilder::new().with_dimensions(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24).with_multisampling(4);
        let display = glium::Display::new(wb, cb, &event_loop)?;
        let resource_pack =  ResourcePack::new(&display);
        let world = state::World::new(&display);
        let picker = Picker::new(&display);

        Ok(
            Game {
                event_loop,
                display,
                camera: Camera::new((WINDOW_HEIGHT/WINDOW_WIDTH) as f32),
                resource_pack,
                picker,
                cursor_state: CursorState {
                    position: None,
                },
                world,
            }
        )
    }

    pub fn start(&mut self) {
        let mut action = Action::Continue;
        let mut previous_clock = Instant::now();

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

        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        let picked_object = self.picker.get_picked_object();
        self.picker.initialize(&self.display, target.get_dimensions());

        state::InWorld::update(self, &mut target, picked_object);

        target.finish().unwrap();
        self.picker.commit(self.cursor_state.position);

        let cursor_state = &mut self.cursor_state;
        let camera = &mut self.camera;
        self.event_loop.poll_events(|e| {
            match e {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested => action = Action::Close,
                        glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize{width, height}) => {
                            camera.scale((height/width) as f32)
                        }
                        glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                            cursor_state.position = Some(position.into());
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        });

        action
    }


}
