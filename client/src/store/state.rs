use crate::store::{Character, Tile};
use std::collections::HashMap;
use valala_engine::scene::NodeId;

pub struct State {
    pub camera: Option<NodeId>,
    pub map: Option<NodeId>,
    pub tiles: HashMap<(i32, i32, i32), Tile>,
    pub actors: Vec<Character>,
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
