use crate::store::{State, Tile};
use valala_engine::{
    math::Vector3,
    resource::{ModelId, ShaderId, TextureId},
    scene::NodeId,
    store::Store,
    view::{Renderable, View, ViewBuilder},
};

pub struct CharacterEntity;

impl Renderable<State> for CharacterEntity {
    fn render(&self, store: &Store<State>, node: NodeId) -> View {
        let state = store
            .world
            .actors
            .iter()
            .find(|t| t.entity == node)
            .unwrap();

        let mut view = ViewBuilder::from_node(node);
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
}

// impl Hoverable<Action> for Character {
//     fn hover(&self) -> Action {
//         Action::HoverCharacter(42)
//     }
// }
