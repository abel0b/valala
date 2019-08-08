use valala_engine::scene::NodeId;

pub struct Trap {
    pub entity: NodeId,
    pub q: i32,
    pub r: i32,
    pub y: i32,
}

impl Trap {
    pub fn new(entity: NodeId, q: i32, r: i32, y: i32) -> Trap {
        Trap { entity, q, r, y }
    }
}
