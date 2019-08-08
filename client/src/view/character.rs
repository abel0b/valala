use crate::store::State;
use valala_engine::{
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
        character
            .model(ModelId("character"))
            .shader(ShaderId("model"))
            .texture(TextureId("character"))
            .scale(state.scale * 0.45)
            .rotate_y(state.orientation.angle());

        view.build()
    }
}

// impl Hoverable<Action> for Character {
//     fn hover(&self) -> Action {
//         Action::HoverCharacter(42)
//     }
// }
