mod character;
mod map;
mod state;
mod tile;
use crate::view::{CharacterEntity, TileEntity};
pub use character::Character;
use rand::Rng;
pub use state::State;
use std::rc::Rc;
pub use tile::Tile;
use valala_engine::{
    camera::Camera,
    scene::NodeId,
    scene::Scene,
    store::Store,
    view::{Hoverable, Renderable},
};

pub enum Action {
    EnterLobby,
    LoadRandomMap,
    HoverEnterTile(NodeId),
    HoverLeaveTile(NodeId),
    Nop,
}

pub fn reducer(store: &mut Store<State, Action>, scene: &mut Scene<State, Action>, action: Action) {
    match action {
        Action::HoverEnterTile(node) => {
            store
                .state
                .tiles
                .values_mut()
                .find(|t| t.entity == node)
                .unwrap()
                .hovered = true;
        }
        Action::HoverLeaveTile(node) => {
            store
                .state
                .tiles
                .values_mut()
                .find(|t| t.entity == node)
                .unwrap()
                .hovered = false;
        }
        Action::EnterLobby => {
            store.state.camera = Some(
                scene
                    .add_camera(
                        NodeId::Root,
                        Camera::isometric(
                            store.context.window.height as f32 / store.context.window.width as f32,
                        ),
                    )
                    .unwrap(),
            );
        }
        Action::LoadRandomMap => {
            store.state.map = Some(scene.add_group(store.state.camera.unwrap()).unwrap());
            let map = store.state.map.unwrap();

            let mut rng = rand::thread_rng();
            let map_radius = 5;
            for q in -map_radius..=map_radius {
                let r1 = std::cmp::max(-map_radius, -q - map_radius);
                let r2 = std::cmp::min(map_radius, -q + map_radius);
                for r in r1..=r2 {
                    let tile_node = scene.add_entity(map).unwrap();
                    store
                        .state
                        .tiles
                        .insert((q, r, 0), Tile::new(tile_node, q, r, 0));
                    let tile_entity = Rc::new(TileEntity);
                    scene.set_renderable(
                        tile_node,
                        Rc::clone(&tile_entity) as Rc<dyn Renderable<State, Action>>,
                    );
                    scene.set_hoverable(
                        tile_node,
                        Rc::clone(&tile_entity) as Rc<dyn Hoverable<Action>>,
                    );
                    if rng.gen_range(0.0, 10.0) < 2.0 {
                        let tile_node = scene.add_entity(map).unwrap();
                        store
                            .state
                            .tiles
                            .insert((q, r, 1), Tile::new(tile_node, q, r, 1));
                        let tile_entity = Rc::new(TileEntity);
                        scene.set_renderable(
                            tile_node,
                            Rc::clone(&tile_entity) as Rc<dyn Renderable<State, Action>>,
                        );
                        scene.set_hoverable(
                            tile_node,
                            Rc::clone(&tile_entity) as Rc<dyn Hoverable<Action>>,
                        );
                    }
                }
            }

            let character_node = scene.add_entity(map).unwrap();
            store.state.actors.push(Character::new(character_node));
            let character_entity = Rc::new(CharacterEntity);
            scene.set_renderable(
                character_node,
                character_entity as Rc<dyn Renderable<State, Action>>,
            );
        }
        Action::Nop => {}
    }
}
