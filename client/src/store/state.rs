use crate::store::{Character, Tile};
use std::collections::HashMap;
use std::rc::Rc;
use valala_engine::scene::NodeId;

pub struct State {
    pub camera: NodeId,
    pub tiles: HashMap<(i32, i32, i32), Rc<Tile>>,
    pub characters: Vec<Rc<Character>>,
}

impl Default for State {
    fn default() -> State {
        State {
            camera: NodeId::Root,
            tiles: HashMap::new(),
            characters: Vec::new(),
        }
    }
}

impl State {
    pub fn new() -> State {
        Default::default()
    }
}
