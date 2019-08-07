use core::f32::consts::PI;
use valala_engine::scene::NodeId;

pub struct Tile {
    pub entity: NodeId,
    pub hovered: bool,
    pub q: i32,
    pub r: i32,
    pub y: i32,
    pub center: (f32, f32),
    pub corners_up: [(f32, f32, f32); 6],
    pub corners_down: [(f32, f32, f32); 6],
}

impl Tile {
    pub const CONVMAT: [[f32; 2]; 2] = [[3.0 / 2.0, 0.0], [1.732_050_8 / 2.0, 1.732_050_8]];
    pub const SIZE: (f32, f32) = (1.0, 1.0);
    pub const HEIGHT: f32 = 0.5;

    pub fn new(entity: NodeId, q: i32, r: i32, y: i32) -> Tile {
        let center = Self::center(q, r);
        let corners_down = [
            Self::corner(center, 0, (y as f32) * Self::HEIGHT),
            Self::corner(center, 1, (y as f32) * Self::HEIGHT),
            Self::corner(center, 2, (y as f32) * Self::HEIGHT),
            Self::corner(center, 3, (y as f32) * Self::HEIGHT),
            Self::corner(center, 4, (y as f32) * Self::HEIGHT),
            Self::corner(center, 5, (y as f32) * Self::HEIGHT),
        ];
        let corners_up = [
            Self::corner(center, 0, (y as f32) * Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 1, (y as f32) * Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 2, (y as f32) * Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 3, (y as f32) * Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 4, (y as f32) * Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 5, (y as f32) * Self::HEIGHT + Self::HEIGHT),
        ];
        Tile {
            entity,
            hovered: false,
            q,
            r,
            y,
            center,
            corners_up,
            corners_down,
        }
    }

    pub fn center(q: i32, r: i32) -> (f32, f32) {
        (
            (Self::CONVMAT[0][0] * (q as f32) + Self::CONVMAT[0][1] * (r as f32)) * Self::SIZE.0,
            (Self::CONVMAT[1][0] * (q as f32) + Self::CONVMAT[1][1] * (r as f32)) * Self::SIZE.1,
        )
    }

    pub fn corner(center: (f32, f32), corner: u8, y: f32) -> (f32, f32, f32) {
        let start_angle = 0.0;
        let angle = 2.0 * PI * (start_angle + f32::from(corner)) / 6.0;
        (
            center.0 + Self::SIZE.0 * angle.cos(),
            y,
            center.1 + Self::SIZE.1 * angle.sin(),
        )
    }
}
