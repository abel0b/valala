use core::f32::consts::PI;
use crate::hex::HexTile;
use crate::mesh;
use crate::world;

pub trait GameState {
    fn update(&self, world: &mut world::World);
}

pub struct Lobby;

impl Lobby {
    pub fn new() -> Lobby {
        Lobby {

        }
    }
}

impl GameState for Lobby {
    fn update(&self, world: &mut world::World) {
        // let params = glium::DrawParameters {
        //     depth: glium::Depth {
        //         test: glium::DepthTest::IfLess,
        //         write: true,
        //         .. Default::default()
        //     },
        //     .. Default::default()
        // };
        // let uniforms = glium::uniform! {
        //     tex: resource_pack.get_texture(&resource::TextureId::Terrain),
        //     coordinates: (0.0f32, 0.0),
        //     view: world.camera.view,
        //     model: world.camera.model,
        //     perspective: world.camera.perspective,
        // };

        // if let Some(mut picking_target) = game.picker.target(&game.display) {
        //     picking_target.clear_depth(1.0);
        //     picking_target.draw(&game.world.map.vertices_buffer, &game.world.map.indices_buffer, &game.world.picking_program, &uniforms, &params).unwrap();
        // }

        // if let Some(object_id) = picked_object {
        //     if let Some((q, r, _z)) = world.map.tiles.get(&object_id) {
        //         let center = HexTile::center(*q,*r);
        //         let radius = 0.4;
        //         let cursor_vertices = [
        //             mesh::SimpleVertex { position: (center.0, 0.0, center.1) },
        //             mesh::SimpleVertex { position: (center.0+(0.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(0.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(1.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(1.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(2.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(2.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(3.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(3.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(4.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(4.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(5.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(5.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(6.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(6.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(7.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(7.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(8.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(8.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(9.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(9.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(10.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(10.0*(2.0*PI/12.0)).sin()*radius) },
        //             mesh::SimpleVertex { position: (center.0+(11.0*(2.0*PI/12.0)).cos()*radius, 0.01, center.1+(11.0*(2.0*PI/12.0)).sin()*radius) },
        //         ];
        //         let cursor_indices: [u32;36] = [0,1,2,0,2,3,0,3,4,0,4,5,0,5,6,0,6,7,0,7,8,0,8,9,0,9,10,0,10,11,0,11,12,0,12,1];
        //
        //         let cursor_vertices_buffer = glium::VertexBuffer::new(&game.display, &cursor_vertices).unwrap();
        //         let cursor_indices_buffer = glium::IndexBuffer::new(&game.display, glium::index::PrimitiveType::TrianglesList, &cursor_indices).unwrap();
        //
        //         target.draw(&cursor_vertices_buffer, &cursor_indices_buffer, &game.world.path_program, &uniforms, &params).unwrap();
        //     }
        // }

        // target.draw(&world.map.vertices_buffer, &world.map.indices_buffer, resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
        // target.draw(&world.map.vertices_buffer, &world.map.border_indices_buffer, resource_pack.get_shader(&resource::ShaderId::Grid), &uniforms, &params).unwrap();
        // target.draw(&world.character_vertices_buffer, &world.character_indices_buffer, resource_pack.get_shader(&resource::ShaderId::Character), &uniforms, &params).unwrap();
    }
}
