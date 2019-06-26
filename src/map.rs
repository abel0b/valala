use crate::hex::HexTile;
use crate::vertex::Vertex;
use crate::identifier::Identifier;
use std::collections::HashMap;
use rand::Rng;
const TILE_HEIGHT: f32 = 0.5;


pub struct HexagonalMap {
    pub tiles: HashMap<u32, (i32,i32,i32)>,
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
        let mut rng = rand::thread_rng();
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                Self::add_hex_tile(&mut vertices, &mut tiles, q, r, 0);

                if rng.gen_range(0.0, 10.0) < 2.0 {
                    Self::add_hex_tile(&mut vertices, &mut tiles, q, r, 1);
                }

                let offset = (count-1)*13;

                indices.extend_from_slice(&[
                    offset, 1+offset, 2+offset,
                    offset, 2+offset, 3+offset,
                    offset, 3+offset, 4+offset,
                    offset, 4+offset, 5+offset,
                    offset, 5+offset, 6+offset,
                    offset, 6+offset, 1+offset,
                    offset+1, offset+7, offset+8,
                    offset+1, offset+2, offset+8,
                    offset+2, offset+8, offset+9,
                    offset+2, offset+3, offset+9,
                    offset+3, offset+9, offset+10,
                    offset+3, offset+4, offset+10,
                    offset+4, offset+10, offset+11,
                    offset+4, offset+5, offset+11,
                    offset+5, offset+11, offset+12,
                    offset+5, offset+6, offset+12,
                    offset+6, offset+12, offset+7,
                    offset+6, offset+1, offset+7,
                ]);

                border_indices.extend_from_slice(&[
                    1+offset, 2+offset,
                    2+offset, 3+offset,
                    3+offset, 4+offset,
                    4+offset, 5+offset,
                    5+offset, 6+offset,
                    6+offset, 1+offset,
                    7+offset, 8+offset,
                    8+offset, 9+offset,
                    10+offset, 11+offset,
                    11+offset, 12+offset,
                    12+offset, 7+offset,
                    1+offset, 7+offset,
                    2+offset, 8+offset,
                    3+offset, 9+offset,
                    4+offset, 10+offset,
                    5+offset, 11+offset,
                    6+offset, 12+offset,
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

    fn add_hex_tile(vertices: &mut Vec<Vertex>, tiles: &mut  HashMap<u32, (i32,i32,i32)>, q: i32, r: i32, y: i32) {
        let id = Identifier::gen();
        tiles.insert(id, (q, r, y));
        let center = HexTile::center(q, r);
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: (center.0, (y as f32)*TILE_HEIGHT, center.1), tex_coords: (0.5, 0.5) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 0, (y as f32)*TILE_HEIGHT), tex_coords: (0.0, 0.5) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 1, (y as f32)*TILE_HEIGHT), tex_coords: (0.333_333, 0.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 2, (y as f32)*TILE_HEIGHT), tex_coords: (0.666_666, 0.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 3, (y as f32)*TILE_HEIGHT), tex_coords: (1.0, 0.5) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 4, (y as f32)*TILE_HEIGHT), tex_coords: (0.666_666, 1.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 5, (y as f32)*TILE_HEIGHT), tex_coords: (0.333_333, 1.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 0, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (0.0, 0.5) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 1, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (0.333_333, 0.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 2, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (0.666_666, 0.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 3, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (1.0, 0.5) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 4, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (0.666_666, 1.0) });
        vertices.push(Vertex { id, coordinates: (q as f32, r as f32, y as f32), position: HexTile::corner(center, 5, (y as f32)*TILE_HEIGHT-TILE_HEIGHT), tex_coords: (0.333_333, 1.0) });
    }

    pub fn tile(&self, id: u32) -> Option<&(i32, i32, i32)> {
        self.tiles.get(&id)
    }
}
