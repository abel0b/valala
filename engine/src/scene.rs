use glium::{uniform, Surface};
use std::collections::HashMap;
use cgmath::num_traits::identities::One;
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

enum Data {
    Camera(Camera),
    View(Box<dyn View>),
}

pub struct Node {
    data: Data,
    children: Vec<Node>,
}

impl Node {
    pub fn append(&mut self, node: Node) -> &mut Node {
        self.children.push(node);
        self.children.last_mut().unwrap()
    }

    pub fn with_camera(camera: Camera) -> Node {
        Node {
            data: Data::Camera(camera),
            children: Vec::new(),
        }
    }

    pub fn with_view(view: Box<dyn View>) -> Node {
        Node {
            data: Data::View(view),
            children: Vec::new(),
        }
    }
}

pub struct Scene {
    next_id: u16,
    // views: HashMap<Entity, Box<dyn View>>,
    // data: HashMap<Entity, Vec<Geometry>>,
    root: Option<Node>,
    // pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            next_id: 1,
            // views: HashMap::new(),
            // data: HashMap::new(),
            root: None,
            // camera: Camera::isometric(1280.0 / 720.0),
        }
    }
}

impl Scene {
    pub fn append(&mut self, node: Node) -> &mut Node {
        let id = self.generate_id();
        match self.root.as_mut() {
            Some(root) => {
                root.append(node);
            },
            None => {
                self.root = Some(node);
            }
        }
        // self.data.insert(id, view.render(id));
        // self.views.insert(id, view);
        self.root.as_mut().unwrap()
    }

    fn generate_id(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).unwrap();
        id
    }

    pub fn render(&mut self, ctx: &Context) {
        let mut target = ctx.backend.display.draw();
        // let mut picking_target_opt = picker.target(&display);
        target.clear_color_and_depth(CLEAR_COLOR, 1.0);

        if let Some(root) = self.root.as_mut() {
            let mut transforms: Vec<cgmath::Matrix4<f32>> = vec![cgmath::Matrix4::one()];
            let mut stack: Vec<&mut Node> = vec![root];

            while !stack.is_empty() {
                let node = stack.pop().unwrap();
                let mut camera = transforms.pop().unwrap();

                match &node.data {
                    Data::Camera(local_camera) => {
                        camera = camera * local_camera.matrix();
                    },
                    Data::View(view) => {
                        println!("view");
                    },
                }

                for child in node.children.iter_mut() {
                    stack.push(child);
                    transforms.push(camera);
                }
            }
        }


        // for (_entity, geometries) in self.data.iter() {
        //     for geometry in geometries.iter() {
        //         if geometry.visible {
        //             let uniforms = glium::uniform! {
        //                 u_light: [-1.0, 0.4, 0.9f32],
        //                 tex: match &geometry.texture_id {
        //                     Some(tex) => &ctx.resource_pack.get_texture(&tex).texture,
        //                     None => &ctx.resource_pack.get_texture(&resource::TextureId("default")).texture,
        //                 },
        //                 view: self.camera.view,
        //                 model: self.camera.model,
        //                 perspective: self.camera.perspective,
        //             };
        //             let params = glium::DrawParameters {
        //                 depth: glium::Depth {
        //                     test: glium::DepthTest::IfLess,
        //                     write: true,
        //                     ..Default::default()
        //                 },
        //                 ..Default::default()
        //             };
        //             let shader = match geometry.shader_id.as_ref() {
        //                 Some(shader_id) => ctx.resource_pack.get_shader(shader_id),
        //                 None => ctx.resource_pack.get_shader(&ShaderId("default")),
        //             };
        //             match &geometry.shape {
        //                 Shape::Mesh(mesh) => {
        //                     let vb: glium::VertexBuffer<Vertex> =
        //                         glium::VertexBuffer::new(&ctx.backend.display, &mesh.vertices)
        //                             .unwrap();
        //                     let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
        //                         &ctx.backend.display,
        //                         mesh.primitive,
        //                         &mesh.indices,
        //                     )
        //                     .unwrap();
        //                     target
        //                         .draw(&vb, &ib, &shader.program, &uniforms, &params)
        //                         .unwrap();
        //                 }
        //                 Shape::Model(model_id) => {
        //                     let vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::new(
        //                         &ctx.backend.display,
        //                         &ctx.resource_pack.get_model(model_id).mesh.vertices,
        //                     )
        //                     .unwrap();
        //                     let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
        //                         &ctx.backend.display,
        //                         ctx.resource_pack.get_model(model_id).mesh.primitive,
        //                         &ctx.resource_pack.get_model(model_id).mesh.indices,
        //                     )
        //                     .unwrap();
        //                     target
        //                         .draw(&vb, &ib, &shader.program, &uniforms, &params)
        //                         .unwrap();
        //                 }
        //             }
        //         }
        //     }
        // }

        target.finish().unwrap();
    }
}
