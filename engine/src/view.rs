use crate::{geometry::Geometry, scene::Entity};

pub trait View {
    fn render(&self) -> Vec<Geometry>;
}
