use valala_engine::{
    geometry::{Geometry},
    resource::{TextureId, ModelId},
    view::View,
};

pub struct Character;

impl Default for Character {
    fn default() -> Character {
        Character
    }
}

impl Character {
    pub fn new() -> Character {
        Default::default()
    }
}

impl View for Character {
    fn render(&self) -> Vec<Geometry> {
        let character = Geometry::with_model_and_texture(ModelId("character"), TextureId("character"));

        vec![character]
    }
}
