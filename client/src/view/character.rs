use valala_engine::{
    geometry::{Geometry, GeometryBuilder},
    resource::{TextureId, ModelId},
    view::View,
    math::Deg,
};

enum Orientation {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

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
        let mut character = GeometryBuilder::with_model_and_texture(ModelId("character"), TextureId("character"));
        character.scale(0.45);
        let orientation = Orientation::Four;
        let angle = Deg((orientation as i32 as f32) * 60.0);
        character.rotate_y(angle);

        vec![character.build()]
    }
}
