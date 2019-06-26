use crate::hex::HexTile;
use crate::vertex::Vertex;
use crate::identifier::Identifier;
use std::collections::HashMap;

pub struct HexagonalMap {
    pub tiles: HashMap<u32, (i32,i32)>,
    pub vertices_buffer: glium::VertexBuffer<Vertex>,
    pub indices_buffer: glium::IndexBuffer<u32>,
    pub border_indices_buffer: glium::IndexBuffer<u32>,
}

impl HexagonalMap {
    // TODO: init vec with exact capacity
    pub fn new(display: &glium::Display, map_radius: i32) -> HexagonalMap {
        let mut tiles = HashMap::new();
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut border_indices = Vec::new();
        let mut id: u32;
        let mut count = 1;
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                let center = HexTile::center(q, r);
                id = Identifier::gen();
                tiles.insert(id, (q, r));
                vertices.push(Vertex { id, position: (center.0, 0.0, center.1), tex_coords: (0.5, 0.5) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 0, 0.0), tex_coords: (0.0, 0.5) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 1, 0.0), tex_coords: (0.333_333, 0.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 2, 0.0), tex_coords: (0.666_666, 0.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 3, 0.0), tex_coords: (1.0, 0.5) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 4, 0.0), tex_coords: (0.666_666, 1.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 5, 0.0), tex_coords: (0.333_333, 1.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 0, -2.0), tex_coords: (0.0, 0.5) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 1, -2.0), tex_coords: (0.333_333, 0.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 2, -2.0), tex_coords: (0.666_666, 0.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 3, -2.0), tex_coords: (1.0, 0.5) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 4, -2.0), tex_coords: (0.666_666, 1.0) });
                vertices.push(Vertex { id, position: HexTile::corner(center, 5, -2.0), tex_coords: (0.333_333, 1.0) });

                indices.extend_from_slice(&[
                    (count-1)*13, 1+(count-1)*13, 2+(count-1)*13,
                    (count-1)*13, 2+(count-1)*13, 3+(count-1)*13,
                    (count-1)*13, 3+(count-1)*13, 4+(count-1)*13,
                    (count-1)*13, 4+(count-1)*13, 5+(count-1)*13,
                    (count-1)*13, 5+(count-1)*13, 6+(count-1)*13,
                    (count-1)*13, 6+(count-1)*13, 1+(count-1)*13,
                ]);

                border_indices.extend_from_slice(&[
                    1+(count-1)*13, 2+(count-1)*13,
                    2+(count-1)*13, 3+(count-1)*13,
                    3+(count-1)*13, 4+(count-1)*13,
                    4+(count-1)*13, 5+(count-1)*13,
                    5+(count-1)*13, 6+(count-1)*13,
                    6+(count-1)*13, 1+(count-1)*13,
                    7+(count-1)*13, 8+(count-1)*13,
                    8+(count-1)*13, 9+(count-1)*13,
                    10+(count-1)*13, 11+(count-1)*13,
                    11+(count-1)*13, 12+(count-1)*13,
                    12+(count-1)*13, 7+(count-1)*13,
                    1+(count-1)*13, 7+(count-1)*13,
                    2+(count-1)*13, 8+(count-1)*13,
                    3+(count-1)*13, 9+(count-1)*13,
                    4+(count-1)*13, 10+(count-1)*13,
                    5+(count-1)*13, 11+(count-1)*13,
                    6+(count-1)*13, 12+(count-1)*13,
                ]);
                count += 1;
            }
        }
        HexagonalMap {
            tiles,
            vertices_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            indices_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
            border_indices_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &border_indices).unwrap(),
        }
    }
}
