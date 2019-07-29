use valala_engine::{
    geometry::{Geometry, GeometryBuilder},
    resource::{ShaderId},
    view::View,
};
use core::f32::consts::PI;

pub struct Tile {
    q: i32,
    r: i32,
    y: i32,
    center: (f32, f32),
    corners_up: [(f32, f32, f32); 6],
    corners_down: [(f32, f32, f32); 6],
}

impl Tile {
    const CONVMAT: [[f32; 2]; 2] = [[3.0 / 2.0, 0.0], [1.732_050_8 / 2.0, 1.732_050_8]];
    const SIZE: (f32, f32) = (1.0, 1.0);
    const HEIGHT: f32 = 0.5;

    pub fn new(q: i32, r: i32, y: i32) -> Tile {
        let center = Self::center(q, r);
        let corners_down = [
            Self::corner(center, 0, (y as f32)*Self::HEIGHT),
            Self::corner(center, 1, (y as f32)*Self::HEIGHT),
            Self::corner(center, 2, (y as f32)*Self::HEIGHT),
            Self::corner(center, 3, (y as f32)*Self::HEIGHT),
            Self::corner(center, 4, (y as f32)*Self::HEIGHT),
            Self::corner(center, 5, (y as f32)*Self::HEIGHT),
        ];
        let corners_up = [
            Self::corner(center, 0, (y as f32)*Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 1, (y as f32)*Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 2, (y as f32)*Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 3, (y as f32)*Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 4, (y as f32)*Self::HEIGHT + Self::HEIGHT),
            Self::corner(center, 5, (y as f32)*Self::HEIGHT + Self::HEIGHT),
        ];
        Tile {
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

impl View for Tile {
    fn render(&self) -> Vec<Geometry> {
        let mut tile = GeometryBuilder::default();
        let mut border = GeometryBuilder::default();
        border.shader(ShaderId("color"));
        let color = if self.y == 0 {
            let a = (((self.q - self.r) % 3 + 3) % 3) as f32;
            (0.85+a*0.1, 0.85+a*0.1, 0.85+a*0.1, 0.85+a*0.1)
        }
        else {
            (0.5, 0.5, 0.5, 1.0)
        };

        tile.shader(ShaderId("color"));

        tile.vertex(
            (self.center.0, (self.y as f32) * Self::HEIGHT, self.center.1),
            color,
            (0.5, 0.5),
        )
        .vertex(
            self.corners_up[0],
            color,
            (0.0, 0.5),
        )
        .vertex(
            self.corners_up[1],
            color,
            (0.333_333, 0.0),
        )
        .vertex(
            self.corners_up[2],
            color,
            (0.666_666, 0.0),
        )
        .vertex(
            self.corners_up[3],
            color,
            (1.0, 0.5),
        )
        .vertex(
            self.corners_up[4],
            color,
            (0.666_666, 1.0),
        )
        .vertex(
            self.corners_up[5],
            color,
            (0.333_333, 1.0),
        )
        .vertex(
            self.corners_down[0],
            color,
            (0.0, 0.5),
        )
        .vertex(
            self.corners_down[1],
            color,
            (0.333_333, 0.0),
        )
        .vertex(
            self.corners_down[2],
            color,
            (0.666_666, 0.0),
        )
        .vertex(
            self.corners_down[3],
            color,
            (1.0, 0.5),
        )
        .vertex(
            self.corners_down[4],
            color,
            (0.666_666, 1.0),
        )
        .vertex(
            self.corners_down[5],
            color,
            (0.333_333, 1.0),
        )
        .triangle(0, 1, 2)
        .triangle(0, 2, 3)
        .triangle(0, 3, 4)
        .triangle(0, 4, 5)
        .triangle(0, 5, 6)
        .triangle(0, 6, 1)
        .triangle(1, 7, 8)
        .triangle(1, 2, 8)
        .triangle(2, 8, 9)
        .triangle(2, 3, 9)
        .triangle(3, 9, 10)
        .triangle(3, 4, 10)
        .triangle(4, 10, 11)
        .triangle(4, 5, 11)
        .triangle(5, 11, 12)
        .triangle(5, 6, 12)
        .triangle(6, 12, 7)
        .triangle(6, 1, 7);

        let color = (0.2,0.2,0.2,1.0);

        border.vertex(
            (self.center.0, (self.y as f32) * Self::HEIGHT, self.center.1),
            color,
            (0.5, 0.5),
        )
        .vertex(
            self.corners_up[0],
            color,
            (0.0, 0.5),
        )
        .vertex(
            self.corners_up[1],
            color,
            (0.333_333, 0.0),
        )
        .vertex(
            self.corners_up[2],
            color,
            (0.666_666, 0.0),
        )
        .vertex(
            self.corners_up[3],
            color,
            (1.0, 0.5),
        )
        .vertex(
            self.corners_up[4],
            color,
            (0.666_666, 1.0),
        )
        .vertex(
            self.corners_up[5],
            color,
            (0.333_333, 1.0),
        )
        .vertex(
            self.corners_down[0],
            color,
            (0.0, 0.5),
        )
        .vertex(
            self.corners_down[1],
            color,
            (0.333_333, 0.0),
        )
        .vertex(
            self.corners_down[2],
            color,
            (0.666_666, 0.0),
        )
        .vertex(
            self.corners_down[3],
            color,
            (1.0, 0.5),
        )
        .vertex(
            self.corners_down[4],
            color,
            (0.666_666, 1.0),
        )
        .vertex(
            self.corners_down[5],
            color,
            (0.333_333, 1.0),
        )
        .line(1, 2)
        .line(2, 3)
        .line(3, 4)
        .line(4, 5)
        .line(5, 6)
        .line(6, 1)
        .line(7, 8)
        .line(8, 9)
        .line(10, 11)
        .line(11, 12)
        .line(12, 7)
        .line(1, 7)
        .line(2, 8)
        .line(3, 9)
        .line(4, 10)
        .line(5, 11)
        .line(6, 12);

        vec![tile.build(), border.build()]
    }
}
