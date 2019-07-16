use crate::data::Map;
use valala_engine::prelude::{Action, Context, GameState, Scene};
use valala_engine::camera::Camera;
use valala_engine::scene::Node;

pub struct Lobby;

impl GameState for Lobby {
    fn enter(&mut self, _ctx: &Context, scene: &mut Scene) {
        let map = Map::new_hexagonal(scene);
        scene
            .append(Node::with_camera(Camera::isometric(1280.0/720.0)))
            .append(Node::with_view(Box::new(map)));
    }

    fn frame(&mut self, _ctx: &Context, _scene: &mut Scene) -> Action {
        Action::Continue
    }
}
