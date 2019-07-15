use glium::{uniform, Surface};
use std::collections::HashMap;

use crate::{
    camera::Camera,
    context::Context,
    geometry::{Geometry, Shape},
    mesh::Vertex,
    resource,
    resource::ShaderId,
    view::View,
};

// TODO: remove unwraps

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);

pub type Entity = u16;

// struct RenderData {
//     pub vertices: Vec<Vertex>,
//     pub normals:  Option<Vec<Normal>>,
//     pub indices: Vec<u32>,
//     pub primitive: glium::index::PrimitiveType,
// }
//
// enum Data {
//     Camera(Camera),
//     View(Box<dyn View>),
// }
//
// struct Node {
//     data: Data,
//     children: Vec<Node>,
// }
//
// impl Node {
//     fn camera(camera: Camera) -> Node {
//         Node {
//             data: Data::Camera(camera),
//             children: Vec::new(),
//         }
//     }
//
//     fn view(view: Box<dyn View>) -> Node {
//         Node {
//             data: Data::View(view),
//             children: Vec::new(),
//         }
//     }
// }

pub struct Scene {
    next_id: u16,
    views: HashMap<Entity, Box<dyn View>>,
    data: HashMap<Entity, Vec<Geometry>>,
    pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            next_id: 1,
            views: HashMap::new(),
            data: HashMap::new(),
            camera: Camera::isometric(1280.0 / 720.0),
        }
    }
}

impl Scene {
    pub fn add(&mut self, view: Box<dyn View>) -> Entity {
        let id = self.generate_id();
        self.data.insert(id, view.render(id));
        self.views.insert(id, view);
        id
    }

    fn generate_id(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).unwrap();
        id
    }

    pub fn render(&self, ctx: &Context) {
        let mut target = ctx.backend.display.draw();
        // let mut picking_target_opt = picker.target(&display);
        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        for (_entity, geometries) in self.data.iter() {
            for geometry in geometries.iter() {
                if geometry.visible {
                    let uniforms = glium::uniform! {
                        u_light: [-1.0, 0.4, 0.9f32],
                        tex: match &geometry.texture_id {
                            Some(tex) => &ctx.resource_pack.get_texture(&tex).texture,
                            None => &ctx.resource_pack.get_texture(&resource::TextureId("default")).texture,
                        },
                        view: self.camera.view,
                        model: self.camera.model,
                        perspective: self.camera.perspective,
                    };
                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    let shader = match geometry.shader_id.as_ref() {
                        Some(shader_id) => ctx.resource_pack.get_shader(shader_id),
                        None => ctx.resource_pack.get_shader(&ShaderId("default")),
                    };
                    match &geometry.shape {
                        Shape::Data(mesh) => {
                            let vb: glium::VertexBuffer<Vertex> =
                                glium::VertexBuffer::new(&ctx.backend.display, &mesh.vertices)
                                    .unwrap();
                            let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
                                &ctx.backend.display,
                                mesh.primitive,
                                &mesh.indices,
                            )
                            .unwrap();
                            target
                                .draw(&vb, &ib, &shader.program, &uniforms, &params)
                                .unwrap();
                        }
                        Shape::Mesh(mesh_id) => {
                            let vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::new(
                                &ctx.backend.display,
                                &ctx.resource_pack.get_mesh(mesh_id).vertices,
                            )
                            .unwrap();
                            let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
                                &ctx.backend.display,
                                ctx.resource_pack.get_mesh(mesh_id).primitive,
                                &ctx.resource_pack.get_mesh(mesh_id).indices,
                            )
                            .unwrap();
                            target
                                .draw(&vb, &ib, &shader.program, &uniforms, &params)
                                .unwrap();
                        }
                    }
                }
            }
        }
        //     let uniforms = glium::uniform! {
        //         // tex: match &entity.texture_id {
        //         //     Some(tex) => ctx.resource_pack.get_texture(&tex),
        //         //     None => ctx.resource_pack.get_texture(&resource::TextureId::Terrain),
        //         // },
        //         u_light: [-1.0, 0.4, 0.9f32],
        //         view: self.camera.view,
        //         model: self.camera.model,
        //         perspective: self.camera.perspective,
        //     };
        //     let params = glium::DrawParameters {
        //         depth: glium::Depth {
        //             test: glium::DepthTest::IfLess,
        //             write: true,
        //             .. Default::default()
        //         },
        //         .. Default::default()
        //     };
        //
        //     if entity.visible {
        //         let vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::new(&ctx.backend.display, &entity.vertices).unwrap();
        //         if let Some(triangles) = entity.triangles.as_ref() {
        //             let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(&ctx.backend.display, glium::index::PrimitiveType::TrianglesList, triangles).unwrap();
        //             // match entity.normals.as_ref() {
        //             //     Some(normals) => {
        //             //         let nb:  glium::VertexBuffer<Normal> = glium::VertexBuffer::new(display, normals).unwrap();
        //             //         target.draw((&vb, &nb), &ib, ctx.resource_pack.get_shader(&resource::ShaderId::Terrain), &uniforms, &params).unwrap();
        //             //     },
        //             //     None => {}
        //             // };
        //             // if entity.pickable {
        //             //     if let Some(picking_target) = picking_target_opt.as_mut() {
        //             //         picking_target.draw(&vb, &ib, ctx.resource_pack.get_shader(&resource::ShaderId::Picking), &uniforms, &params).unwrap();
        //             //     }
        //             // }
        //         }
        //         if let Some(lines) = entity.lines.as_ref() {
        //             let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(&ctx.backend.display, glium::index::PrimitiveType::LinesList, lines).unwrap();
        //             target.draw(&vb, &ib, &shader.program, &uniforms, &params).unwrap();
        //         }
        //
        //         if let Some(mesh_id) = entity.mesh_id.as_ref() {
        //             target.draw((&ctx.resource_pack.get_mesh(mesh_id).vertices, &ctx.resource_pack.get_mesh(mesh_id).normals), &ctx.resource_pack.get_mesh(mesh_id).indices,  &shader.program, &uniforms, &params).unwrap();
        //         }
        //     }
        // }

        target.finish().unwrap();
    }

    // pub fn hide_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = false;
    // }
    //
    // pub fn show_entity(&mut self, entity_id: EntityId) {
    //     self.entities.get_mut(&entity_id).unwrap().visible = true;
    // }
}
