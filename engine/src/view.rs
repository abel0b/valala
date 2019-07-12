use crate::{
    geometry::Geometry,
    scene::Entity,
};

pub trait View {
    fn render(&self, entity: Entity) -> Vec<Geometry>;
}
