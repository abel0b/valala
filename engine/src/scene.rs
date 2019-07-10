use std::collections::HashMap;
use crate::entity::{Entity, EntityId};
use crate::mesh::{Vertex, Normal};
use glium::{uniform,Surface};

use crate::{
    resource,
    resource::ShaderId,
    picking::Picker,
    camera::Camera,
    context::Context,
};

// TODO: remove unwraps

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);

pub struct Scene {
    next_id: u16,
    entities: HashMap<EntityId, Entity>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_id: 1,
            entities: HashMap::new(),
            camera: Camera::isometric(1280.0 / 720.0)
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> u16 {
        let id = entity.id;
        self.entities.insert(entity.id, entity);
        id
    }

    pub fn generate_id(&mut self) -> u16 {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).unwrap();
        id
    }

    pub fn render(&self, ctx: &Context) {
        let mut target = ctx.backend.display.draw();
        // let mut picking_target_opt = picker.target(&display);
        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        for entity in self.iter_entities() {
            let shader = match entity.shader_id.as_ref() {
                Some(shader_id) => ctx.resource_pack.get_shader(shader_id),
                None => ctx.resource_pack.get_shader(&ShaderId("default")),
            };
            let uniforms = glium::uniform! {
                // tex: match &entity.texture_id {
                //     Some(tex) => ctx.resource_pack.get_texture(&tex),
                //     None => ctx.resource_pack.get_texture(&resource::TextureId::Terrain),
                // },
                u_light: [-1.0, 0.4, 0.9f32],
                view: self.camera.view,
                model: self.camera.model,
                perspective: self.camera.perspective,
            };
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };

            if entity.visible {
                let vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::new(&ctx.backend.display, &entity.vertices).unwrap();
                if let Some(triangles) = entity.triangles.as_ref() {
                    let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(&ctx.backend.display, glium::index::PrimitiveType::TrianglesList, triangles).unwrap();
                    // match entity.normals.as_ref() {
                    //     Some(normals) => {
                    //         let nb:  glium::VertexBuffer<Normal> = glium::VertexBuffer::new(display, normals).unwrap();
                    //         target.draw((&vb, &nb), &ib, ctx.resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
                    //     },
                    //     None => {}
                    // };
                    // if entity.pickable {
                    //     if let Some(picking_target) = picking_target_opt.as_mut() {
                    //         picking_target.draw(&vb, &ib, ctx.resource_pack.get_shader(&resource::ShaderId::Picking), &uniforms, &params).unwrap();
                    //     }
                    // }
                }
                if let Some(lines) = entity.lines.as_ref() {
                    let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(&ctx.backend.display, glium::index::PrimitiveType::LinesList, lines).unwrap();
                    target.draw(&vb, &ib, &shader.program, &uniforms, &params).unwrap();
                }

                if let Some(mesh_id) = entity.mesh_id.as_ref() {
                    target.draw((&ctx.resource_pack.get_mesh(mesh_id).vertices, &ctx.resource_pack.get_mesh(mesh_id).normals), &ctx.resource_pack.get_mesh(mesh_id).indices,  &shader.program, &uniforms, &params).unwrap();
                }
            }
        }

        target.finish().unwrap();
    }

    fn iter_entities(&self) -> std::collections::hash_map::Values<u16, Entity> {
        self.entities.values()
    }

    // pub fn hide_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = false;
    // }
    //
    // pub fn show_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = true;
    // }
}
