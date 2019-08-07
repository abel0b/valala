use crate::{
    camera::Camera,
    color::Color,
    geometry::{Geometry, Shape},
    mesh::{Normal, PrimitiveType, Vertex},
    picking::PickingEvent,
    resource::{ShaderId, TextureId},
    store::Store,
    view::{Hoverable, Renderable, View},
};
use cgmath::num_traits::identities::One;
use cgmath::Matrix4;
use glium::{uniform, Surface};
use hashbrown::{hash_map::Iter, HashMap};
use std::convert::TryInto;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum NodeId {
    Root,
    Camera(u32),
    Entity(u32),
    Group(u32),
}

enum NodeKind {
    Group,
    Camera,
    Entity,
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

    pub fn iter(&self) -> Iter<(ShaderId, TextureId, PrimitiveType, bool), CacheEntry> {
        self.entries.iter()
    }

    pub fn add<S, A>(
        &mut self,
        store: &Store<S, A>,
        transform: cgmath::Matrix4<f32>,
        geometry: &Geometry,
    ) {
        let mesh = match &geometry.shape {
            Shape::Mesh(mesh) => mesh,
            Shape::Model(model_id) => &store.context.resource_pack.get_model(&model_id).mesh,
        };
        match self.entries.get_mut(&(
            geometry.shader_id,
            geometry.texture_id,
            mesh.primitive,
            mesh.normals.is_some(),
        )) {
            Some(entry) => {
                let offset: u32 = entry.vertices.len().try_into().unwrap();
                entry.vertices.extend(&mesh.vertices);
                entry.indices.extend(
                    &mesh
                        .indices
                        .iter()
                        .map(|&i| i + offset)
                        .collect::<Vec<u32>>(),
                );
                if let Some(normals) = &mesh.normals {
                    entry.normals.as_mut().unwrap().extend(normals);
                }
            }
            None => {
                self.entries.insert(
                    (
                        geometry.shader_id,
                        geometry.texture_id,
                        mesh.primitive,
                        mesh.normals.is_some(),
                    ),
                    CacheEntry {
                        transform,
                        normals: match &mesh.normals {
                            Some(normals) => Some(normals.clone()),
                            None => None,
                        },
                        vertices: mesh.vertices.clone(),
                        indices: mesh.indices.clone(),
                    },
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

pub struct Scene<S, A> {
    last_id: u32,
    nodes: HashMap<NodeId, Node>,
    cameras: HashMap<NodeId, Camera>,
    renderables: HashMap<NodeId, Rc<dyn Renderable<S, A>>>,
    hoverables: HashMap<NodeId, Rc<dyn Hoverable<A>>>,
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

impl<S, A> Default for Scene<S, A> {
    fn default() -> Scene<S, A> {
        let mut nodes = HashMap::new();
        nodes.insert(
            NodeId::Root,
            Node::with_kind_and_parent(NodeKind::Group, None),
        );
        Scene {
            last_id: 0,
            nodes,
            cache: Cache::new(),
            cameras: HashMap::new(),
            renderables: HashMap::new(),
            hoverables: HashMap::new(),
            views: HashMap::new(),
            clear_color: Color::from_rgb(12, 12, 12),
        }
    }
}

impl<S, A> Scene<S, A> {
    pub fn new() -> Scene<S, A> {
        Default::default()
    }

    #[inline]
    fn next_id(&mut self) -> u32 {
        self.last_id = self.last_id.checked_add(1).unwrap();
        self.last_id
    }

    pub fn add_entity(&mut self, parent_id: NodeId) -> Option<NodeId> {
        let id = self.next_id();
        let node_id = NodeId::Entity(id);
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.children.push(node_id);
                self.nodes.insert(
                    node_id,
                    Node::with_kind_and_parent(NodeKind::Entity, Some(parent_id)),
                );
                Some(node_id)
            }
            None => None,
        }
    }

    pub fn set_renderable(
        &mut self,
        node_id: NodeId,
        renderable: Rc<dyn Renderable<S, A>>,
    ) -> Option<NodeId> {
        match node_id {
            NodeId::Entity(_id) => match self.nodes.get_mut(&node_id) {
                Some(_) => {
                    self.renderables.insert(node_id, renderable);
                    Some(node_id)
                }
                None => None,
            },
            _ => None,
        }
    }

    pub fn set_hoverable(
        &mut self,
        node_id: NodeId,
        hoverable: Rc<dyn Hoverable<A>>,
    ) -> Option<NodeId> {
        match node_id {
            NodeId::Entity(_id) => match self.nodes.get_mut(&node_id) {
                Some(_) => {
                    self.hoverables.insert(node_id, hoverable);
                    Some(node_id)
                }
                None => None,
            },
            _ => None,
        }
    }

    pub fn add_camera(&mut self, parent_id: NodeId, camera: Camera) -> Option<NodeId> {
        let id = NodeId::Camera(self.next_id());
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.children.push(id);
                self.nodes.insert(
                    id,
                    Node::with_kind_and_parent(NodeKind::Camera, Some(parent_id)),
                );
                self.cameras.insert(id, camera);
                Some(id)
            }
            None => None,
        }
    }

    pub fn add_group(&mut self, parent_id: NodeId) -> Option<NodeId> {
        let id = NodeId::Group(self.next_id());
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.children.push(id);
                self.nodes.insert(
                    id,
                    Node::with_kind_and_parent(NodeKind::Group, Some(parent_id)),
                );
                Some(id)
            }
            None => None,
        }
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn render(&mut self, store: &mut Store<S, A>) {
        let mut target = store.context.backend.display.draw();

        for event in store.context.picker.update().iter() {
            match event {
                PickingEvent::HoverEnter(node_id) => {
                    let hoverable = self.hoverables.get(node_id).unwrap();
                    let action = hoverable.hover_enter(*node_id);
                    store.dispatch(self, action);
                }
                PickingEvent::HoverLeave(node_id) => {
                    let hoverable = self.hoverables.get(node_id).unwrap();
                    let action = hoverable.hover_leave(*node_id);
                    store.dispatch(self, action);
                }
                PickingEvent::MouseUp(_node_id) => {}
                PickingEvent::MouseDown(_node_id) => {}
            }
        }

        let mut picking_target_opt = store.context.picker.target(&store.context.backend.display);
        target.clear_color_and_depth(self.clear_color.into(), 1.0);
        store
            .context
            .backend
            .glyph_brush
            .draw_queued(&store.context.backend.display, &mut target);

        if self.nodes[&NodeId::Root].dirty {
            self.cache = Cache::new();
            let mut stack: Vec<NodeId> = vec![NodeId::Root];
            let mut transforms: Vec<cgmath::Matrix4<f32>> = vec![cgmath::Matrix4::one()];

            while !stack.is_empty() {
                let node_id = stack.pop().unwrap();
                let node: &Node = &self.nodes.get(&node_id).unwrap();
                let mut transform = transforms.pop().unwrap();

                match node.kind {
                    NodeKind::Camera => {
                        transform = transform * self.cameras.get(&node_id).unwrap().matrix();
                    }
                    NodeKind::Entity => {
                        if let Some(renderable) = self.renderables.get(&node_id) {
                            self.views
                                .insert(node_id, renderable.render(store, node_id));
                        }
                        if let Some(view) = self.views.get(&node_id) {
                            for geometry in view.geometries.iter() {
                                self.cache
                                    .add(store, transform * geometry.transform, &geometry);
                            }
                        }
                    }
                    NodeKind::Group => {}
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
                tex: &store.context.resource_pack.get_texture(&texture_id).texture,
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
            let shader = store.context.resource_pack.get_shader(&shader_id);
            let vb: glium::VertexBuffer<Vertex> =
                glium::VertexBuffer::new(&store.context.backend.display, &entry.vertices).unwrap();
            let nb: Option<glium::VertexBuffer<Normal>> = match &entry.normals {
                Some(normals) => Some(
                    glium::VertexBuffer::new(&store.context.backend.display, &normals).unwrap(),
                ),
                None => None,
            };
            let ib: glium::IndexBuffer<u32> = glium::IndexBuffer::new(
                &store.context.backend.display,
                primitive.into(),
                &entry.indices,
            )
            .unwrap();
            if let Some(picking_target) = picking_target_opt.as_mut() {
                picking_target
                    .draw(
                        &vb,
                        &ib,
                        &store
                            .context
                            .resource_pack
                            .get_shader(&ShaderId("picking"))
                            .program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }
            match nb {
                Some(normals) => {
                    target
                        .draw((&vb, &normals), &ib, &shader.program, &uniforms, &params)
                        .unwrap();
                }
                None => {
                    target
                        .draw(&vb, &ib, &shader.program, &uniforms, &params)
                        .unwrap();
                }
            }
        }

        store.context.picker.commit(store.context.mouse.position);
        target.finish().unwrap();
    }
}
