use crate::store::tile::{TileKind, TileState};
use crate::store::Map;
use crate::store::{Action, Character, Tile, Trap};
use crate::view::{character, tile, trap};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use valala_engine::scene::Scene;
use valala_engine::store::Store;
use valala_engine::{scene::Camera, scene::Uid};
use valala_engine::{
    scene::{Entity, NodeIndex},
    store::World,
};

pub struct State {
    pub camera: Option<Uid>,
    pub map: Map,
    pub traps: HashMap<(i32, i32, i32), Trap>,
    pub actors: Vec<Character>,
}

impl World for State {
    type Action = Action;

    fn apply(store: &mut Store<State>, scene: &mut Scene<State>, action: Action) {
        match action {
            Action::HoverEnterTile(node) => {
                let tile_coords = {
                    let tile = store
                        .world
                        .map
                        .tiles
                        .values_mut()
                        .find(|t| t.entity == node)
                        .unwrap();
                    println!("tile ({} {} {})", tile.q, tile.r, tile.y);

                    if tile.kind == TileKind::Ground {
                        tile.state = TileState::Hover;
                        Some((tile.q, tile.r))
                    } else {
                        None
                    }
                };

                if let Some(coords) = tile_coords {
                    let player = store.world.actors.first().unwrap();
                    if let Some(path) = store
                        .world
                        .map
                        .shortest_path((player.position.0, player.position.1), coords)
                    {
                        store.world.map.set_path(path);
                    }
                }
            }
            Action::MouseDownTile(node) => {
                let tile = store
                    .world
                    .map
                    .tiles
                    .values_mut()
                    .find(|t| t.entity == node)
                    .unwrap();
                if tile.kind == TileKind::Ground {
                    store.world.actors.first_mut().unwrap().position = (tile.q, tile.r, tile.y);
                }
            }
            Action::HoverLeaveTile(node) => {
                store.world.map.set_path(Vec::new());
                store
                    .world
                    .map
                    .tiles
                    .values_mut()
                    .find(|t| t.entity == node)
                    .unwrap()
                    .state = TileState::Normal;
            }
            Action::EnterLobby => {
                store.world.camera = Some(
                    scene
                        .add_camera(
                            NodeIndex::Root,
                            Camera::isometric(
                                store.context.window.height as f32
                                    / store.context.window.width as f32,
                            ),
                        )
                        .unwrap(),
                );
            }
            Action::LoadRandomMap => {
                store.world.map.entity = Some(
                    scene
                        .add_group(NodeIndex::Camera(store.world.camera.unwrap()))
                        .unwrap(),
                );

                let mut rng: StdRng = SeedableRng::from_seed([2; 32]);
                let map_radius = 5;
                for q in -map_radius..=map_radius {
                    let r1 = std::cmp::max(-map_radius, -q - map_radius);
                    let r2 = std::cmp::min(map_radius, -q + map_radius);
                    for r in r1..=r2 {
                        let kind = if rng.gen_range(0.0, 10.0) < 2.0 {
                            TileKind::Obstacle
                        } else {
                            TileKind::Ground
                        };

                        let tile_node = scene
                            .add_entity(
                                NodeIndex::Group(store.world.map.entity.unwrap()),
                                Entity {
                                    render: Some(tile::render),
                                    on_hover_enter: Some(tile::on_hover_enter),
                                    on_hover_leave: Some(tile::on_hover_leave),
                                    on_mouse_down: Some(tile::on_mouse_down),
                                    on_mouse_up: Some(tile::on_mouse_up),
                                },
                            )
                            .unwrap();
                        store
                            .world
                            .map
                            .tiles
                            .insert((q, r), Tile::new(tile_node, q, r, kind));
                    }
                }

                let character_node = scene
                    .add_entity(
                        NodeIndex::Group(store.world.map.entity.unwrap()),
                        Entity {
                            render: Some(character::render),
                            on_hover_enter: None,
                            on_hover_leave: None,
                            on_mouse_down: None,
                            on_mouse_up: None,
                        },
                    )
                    .unwrap();
                store.world.actors.push(Character::new(character_node));
                let trap = scene
                    .add_entity(
                        NodeIndex::Group(store.world.map.entity.unwrap()),
                        Entity {
                            render: Some(trap::render),
                            on_hover_enter: None,
                            on_hover_leave: None,
                            on_mouse_down: None,
                            on_mouse_up: None,
                        },
                    )
                    .unwrap();
                store
                    .world
                    .traps
                    .insert((2, 2, 1), Trap::new(trap, 2, 2, 1));
            }
            Action::Nop => {}
        }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            camera: None,
            traps: HashMap::new(),
            map: Map::new(),
            actors: Vec::new(),
        }
    }
}

impl State {
    pub fn new() -> State {
        Default::default()
    }
}
