use crate::{geometry::Geometry};

pub trait View {
    fn render(&self) -> Vec<Geometry>;
}
