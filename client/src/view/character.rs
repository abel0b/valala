use crate::store::{State, Tile};
use valala_engine::{
    math::Vector3,
    resource::{ModelId, ShaderId, TextureId},
    scene::Uid,
    store::Store,
    view::{View, ViewBuilder},
};

pub fn render(store: &Store<State>, uid: Uid) -> View {
    let state = store.world.actors.iter().find(|t| t.entity == uid).unwrap();

    let mut view = ViewBuilder::with_id(uid.0);
    let character = view.geometry();

    let position = {
        let center = Tile::center(state.position.0, state.position.1);
        Vector3::new(center.0, 0.0, center.1)
    };

    character
        .model(ModelId("character"))
        .shader(ShaderId("model"))
        .texture(TextureId("character"))
        .translate(position)
        .rotate_y(state.orientation.angle())
        .scale(state.scale * 0.45);

    view.build()
}
