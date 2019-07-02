use std::string::String;
use crate::entity::EntityId;
use crate::scene::Scene;

pub struct Character {
    id: EntityId,
    health: u16,
    mana: u8,
    name: String,
}

impl Character {
    pub fn new(scene: &Scene, health: u16, mana: u8, name: String) -> Character {
        Character {
            id: 42,
            health,
            mana,
            name,
        }
    }
}
