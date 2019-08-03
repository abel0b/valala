use valala_engine::{
    prelude::{Transition, Context, Stage, Scene},
    camera::Camera,
    scene::NodeId,
    store::Store,
};
use rand::Rng;
use crate::{
    view::{
        Character,
        Tile,
    },
    store::{State, Action},
};

pub struct Lobby;

impl Stage<State, Action> for Lobby {
    fn enter(&mut self, ctx: &Context, scene: &mut Scene) {
        let camera = scene.add_camera(NodeId::Root, Camera::isometric(ctx.window.height as f32 / ctx.window.width as f32)).unwrap();

        let mut rng = rand::thread_rng();
        let map_radius = 5;
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                let _tile = scene.add_view(camera, Box::new(Tile::new(q, r, 0)));
                if rng.gen_range(0.0, 10.0) < 2.0 {
                    let _tile = scene.add_view(camera, Box::new(Tile::new(q, r, 1)));
                }
            }
        }

        let _character = scene.add_view(camera, Box::new(Character::new()));
    }

    fn frame(&mut self, _ctx: &Context, _scene: &mut Scene, store: &mut Store<State,Action>) -> Transition<State,Action> {
        // store.dispatch(Action::Oof);
        Transition::Continue
    }
}
