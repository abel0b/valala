use crate::{
    store::{Action, State},
    store::{Character, Tile},
};
use rand::Rng;
use std::rc::Rc;
use valala_engine::{
    camera::Camera,
    prelude::{Context, Scene, Stage, Transition},
    scene::NodeId,
    store::Store,
    view::{Hoverable, Renderable},
};

pub struct Lobby;

impl Stage<State, Action> for Lobby {
    fn enter(
        &mut self,
        ctx: &Context,
        scene: &mut Scene<Action>,
        store: &mut Store<State, Action>,
    ) {
        store.state.camera = scene
            .add_camera(
                NodeId::Root,
                Camera::isometric(ctx.window.height as f32 / ctx.window.width as f32),
            )
            .unwrap();

        let mut rng = rand::thread_rng();
        let map_radius = 5;
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                let tile = Rc::new(Tile::new(q, r, 0));
                let tile_node = scene.add_entity(store.state.camera).unwrap();
                scene.set_renderable(tile_node, Rc::clone(&tile) as Rc<dyn Renderable>);
                scene.set_hoverable(tile_node, Rc::clone(&tile) as Rc<dyn Hoverable<Action>>);
                store.state.tiles.insert((q, r, 0), tile);
                if rng.gen_range(0.0, 10.0) < 2.0 {
                    let tile = Rc::new(Tile::new(q, r, 1));
                    let tile_node = scene.add_entity(store.state.camera).unwrap();
                    scene.set_renderable(tile_node, Rc::clone(&tile) as Rc<dyn Renderable>);
                    scene.set_hoverable(tile_node, Rc::clone(&tile) as Rc<dyn Hoverable<Action>>);
                    store.state.tiles.insert((q, r, 0), tile);
                }
            }
        }

        let character = Rc::new(Character::new());
        let character_node = scene.add_entity(store.state.camera).unwrap();
        scene.set_renderable(character_node, Rc::clone(&character) as Rc<dyn Renderable>);
        store.state.characters.push(character);
    }

    fn frame(
        &mut self,
        _ctx: &Context,
        _scene: &mut Scene<Action>,
        store: &mut Store<State, Action>,
    ) -> Transition<State, Action> {
        store.dispatch(Action::Nop);
        Transition::Continue
    }
}
