use glium::{uniform, Surface};
use hashbrown::{HashMap, hash_map::Iter};
use cgmath::num_traits::identities::One;
use cgmath::Matrix4;
use std::convert::TryInto;
use crate::{
    camera::Camera,
    context::Context,
    geometry::{Geometry, Shape},
    mesh::{Vertex, Normal, PrimitiveType},
    resource::{ShaderId, TextureId},
    view::{View, ViewBuilder, Renderable},
    color::Color,
    picking::Picker,
};

// TODO: remove unwraps

pub type Entity = u16;

// struct RenderData {
//     pub vertices: Vec<Vertex>,
//     pub normals:  Option<Vec<Normal>>,
//     pub indices: Vec<u32>,
//     pub primitive: glium::index::PrimitiveType,
// }

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum NodeId {
    Root,
    Camera(u32),
    Renderable(u32),
}

enum NodeKind {
    Group,
    Camera,
    Renderable,
}

#[allow(dead_code)]
struct Node {
    dirty: bool,
    kind: NodeKind,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

struct CacheEntry {
    transform: Matrix4<f32>,
    vertices: Vec<Vertex>,
    normals: Option<Vec<Normal>>,
    indices: Vec<u32>,
}

struct Cache {
    entries: HashMap<(ShaderId, TextureId, PrimitiveType, bool), CacheEntry>, // add Transform
}

impl Cache {
    pub fn new() -> Cache {
        Default::default()
    }

    pub fn iter<'a>(&'a self) -> Iter<(ShaderId, TextureId, PrimitiveType, bool), CacheEntry> {
        self.entries.iter()
    }

    pub fn add(&mut self, context: &Context, transform: cgmath::Matrix4<f32>, geometry: &Geometry) {
        let mesh = match &geometry.shape {
            Shape::Mesh(mesh) => {
                mesh
            },
            Shape::Model(model_id) => {
                &context.resource_pack.get_model(&model_id).mesh
            }
        };
        match self.entries.get_mut(&(geometry.shader_id, geometry.texture_id, mesh.primitive, mesh.normals.is_some())) {
            Some(entry) => {
                let offset: u32 = entry.vertices.len().try_into().unwrap();
                entry.vertices.extend(&mesh.vertices);
                entry.indices.extend(&mesh.indices.iter().map(|&i| i+offset).collect::<Vec<u32>>());
                if let Some(normals) = &mesh.normals {
                    entry.normals.as_mut().unwrap().extend(normals);
                }
            },
            None => {
                self.entries.insert(
                    (geometry.shader_id, geometry.texture_id, mesh.primitive, mesh.normals.is_some()),
                    CacheEntry {
                        transform,
                        normals: match &mesh.normals {
                            Some(normals) => Some(normals.clone()),
                            None => None
                        },
                        vertices: mesh.vertices.clone(),
                        indices: mesh.indices.clone(),
                    }
                );
            }
        }
    }

}

impl Default for Cache {
    fn default() -> Cache {
        Cache {
            entries: HashMap::new(),
        }
    }
}

pub struct Scene {
    last_id: u32,
    nodes: HashMap<NodeId, Node>,
    cameras: HashMap<NodeId, Camera>,
    renderables: HashMap<NodeId, Box<dyn Renderable>>,
    views: HashMap<NodeId, View>,
    cache: Cache,
    clear_color: Color,
}

impl Node {
    pub fn with_kind_and_parent(kind: NodeKind, parent: Option<NodeId>) -> Node {
        Node {
            parent,
            dirty: true,
            kind,
            children: Vec::new(),
        }
    }
}

impl Default for Scene {
    fn default() -> Scene {
        let mut nodes = HashMap::new();
        nodes.insert(NodeId::Root, Node::with_kind_and_parent(NodeKind::Group, None));
        Scene {
            last_id: 0,
            nodes,
            cache: Cache::new(),
            cameras: HashMap::new(),
            renderables: HashMap::new(),
            views: HashMap::new(),
            clear_color: Color::from_rgb(12, 12, 12),
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Default::default()
    }

    #[inline]
    fn next_id(&mut self) -> u32 {
        self.last_id = self.last_id.checked_add(1).unwrap();
        self.last_id
    }

    pub fn add_renderable(&mut self, parent_id: NodeId, renderable: Box<dyn Renderable>) -> Option<NodeId> {
        let id = self.next_id();
        let node_id = NodeId::Renderable(id);
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.children.push(node_id);
                self.nodes.insert(node_id, Node::with_kind_and_parent(NodeKind::Renderable, Some(parent_id)));
                self.views.insert(node_id, renderable.render(ViewBuilder::with_id(id)));
                self.renderables.insert(node_id, renderable);
                Some(node_id)
            },
            None => None,
        }
    }

    pub fn add_camera(&mut self, parent_id: NodeId, camera: Camera) -> Option<NodeId> {
        let id = NodeId::Camera(self.next_id());
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.children.push(id);
                self.nodes.insert(id, Node::with_kind_and_parent(NodeKind::Camera, Some(parent_id)));
                self.cameras.insert(id, camera);
                Some(id)
            },
            None => None,
        }
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn render(&mut self, ctx: &mut Context) {
        let mut target = ctx.backend.display.draw();

        let mut picking_target_opt = ctx.picker.target(&ctx.backend.display);
        target.clear_color_and_depth(self.clear_color.into(), 1.0);
        ctx.backend.glyph_brush.draw_queued(&ctx.backend.display, &mut target);

        if self.nodes[&NodeId::Root].dirty {
            self.cache = Cache::new();
            let mut stack: Vec<NodeId> = vec![NodeId::Root];
            let mut transforms: Vec<cgmath::Matrix4<f32>> = vec![cgmath::Matrix4::one()];

            while !stack.is_empty() {
                let node_id = stack.pop().unwrap();
                let node = &self.nodes.get(&node_id).unwrap();
                let mut transform = transforms.pop().unwrap();

                match node.kind {
                    NodeKind::Camera => {
                        transform = transform * self.cameras.get(&node_id).unwrap().matrix();
                    },
                    NodeKind::Renderable => {
                        let view = self.views.get(&node_id).unwrap();
                        for geometry in view.geometries.iter() {
                            self.cache.add(
                                ctx,
                                transform * geometry.transform,
                                &geometry,
                            );
                        }
                    },
                    NodeKind::Group => {},
                }

                for child in node.children.iter() {
                    stack.push(*child);
                    transforms.push(transform);
                }
            }
        }

        for ((shader_id, texture_id, primitive, _has_normals), entry) in self.cache.iter() {
            let uniforms = glium::uniform! {
                u_light: [-1.0, 0.4, 0.9f32],
                tex: &ctx.resource_pack.get_texture(&texture_id).texture,
                transform: cgmath::conv::array4x4(entry.transform),
            };
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            let shader = ctx.resource_pack.get_shader(&shader_id);
            let vb: glium::VertexBuffer<Vertex> =
            glium::VertexBuffer::new(&ctx.backend.display, &entry.vertices)
            .unwrap();
            let nb: Option<glium::VertexBuffer<Normal>> = match &entry.normals {
                Some(normals) => Some(glium::VertexBuffer::new(&ctx.backend.display, &normals)
                .unwrap()),
                None => None,
            };
            let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
                &ctx.backend.display,
                primitive.into(),
                &entry.indices,
            )
            .unwrap();
            if let Some(picking_target) = picking_target_opt.as_mut() {
                picking_target.draw(&vb, &ib, &ctx.resource_pack.get_shader(&ShaderId("picking")).program, &uniforms, &params).unwrap();
            }
            match nb {
                Some(normals) => {
                    target
                    .draw((&vb, &normals), &ib, &shader.program, &uniforms, &params)
                    .unwrap();
                },
                None => {
                    target
                    .draw(&vb, &ib, &shader.program, &uniforms, &params)
                    .unwrap();
                }
            }
        }

        ctx.picker.commit(ctx.mouse.position);
        target.finish().unwrap();
    }
}
