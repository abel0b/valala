use crate::store::Character;
use valala_engine::{
    resource::{ModelId, ShaderId, TextureId},
    view::{Renderable, View, ViewBuilder},
};

impl Renderable for Character {
    fn render(&self, mut view: ViewBuilder) -> View {
        let character = view.geometry();
        character
            .model(ModelId("character"))
            .shader(ShaderId("model"))
            .texture(TextureId("character"))
            .scale(self.scale * 0.45)
            .rotate_y(self.orientation.angle());

        view.build()
    }
}

// impl Hoverable<Action> for Character {
//     fn hover(&self) -> Action {
//         Action::HoverCharacter(42)
//     }
// }
