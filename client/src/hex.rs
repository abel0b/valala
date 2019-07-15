use std::f32::consts::PI;

static CONVMAT: [[f32; 2]; 2] = [[3.0 / 2.0, 0.0], [1.732_050_8 / 2.0, 1.732_050_8]];

const HEX_SIZE_X: f32 = 1.0;
const HEX_SIZE_Z: f32 = 1.0;

pub struct HexTile {
    pub id: u32,
    pub center: (f32, f32),
    pub coordinates: (i32, i32, i32),
}

impl HexTile {
    // fn add(&self, other: &Hex) -> Hex {
    //     Hex::new(self.coordinates.0 + other.coordinates.0, self.coordinates.1 + other.coordinates.1, self.coordinates.2 + other.coordinates.2)
    // }
    //
    // fn substract(&self, other: &Hex) -> Hex {
    //     Hex::new(self.coordinates.0 - other.coordinates.0, self.coordinates.1 - other.coordinates.1, self.coordinates.2 - other.coordinates.2)
    // }
    //
    // fn multiply(&self, other: &Hex) -> Hex {
    //     Hex::new(self.coordinates.0 * other.coordinates.0, self.coordinates.1 * other.coordinates.1, self.coordinates.2 * other.coordinates.2)
    // }
    //
    // fn length(&self) -> i32 {
    //      (self.coordinates.0.abs() + self.coordinates.1.abs() + self.coordinates.2.abs()) / 2
    // }
    //
    // fn distance(&self, other: &Hex) -> i32 {
    //     self.substract(other).length()
    // }

    pub fn center(q: i32, r: i32) -> (f32, f32) {
        (
            (CONVMAT[0][0] * (q as f32) + CONVMAT[0][1] * (r as f32)) * HEX_SIZE_X,
            (CONVMAT[1][0] * (q as f32) + CONVMAT[1][1] * (r as f32)) * HEX_SIZE_Z,
        )
    }

    pub fn corner(center: (f32, f32), corner: u32, y: f32) -> (f32, f32, f32) {
        let start_angle = 0.0;
        let angle = 2.0 * PI * (start_angle + (corner as f32)) / 6.0;
        (
            center.0 + HEX_SIZE_X * angle.cos(),
            y,
            center.1 + HEX_SIZE_Z * angle.sin(),
        )
    }

    pub fn normal(corner: u32, y: f32) -> (f32, f32, f32) {
        let start_angle = 0.0;
        let angle = 2.0 * PI * (start_angle + (corner as f32)) / 6.0;
        (HEX_SIZE_X * angle.cos(), y, HEX_SIZE_Z * angle.sin())
    }
}
