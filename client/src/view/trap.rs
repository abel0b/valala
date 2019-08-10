use crate::store::{State, Tile};
use valala_engine::{
    math::Vector3,
    scene::Uid,
    store::Store,
    view::{View, ViewBuilder},
};

pub fn render(store: &Store<State>, uid: Uid) -> View {
    let mut view = ViewBuilder::with_id(uid.0);
    let state = store
        .world
        .traps
        .values()
        .find(|t| t.entity == uid)
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
