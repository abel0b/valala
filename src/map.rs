use crate::hex::HexTile;
use rand::Rng;
use crate::entity::{EntityId, EntityFactory};
use crate::scene::Scene;
use crate::character::Character;
use crate::resource::MeshId;

const TILE_HEIGHT: f32 = 0.5;

pub struct Map {
    // pub tiles: HashMap<u32, (i32,i32,i32)>,
    map_entity: EntityId,
    player_entity: EntityId,
    player: Character,
}

impl Map {
    pub fn new_hexagonal(scene: &mut Scene, display: &glium::Display, map_radius: i32) -> Map {
        let mut map_factory = EntityFactory::new(scene).visible().pickable();
        let mut rng = rand::thread_rng();

        let mut tile_number = 0;
        for q in -map_radius..=map_radius {
            let r1 = std::cmp::max(-map_radius, -q - map_radius);
            let r2 = std::cmp::min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                map_factory = Self::add_hex_tile(map_factory, q, r, 0, tile_number*13);
                tile_number += 1;

                if rng.gen_range(0.0, 10.0) < 2.0 {
                    map_factory = Self::add_hex_tile(map_factory, q, r, 1, tile_number*13);
                    tile_number += 1;
                }
            }
        }

        let map_entity = map_factory.build(display);
        let player_entity = EntityFactory::new(scene).visible().pickable().mesh(MeshId::Character).build(display);

        Map {
            map_entity: scene.add_entity(map_entity),
            player_entity: scene.add_entity(player_entity),
            player: Character::new(scene, 500, 10, "Nargan".to_string()),
        }
    }

    fn add_hex_tile(map_factory: EntityFactory, q: i32, r: i32, y: i32, offset: u32) -> EntityFactory {
        let center = HexTile::center(q, r);
        map_factory
            .group()
            .vertex((center.0, (y as f32)*TILE_HEIGHT, center.1), (0.5, 0.5), (q as f32, r as f32, y as f32))
            .normal((0.0, 1.0, 0.0))
            .vertex(HexTile::corner(center, 0, (y as f32)*TILE_HEIGHT), (0.0, 0.5), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(0, 1.0))
            .vertex(HexTile::corner(center, 1, (y as f32)*TILE_HEIGHT), (0.333_333, 0.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(1, 1.0))
            .vertex(HexTile::corner(center, 2, (y as f32)*TILE_HEIGHT), (0.666_666, 0.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(2, 1.0))
            .vertex(HexTile::corner(center, 3, (y as f32)*TILE_HEIGHT), (1.0, 0.5), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(3, 1.0))
            .vertex(HexTile::corner(center, 4, (y as f32)*TILE_HEIGHT), (0.666_666, 1.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(4, 1.0))
            .vertex(HexTile::corner(center, 5, (y as f32)*TILE_HEIGHT), (0.333_333, 1.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(5, 1.0))
            .vertex(HexTile::corner(center, 0, ((y as f32) - 1.0)*TILE_HEIGHT), (0.0, 0.5), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(0, -1.0))
            .vertex(HexTile::corner(center, 1, ((y as f32) - 1.0)*TILE_HEIGHT), (0.333_333, 0.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(1, -1.0))
            .vertex(HexTile::corner(center, 2, ((y as f32) - 1.0)*TILE_HEIGHT), (0.666_666, 0.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(2, -1.0))
            .vertex(HexTile::corner(center, 3, ((y as f32) - 1.0)*TILE_HEIGHT), (1.0, 0.5), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(3, -1.0))
            .vertex(HexTile::corner(center, 4, ((y as f32) - 1.0)*TILE_HEIGHT), (0.666_666, 1.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(4, -1.0))
            .vertex(HexTile::corner(center, 5, ((y as f32) - 1.0)*TILE_HEIGHT), (0.333_333, 1.0), (q as f32, r as f32, y as f32))
            .normal(HexTile::normal(5, -1.0))
            .triangle(offset, 1+offset, 2+offset)
            .triangle(offset, 2+offset, 3+offset)
            .triangle(offset, 3+offset, 4+offset)
            .triangle(offset, 4+offset, 5+offset)
            .triangle(offset, 5+offset, 6+offset)
            .triangle(offset, 6+offset, 1+offset)
            .triangle(offset+1, offset+7, offset+8)
            .triangle(offset+1, offset+2, offset+8)
            .triangle(offset+2, offset+8, offset+9)
            .triangle(offset+2, offset+3, offset+9)
            .triangle(offset+3, offset+9, offset+10)
            .triangle(offset+3, offset+4, offset+10)
            .triangle(offset+4, offset+10, offset+11)
            .triangle(offset+4, offset+5, offset+11)
            .triangle(offset+5, offset+11, offset+12)
            .triangle(offset+5, offset+6, offset+12)
            .triangle(offset+6, offset+12, offset+7)
            .triangle(offset+6, offset+1, offset+7)
            .line(1+offset, 2+offset)
            .line(2+offset, 3+offset)
            .line(3+offset, 4+offset)
            .line(4+offset, 5+offset)
            .line(5+offset, 6+offset)
            .line(6+offset, 1+offset)
            .line(7+offset, 8+offset)
            .line(8+offset, 9+offset)
            .line(10+offset, 11+offset)
            .line(11+offset, 12+offset)
            .line(12+offset, 7+offset)
            .line(1+offset, 7+offset)
            .line(2+offset, 8+offset)
            .line(3+offset, 9+offset)
            .line(4+offset, 10+offset)
            .line(5+offset, 11+offset)
            .line(6+offset, 12+offset)
    }
}
