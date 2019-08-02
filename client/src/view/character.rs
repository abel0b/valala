use valala_engine::{
    geometry::{Geometry, GeometryBuilder},
    resource::{TextureId, ModelId},
    view::View,
    math::Deg,
};

#[derive(Copy, Clone)]
enum Orientation {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl Orientation {
    pub fn angle(self) -> Deg<f32> {
        Deg((self as i32 as f32) * 60.0)
    }
}

pub struct Character {
    orientation: Orientation,
    scale: f32,
}

impl Default for Character {
    fn default() -> Character {
        Character {
            orientation: Orientation::One,
            scale: 1.0,
        }
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
        character.scale(self.scale * 0.45);
        let angle = self.orientation.angle();
        character.rotate_y(angle);

        vec![character.build()]
    }
}
