use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub id: u32,
    pub color: (f32, f32, f32, f32),
    pub data: (f32, f32, f32, f32),
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
}

#[derive(Copy, Clone)]
pub struct SimpleVertex {
    pub position: (f32, f32, f32),
}

implement_vertex!(Vertex, id, data, color, position, tex_coords);
implement_vertex!(SimpleVertex, position);
