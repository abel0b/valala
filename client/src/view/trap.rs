use crate::store::{State, Tile};
use valala_engine::{
    math::Vector3,
    scene::NodeId,
    store::Store,
    view::{Renderable, View, ViewBuilder},
};

pub struct TrapEntity;

impl Renderable<State> for TrapEntity {
    fn render(&self, store: &Store<State>, node: NodeId) -> View {
        let mut view = ViewBuilder::from_node(node);
        let state = store
            .world
            .traps
            .values()
            .find(|t| t.entity == node)
            .unwrap();

        let center = Tile::center(state.q, state.r);

        let trap = view.geometry();

        let color = (0.8, 0.1, 0.1, 1.0);
        let width = 0.4;

        trap.vertex((-width, 0.5, -width), color, (0.0, 0.0))
            .vertex((-width, 0.5, width), color, (0.0, 0.0))
            .vertex((width, 0.5, -width), color, (0.0, 0.0))
            .vertex((width, 0.5, width), color, (0.0, 0.0))
            .triangle(0, 1, 2)
            .triangle(1, 2, 3)
            .translate(Vector3::new(center.0, 0.0, center.1));

        view.build()
    }
}
