use crate::map;
use glium::{glutin, Surface, uniform};
use core::f32::consts::PI;
use crate::game::Game;
use crate::vertex::Vertex;
use crate::hex::HexTile;

pub trait GameState {
    fn update(game: &mut Game, target: &mut glium::Frame, picked_object: Option<u32>);
}

pub struct World {
    map: map::HexagonalMap,
    terrain_program: glium::Program,
    grid_program: glium::Program,
    path_program: glium::Program,
    picking_program: glium::Program,
}

impl World {
    pub fn new(display: &glium::Display) -> World {
        World {
            map: map::HexagonalMap::new(display, 5),
            terrain_program: glium::Program::from_source(display, include_str!("./shader/terrain.vert"), include_str!("./shader/terrain.frag"), None).unwrap(),
            grid_program: glium::Program::from_source(display, include_str!("./shader/grid.vert"), include_str!("./shader/grid.frag"), None).unwrap(),
            path_program: glium::Program::from_source(display, include_str!("./shader/path.vert"), include_str!("./shader/path.frag"), None).unwrap(),
            picking_program: glium::Program::from_source(display, include_str!("./shader/picking.vert"), include_str!("./shader/picking.frag"), None).unwrap(),
        }
    }
}

pub struct InWorld;

impl InWorld {
    pub fn update(game: &mut Game, target: &mut glium::Frame, picked_object: Option<u32>) {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let uniforms = glium::uniform! {
            tex: &game.resource_pack.images.get("terrain.png").unwrap().texture,
            coordinates: (0.0f32, 0.0),
            view: game.camera.view,
            model: game.camera.model,
            perspective: game.camera.perspective,
        };

        if let Some(mut picking_target) = game.picker.target(&game.display) {
            picking_target.clear_depth(1.0);
            picking_target.draw(&game.world.map.vertices_buffer, &game.world.map.indices_buffer, &game.world.picking_program, &uniforms, &params).unwrap();
        }

        if let Some(object_id) = picked_object {
            if let Some((q, r)) = game.world.map.tiles.get(&object_id) {
                let center = HexTile::center(*q,*r);
                let radius = 0.4;
                let cursor_vertices = [
                    Vertex { id: 10000, position: (center.0, 0.0, center.1), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(0.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(0.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(1.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(1.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(2.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(2.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(3.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(3.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(4.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(4.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(5.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(5.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(6.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(6.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(7.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(7.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(8.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(8.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(9.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(9.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(10.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(10.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                    Vertex { id: 10000, position: (center.0+(11.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(11.0*(2.0*PI/12.0)).sin()*radius), tex_coords: (0.0, 0.0) },
                ];
                let cursor_indices: [u32;36] = [0,1,2,0,2,3,0,3,4,0,4,5,0,5,6,0,6,7,0,7,8,0,8,9,0,9,10,0,10,11,0,11,12,0,12,1];

                let cursor_vertices_buffer = glium::VertexBuffer::new(&game.display, &cursor_vertices).unwrap();
                let cursor_indices_buffer = glium::IndexBuffer::new(&game.display, glium::index::PrimitiveType::TrianglesList, &cursor_indices).unwrap();

                target.draw(&cursor_vertices_buffer, &cursor_indices_buffer, &game.world.path_program, &uniforms, &params).unwrap();
            }
        }

        target.draw(&game.world.map.vertices_buffer, &game.world.map.indices_buffer, &game.world.terrain_program, &uniforms, &params).unwrap();
        target.draw(&game.world.map.vertices_buffer, &game.world.map.border_indices_buffer, &game.world.grid_program, &uniforms, &params).unwrap();
    }
}
