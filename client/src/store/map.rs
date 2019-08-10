use crate::store::Tile;
use crate::store::{TileKind, TileState};
use pathfinding::prelude::astar;
use std::collections::HashMap;
use valala_engine::scene::Uid;

#[derive(Default)]
pub struct Map {
    pub path: Vec<(i32, i32)>,
    pub entity: Option<Uid>,
    pub tiles: HashMap<(i32, i32), Tile>,
}

// var axial_directions = [
//     Hex(+1, 0), Hex(+1, -1), Hex(0, -1),
//     Hex(-1, 0), Hex(-1, +1), Hex(0, +1),
// ]
//
// function hex_direction(direction):
//     return axial_directions[direction]
//
// function hex_neighbor(hex, direction):
//     var dir = hex_direction(direction)
//     return Hex(hex.q + dir.q, hex.r + dir.r)

fn hex_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    (((a.0 - b.0).abs() + (a.0 + a.1 - b.0 - b.1).abs() + (a.1 - b.1).abs()) / 2) as u32
}

impl Map {
    pub fn new() -> Map {
        Default::default()
    }

    pub fn successors(&self, hex: (i32, i32)) -> Vec<((i32, i32), u32)> {
        let mut neighbors = Vec::new();
        for dir in [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)].iter() {
            if let Some(tile) = self.tiles.get(&hex) {
                if tile.kind == TileKind::Ground {
                    neighbors.push((hex.0 + dir.0, hex.1 + dir.1))
                }
            }
        }

        neighbors.into_iter().map(|p| (p, 1)).collect()
    }

    pub fn set_path(&mut self, path: Vec<(i32, i32)>) {
        for position in self.path.iter() {
            if let Some(tile) = self.tiles.get_mut(position) {
                tile.state = TileState::Normal;
            }
        }

        self.path = path;
        for position in self.path.iter() {
            if let Some(tile) = self.tiles.get_mut(position) {
                tile.state = TileState::Path;
            }
        }
    }

    pub fn shortest_path(
        &self,
        origin: (i32, i32),
        destination: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        let result = astar(
            &origin,
            |p| self.successors(*p),
            |p| hex_distance(*p, destination),
            |p| *p == destination,
        );
        match result {
            Some((path, _c)) => Some(path),
            None => None,
        }
    }
}

impl Map {}
