
const CONVMAT: [f32; 4] = [3.0/2.0, 0.0, 1.73205080757/2.0, 1.73205080757];
// const CONVMAT_I: [f32; 4] = [1.73205080757/3.0, -1.0/3.0, 0.0, 2.0/3.0];
const HEX_SIZE_X: f32 = 1.0;
const HEX_SIZE_Z: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub id: u32,
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

glium::implement_vertex!(Vertex, id, position, tex_coords);

pub struct Hex {
    pub id: u32,
    pub center: (f32, f32),
    pub coordinates: (i32, i32, i32),
    pub vertices_buffer: glium::VertexBuffer<Vertex>,
    pub indices_buffer: glium::IndexBuffer<u32>,
    pub border_indices_buffer: glium::IndexBuffer<u32>,
}

impl Hex {
    pub fn new(id: u32, display: &glium::Display, q: i32, r: i32, s: i32) -> Hex {
        let center = (
            (CONVMAT[0] * (q as f32) + CONVMAT[1] * (r as f32)) * HEX_SIZE_X,
            (CONVMAT[2] * (q as f32) + CONVMAT[3] * (r as f32)) * HEX_SIZE_Z,
        );
        let vertices: [Vertex; 13] = [
            Vertex { id: id, position: (center.0, 0.0, center.1), tex_coords: (0.5, 0.5) },
            Vertex { id: id, position: Self::corner(&center, 0, 0.0), tex_coords: (0.0, 0.5) },
            Vertex { id: id, position: Self::corner(&center, 1, 0.0), tex_coords: (0.3333333, 0.0) },
            Vertex { id: id, position: Self::corner(&center, 2, 0.0), tex_coords: (0.6666666, 0.0) },
            Vertex { id: id, position: Self::corner(&center, 3, 0.0), tex_coords: (1.0, 0.5) },
            Vertex { id: id, position: Self::corner(&center, 4, 0.0), tex_coords: (0.6666666, 1.0) },
            Vertex { id: id, position: Self::corner(&center, 5, 0.0), tex_coords: (0.3333333, 1.0) },
            Vertex { id: id, position: Self::corner(&center, 0, -2.0), tex_coords: (0.0, 0.5) },
            Vertex { id: id, position: Self::corner(&center, 1, -2.0), tex_coords: (0.3333333, 0.0) },
            Vertex { id: id, position: Self::corner(&center, 2, -2.0), tex_coords: (0.6666666, 0.0) },
            Vertex { id: id, position: Self::corner(&center, 3, -2.0), tex_coords: (1.0, 0.5) },
            Vertex { id: id, position: Self::corner(&center, 4, -2.0), tex_coords: (0.6666666, 1.0) },
            Vertex { id: id, position: Self::corner(&center, 5, -2.0), tex_coords: (0.3333333, 1.0) },
       ];
       let border_indices: [u32; 34] = [
           1,2,
           2,3,
           3,4,
           4,5,
           5,6,
           6,1,
           7,8,
           8,9,
           10,11,
           11,12,
           12,7,
           1,7,
           2,8,
           3,9,
           4,10,
           5,11,
           6,12,
       ];

       let indices: [u32; 18] = [
           0,1,2,
           0,2,3,
           0,3,4,
           0,4,5,
           0,5,6,
           0,6,1,
       ];
        Hex {
            id: id,
            center: center,
            coordinates: (q, r, s),
            vertices_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            indices_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
            border_indices_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &border_indices).unwrap(),
        }
    }
    //
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

    fn corner(center: &(f32, f32), corner: u32, y: f32) -> (f32, f32, f32) {
        let start_angle = 0.0;
        let angle = 2.0 * 3.14159 * (start_angle + (corner as f32)) / 6.0;
        (center.0 + HEX_SIZE_X * angle.cos(), y, center.1 + HEX_SIZE_Z * angle.sin())
    }
}
