use crate::store::{Action, Character, Tile};
use crate::view::{CharacterEntity, TileEntity};
use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;
use valala_engine::scene::Scene;
use valala_engine::store::Store;
use valala_engine::{
    camera::Camera,
    view::{Hoverable, Renderable},
};
use valala_engine::{scene::NodeId, store::World};

pub struct State {
    pub camera: Option<NodeId>,
    pub map: Option<NodeId>,
    pub tiles: HashMap<(i32, i32, i32), Tile>,
    pub actors: Vec<Character>,
}

impl World for State {
    type Action = Action;

    fn apply(store: &mut Store<State>, scene: &mut Scene<State>, action: Action) {
        match action {
            Action::HoverEnterTile(node) => {
                store
                    .world
                    .tiles
                    .values_mut()
                    .find(|t| t.entity == node)
                    .unwrap()
                    .hovered = true;
            }
            Action::HoverLeaveTile(node) => {
                store
                    .world
                    .tiles
                    .values_mut()
                    .find(|t| t.entity == node)
                    .unwrap()
                    .hovered = false;
            }
            Action::EnterLobby => {
                store.world.camera = Some(
                    scene
                        .add_camera(
                            NodeId::Root,
                            Camera::isometric(
                                store.context.window.height as f32
                                    / store.context.window.width as f32,
                            ),
                        )
                        .unwrap(),
                );
            }
            Action::LoadRandomMap => {
                store.world.map = Some(scene.add_group(store.world.camera.unwrap()).unwrap());
                let map = store.world.map.unwrap();

                let mut rng = rand::thread_rng();
                let map_radius = 5;
                for q in -map_radius..=map_radius {
                    let r1 = std::cmp::max(-map_radius, -q - map_radius);
                    let r2 = std::cmp::min(map_radius, -q + map_radius);
                    for r in r1..=r2 {
                        let tile_node = scene.add_entity(map).unwrap();
                        store
                            .world
                            .tiles
                            .insert((q, r, 0), Tile::new(tile_node, q, r, 0));
                        let tile_entity = Rc::new(TileEntity);
                        scene.set_renderable(
                            tile_node,
                            Rc::clone(&tile_entity) as Rc<dyn Renderable<State>>,
                        );
                        scene.set_hoverable(
                            tile_node,
                            Rc::clone(&tile_entity) as Rc<dyn Hoverable<State>>,
                        );
                        if rng.gen_range(0.0, 10.0) < 2.0 {
                            let tile_node = scene.add_entity(map).unwrap();
                            store
                                .world
                                .tiles
                                .insert((q, r, 1), Tile::new(tile_node, q, r, 1));
                            let tile_entity = Rc::new(TileEntity);
                            scene.set_renderable(
                                tile_node,
                                Rc::clone(&tile_entity) as Rc<dyn Renderable<State>>,
                            );
                            scene.set_hoverable(
                                tile_node,
                                Rc::clone(&tile_entity) as Rc<dyn Hoverable<State>>,
                            );
                        }
                    }
                }

                let character_node = scene.add_entity(map).unwrap();
                store.world.actors.push(Character::new(character_node));
                let character_entity = Rc::new(CharacterEntity);
                scene.set_renderable(
                    character_node,
                    character_entity as Rc<dyn Renderable<State>>,
                );
            }
            Action::Nop => {}
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            camera: None,
            map: None,
            tiles: HashMap::new(),
            actors: Vec::new(),
        }
    }
}

impl State {
    pub fn new() -> State {
        Default::default()
    }
}
