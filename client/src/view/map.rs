use valala_engine::{
    geometry::{Geometry, GeometryBuilder},
    resource::{ShaderId, TextureId},
    scene::Entity,
    view::View,
};

use crate::{data::Map, hex::HexTile};
use rand::Rng;

const TILE_HEIGHT: f32 = 0.5;

impl View for Map {
    fn render(&self, _entity: Entity) -> Vec<Geometry> {
        let mut map = GeometryBuilder::default();
        map.visible()
            .pickable()
            .shader(ShaderId("map"))
            .texture(TextureId("stone"));
        let mut grid = GeometryBuilder::default();
        grid.visible();
        let mut tile_number = 0;
        let mut rng = rand::thread_rng();
        let map_radius = 5;
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                add_hex_tile(&mut map, &mut grid, q, r, 0, tile_number * 13);
                tile_number += 1;

                if rng.gen_range(0.0, 10.0) < 2.0 {
                    add_hex_tile(&mut map, &mut grid, q, r, 1, tile_number * 13);
                    tile_number += 1;
                }
            }
        }

        vec![map.build(), grid.build()]
    }
}

fn add_hex_tile(
    map: &mut GeometryBuilder,
    grid: &mut GeometryBuilder,
    q: i32,
    r: i32,
    y: i32,
    offset: u32,
) {
    let center = HexTile::center(q, r);
    map.vertex(
        (center.0, (y as f32) * TILE_HEIGHT, center.1),
        (0.5, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .normal((0.0, 1.0, 0.0))
    .vertex(
        HexTile::corner(center, 0, (y as f32) * TILE_HEIGHT),
        (0.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(0, 1.0))
    .vertex(
        HexTile::corner(center, 1, (y as f32) * TILE_HEIGHT),
        (0.333_333, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(1, 1.0))
    .vertex(
        HexTile::corner(center, 2, (y as f32) * TILE_HEIGHT),
        (0.666_666, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(2, 1.0))
    .vertex(
        HexTile::corner(center, 3, (y as f32) * TILE_HEIGHT),
        (1.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(3, 1.0))
    .vertex(
        HexTile::corner(center, 4, (y as f32) * TILE_HEIGHT),
        (0.666_666, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(4, 1.0))
    .vertex(
        HexTile::corner(center, 5, (y as f32) * TILE_HEIGHT),
        (0.333_333, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(5, 1.0))
    .vertex(
        HexTile::corner(center, 0, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(0, -1.0))
    .vertex(
        HexTile::corner(center, 1, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.333_333, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(1, -1.0))
    .vertex(
        HexTile::corner(center, 2, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.666_666, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(2, -1.0))
    .vertex(
        HexTile::corner(center, 3, ((y as f32) - 1.0) * TILE_HEIGHT),
        (1.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(3, -1.0))
    .vertex(
        HexTile::corner(center, 4, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.666_666, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(4, -1.0))
    .vertex(
        HexTile::corner(center, 5, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.333_333, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .normal(HexTile::normal(5, -1.0))
    .triangle(offset, 1 + offset, 2 + offset)
    .triangle(offset, 2 + offset, 3 + offset)
    .triangle(offset, 3 + offset, 4 + offset)
    .triangle(offset, 4 + offset, 5 + offset)
    .triangle(offset, 5 + offset, 6 + offset)
    .triangle(offset, 6 + offset, 1 + offset)
    .triangle(offset + 1, offset + 7, offset + 8)
    .triangle(offset + 1, offset + 2, offset + 8)
    .triangle(offset + 2, offset + 8, offset + 9)
    .triangle(offset + 2, offset + 3, offset + 9)
    .triangle(offset + 3, offset + 9, offset + 10)
    .triangle(offset + 3, offset + 4, offset + 10)
    .triangle(offset + 4, offset + 10, offset + 11)
    .triangle(offset + 4, offset + 5, offset + 11)
    .triangle(offset + 5, offset + 11, offset + 12)
    .triangle(offset + 5, offset + 6, offset + 12)
    .triangle(offset + 6, offset + 12, offset + 7)
    .triangle(offset + 6, offset + 1, offset + 7);

    grid.vertex(
        (center.0, (y as f32) * TILE_HEIGHT, center.1),
        (0.5, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 0, (y as f32) * TILE_HEIGHT),
        (0.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 1, (y as f32) * TILE_HEIGHT),
        (0.333_333, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 2, (y as f32) * TILE_HEIGHT),
        (0.666_666, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 3, (y as f32) * TILE_HEIGHT),
        (1.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 4, (y as f32) * TILE_HEIGHT),
        (0.666_666, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 5, (y as f32) * TILE_HEIGHT),
        (0.333_333, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 0, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 1, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.333_333, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 2, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.666_666, 0.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 3, ((y as f32) - 1.0) * TILE_HEIGHT),
        (1.0, 0.5),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 4, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.666_666, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .vertex(
        HexTile::corner(center, 5, ((y as f32) - 1.0) * TILE_HEIGHT),
        (0.333_333, 1.0),
        (q as f32, r as f32, y as f32),
    )
    .line(1 + offset, 2 + offset)
    .line(2 + offset, 3 + offset)
    .line(3 + offset, 4 + offset)
    .line(4 + offset, 5 + offset)
    .line(5 + offset, 6 + offset)
    .line(6 + offset, 1 + offset)
    .line(7 + offset, 8 + offset)
    .line(8 + offset, 9 + offset)
    .line(10 + offset, 11 + offset)
    .line(11 + offset, 12 + offset)
    .line(12 + offset, 7 + offset)
    .line(1 + offset, 7 + offset)
    .line(2 + offset, 8 + offset)
    .line(3 + offset, 9 + offset)
    .line(4 + offset, 10 + offset)
    .line(5 + offset, 11 + offset)
    .line(6 + offset, 12 + offset);
}
