use valala_engine::prelude::{Scene, Entity};

#[derive(Default)]
pub struct Map {
    map_entity: Entity,
}

impl Map {
    pub fn new_hexagonal(scene: &mut Scene) -> Map {
        Map {
            map_entity: 1,
        }
    }
}
