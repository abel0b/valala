pub struct Hex {
    coordinates: (i32, i32, i32),
}

const CONVMAT: [f32; 4] = [1.73205080757, 1.73205080757/2.0, 0.0, 3.0/2.0];
const CONVMAT_I: [f32; 4] = [1.73205080757/3.0, -1.0/3.0, 0.0, 2.0/3.0];
const HEX_SIZE: f32 = 0.5;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

glium::implement_vertex!(Vertex, position, tex_coords);

impl Hex {
    pub fn new(q: i32, r: i32, s: i32) -> Hex {
        Hex {
            coordinates: (q, r, s),
        }
    }

    fn add(&self, other: &Hex) -> Hex {
        Hex::new(self.coordinates.0 + other.coordinates.0, self.coordinates.1 + other.coordinates.1, self.coordinates.2 + other.coordinates.2)
    }

    fn substract(&self, other: &Hex) -> Hex {
        Hex::new(self.coordinates.0 - other.coordinates.0, self.coordinates.1 - other.coordinates.1, self.coordinates.2 - other.coordinates.2)
    }

    fn multiply(&self, other: &Hex) -> Hex {
        Hex::new(self.coordinates.0 * other.coordinates.0, self.coordinates.1 * other.coordinates.1, self.coordinates.2 * other.coordinates.2)
    }

    fn length(&self) -> i32 {
         (self.coordinates.0.abs() + self.coordinates.1.abs() + self.coordinates.2.abs()) / 2
    }

    fn distance(&self, other: &Hex) -> i32 {
        self.substract(other).length()
    }

    fn center(&self) -> (f32, f32) {
        (
            (CONVMAT[0] * (self.coordinates.0 as f32) + CONVMAT[1] * (self.coordinates.1 as f32)) * HEX_SIZE,
            (CONVMAT[2] * (self.coordinates.0 as f32) + CONVMAT[3] * (self.coordinates.1 as f32)) * HEX_SIZE,
        )
    }

    fn corner(&self, center: &(f32, f32), corner: u32) -> (f32, f32, f32) {
        let start_angle = 0.5;
        let angle = 2.0 * 3.14159 * (start_angle + (corner as f32)) / 6.0;
        (center.0 + HEX_SIZE * angle.cos(), 0.0, center.1 + HEX_SIZE * angle.sin())
    }

    pub fn vertices(&self) -> [Vertex; 7] {
        let center = self.center();
        [
            Vertex { position: (center.0, 0.0, center.1), tex_coords: (0.5, 0.5) },
            Vertex { position: self.corner(&center, 0), tex_coords: (0.0, 0.5) },
            Vertex { position: self.corner(&center, 1), tex_coords: (0.3333333, 0.0) },
            Vertex { position: self.corner(&center, 2), tex_coords: (0.6666666, 0.0) },
            Vertex { position: self.corner(&center, 3), tex_coords: (1.0, 0.5) },
            Vertex { position: self.corner(&center, 4), tex_coords: (0.6666666, 1.0) },
            Vertex { position: self.corner(&center, 5), tex_coords: (0.3333333, 1.0) },
       ]
    }

    pub fn border_indices(&self) -> [u32; 12] {
        [
            1,2,
            2,3,
            3,4,
            4,5,
            5,6,
            6,1,
        ]
    }

    pub fn indices(&self) -> [u32; 18] {
        [
            0,1,2,
            0,2,3,
            0,3,4,
            0,4,5,
            0,5,6,
            0,6,1,
        ]
    }
}
