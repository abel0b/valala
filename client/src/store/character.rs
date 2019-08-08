use rand::Rng;
use valala_engine::{math::Deg, scene::NodeId};

#[derive(Copy, Clone)]
pub enum Orientation {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl From<u8> for Orientation {
    fn from(n: u8) -> Orientation {
        match n {
            0 => Orientation::Zero,
            1 => Orientation::One,
            2 => Orientation::Two,
            3 => Orientation::Three,
            4 => Orientation::Four,
            5 => Orientation::Five,
            _ => panic!("can't convert {} into Orientation", n),
        }
    }
}

impl Orientation {
    pub fn angle(self) -> Deg<f32> {
        Deg((self as i32 as f32) * 60.0)
    }

    pub fn random() -> Orientation {
        let mut rng = rand::thread_rng();
        Orientation::from(rng.gen_range(0, 6))
    }
}

pub struct Character {
    pub entity: NodeId,
    pub position: (i32, i32, i32),
    pub orientation: Orientation,
    pub scale: f32,
}

impl Character {
    pub fn new(entity: NodeId) -> Character {
        Character {
            entity,
            position: (0, 0, 0),
            orientation: Orientation::random(),
            scale: 1.0,
        }
    }
}
