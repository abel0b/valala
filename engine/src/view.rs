use crate::geometry::{Geometry, GeometryBuilder};
use crate::scene::NodeId;
use crate::store::Store;

pub trait Renderable<S, A> {
    fn render(&self, store: &Store<S, A>, node: NodeId) -> View;
}

pub trait Hoverable<A> {
    fn hover_enter(&self, node: NodeId) -> A;
    fn hover_leave(&self, node: NodeId) -> A;
}

#[derive(Default)]
pub struct View {
    pub geometries: Vec<Geometry>,
}

impl View {
    pub fn new() -> View {
        Default::default()
    }
}

pub struct ViewBuilder {
    id: u32,
    geometries: Vec<GeometryBuilder>,
}

impl ViewBuilder {
    pub fn with_id(id: u32) -> ViewBuilder {
        ViewBuilder {
            id,
            geometries: Vec::new(),
        }
    }

    pub fn from_node(node: NodeId) -> ViewBuilder {
        match node {
            NodeId::Entity(id) => ViewBuilder {
                id,
                geometries: Vec::new(),
            },
            _ => panic!("not an entity node"),
        }
    }

    pub fn geometry(&mut self) -> &mut GeometryBuilder {
        self.geometries.push(GeometryBuilder::with_id(self.id));
        self.geometries.last_mut().unwrap()
    }

    pub fn build(&self) -> View {
        View {
            geometries: self.geometries.iter().map(|g| g.build()).collect(),
        }
    }
}
