#[derive(Copy, Clone)]
pub struct Vertex {
    pub id: u32,
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

glium::implement_vertex!(Vertex, id, position, tex_coords);
