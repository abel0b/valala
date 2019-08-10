use rand::Rng;
use valala_engine::scene::Uid;

#[derive(Copy, Clone)]
pub enum Orientation {
    Forward = 0,
    Right = 1,
    RightBack = 2,
    Back = 3,
    LeftBack = 4,
    Left = 5,
}

impl From<u8> for Orientation {
    fn from(n: u8) -> Orientation {
        match n {
            0 => Orientation::Forward,
            1 => Orientation::Right,
            2 => Orientation::RightBack,
            3 => Orientation::Back,
            4 => Orientation::LeftBack,
            5 => Orientation::Left,
            _ => panic!("can't convert {} into Orientation", n),
        }
    }
}

impl Orientation {
    pub fn angle(self) -> f32 {
        (self as i32 as f32) * 60.0
    }

    pub fn random() -> Orientation {
        let mut rng = rand::thread_rng();
        Orientation::from(rng.gen_range(0, 6))
    }
}

pub struct Character {
    pub entity: Uid,
    pub position: (i32, i32, i32),
    pub orientation: Orientation,
    pub scale: f32,
}

impl Character {
    pub fn new(entity: Uid) -> Character {
        Character {
            entity,
            position: (0, 0, 0),
            orientation: Orientation::random(),
            scale: 1.0,
        }
    }
}
