use std::collections::HashMap;
use crate::entity::{Entity, EntityId};

// TODO: remove unwraps

pub struct Scene {
    next_id: u16,
    entities: HashMap<EntityId, Entity>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_id: 1,
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> u16 {
        let id = entity.id;
        self.entities.insert(entity.id, entity);
        id
    }

    pub fn generate_id(&mut self) -> u16 {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).unwrap();
        id
    }

    // pub fn pick_object(&self, id: &u32) {
    //     self.entities[id].select();
    // }

    pub fn iter_entities(&self) -> std::collections::hash_map::Values<u16, Entity> {
        self.entities.values()
    }

    // pub fn hide_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = false;
    // }
    //
    // pub fn show_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = true;
    // }
}
