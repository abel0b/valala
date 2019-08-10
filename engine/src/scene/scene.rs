use crate::stage::Transition;
use crate::{
    color::Color,
    mesh::{Normal, Vertex},
    picking::PickingEvent,
    resource::{ShaderId},
    scene::Camera,
    scene::Entity,
    scene::Uid,
    scene::{Cache, Node, NodeIndex},
    store::{Store, World},
};
use glium::{uniform, Surface};
use hashbrown::HashMap;
use nalgebra::Matrix4;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SceneError {
    NodeNotFound,
}

type SceneResult<R> = Result<R, SceneError>;

pub struct Scene<W: World> {
    last_id: u32,
    nodes: HashMap<NodeIndex, Node>,
    cameras: HashMap<Uid, Camera>,
    entities: HashMap<Uid, Entity<W>>,
    cache: Cache,
    clear_color: Color,
}

impl<W> Default for Scene<W>
where
    W: World,
{
    fn default() -> Scene<W> {
        let mut nodes = HashMap::new();
        nodes.insert(
            NodeIndex::Root,
            Node::with_index_and_parent(NodeIndex::Root, None),
        );
        Scene {
            last_id: 0,
            nodes,
            cameras: HashMap::new(),
            entities: HashMap::new(),
            cache: Cache::new(),
            clear_color: Color::from_rgb(12, 12, 12),
        }
    }
}

impl<W> Scene<W>
where
    W: World,
{
    pub fn new() -> Scene<W> {
        Default::default()
    }

    #[inline]
    fn next_id(&mut self) -> u32 {
        self.last_id = self.last_id.checked_add(1).unwrap();
        self.last_id
    }

    pub fn add_entity(&mut self, parent_id: NodeIndex, entity: Entity<W>) -> SceneResult<Uid> {
        let id = self.next_id();
        let node_id = NodeIndex::Entity(Uid(id));
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.add_child(node_id);
                self.nodes.insert(
                    node_id,
                    Node::with_index_and_parent(node_id, Some(parent_id)),
                );
                self.entities.insert(Uid(id), entity);
                Ok(Uid(id))
            }
            None => Err(SceneError::NodeNotFound),
        }
    }

    pub fn add_camera(&mut self, parent_id: NodeIndex, camera: Camera) -> SceneResult<Uid> {
        let id = self.next_id();
        let node_id = NodeIndex::Camera(Uid(id));
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.add_child(node_id);
                self.nodes.insert(
                    node_id,
                    Node::with_index_and_parent(node_id, Some(parent_id)),
                );
                self.cameras.insert(Uid(id), camera);
                Ok(Uid(id))
            }
            None => Err(SceneError::NodeNotFound),
        }
    }

    pub fn add_group(&mut self, parent_id: NodeIndex) -> SceneResult<Uid> {
        let id = self.next_id();
        let node_id = NodeIndex::Group(Uid(id));
        match self.nodes.get_mut(&parent_id) {
            Some(parent) => {
                parent.add_child(node_id);
                self.nodes.insert(
                    node_id,
                    Node::with_index_and_parent(node_id, Some(parent_id)),
                );
                Ok(Uid(id))
            }
            None => Err(SceneError::NodeNotFound),
        }
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn render(&mut self, store: &mut Store<W>) {
        let mut target = store.context.backend.display.draw();

        for event in store.context.picker.update().iter() {
            match event {
                PickingEvent::HoverEnter(uid) => {
                    if let Some(Entity {
                        on_hover_enter: Some(on_hover_enter),
                        ..
                    }) = self.entities.get(uid)
                    {
                        let action = on_hover_enter(*uid);
                        store.dispatch(self, action);
                    }
                }
                PickingEvent::HoverLeave(uid) => {
                    if let Some(Entity {
                        on_hover_leave: Some(on_hover_leave),
                        ..
                    }) = self.entities.get(uid)
                    {
                        let action = on_hover_leave(*uid);
                        store.dispatch(self, action);
                    }
                }
                PickingEvent::MouseUp(_uid) => {}
                PickingEvent::MouseDown(_uid) => {}
            }
        }

        if self.nodes[&NodeIndex::Root].dirty {
            self.cache = Cache::new();
            let mut stack: Vec<NodeIndex> = vec![NodeIndex::Root];
            let mut transforms: Vec<Matrix4<f32>> = vec![Matrix4::identity()];

            while !stack.is_empty() {
                let node_id = stack.pop().unwrap();
                let node = self.nodes.get_mut(&node_id).unwrap();
                let mut transform = transforms.pop().unwrap();

                if !node.dirty || !node.visible {
                    break;
                }

                match node.index {
                    NodeIndex::Camera(uid) => {
                        transform = transform * self.cameras.get(&uid).unwrap().matrix();
                    }
                    NodeIndex::Entity(uid) => {
                        self.cache
                            .update(store, uid, self.entities.get(&uid).unwrap(), transform);
                    }
                    NodeIndex::Group(_uid) => {}
                    NodeIndex::Root => {}
                }

                // node.dirty = false;

                for child in node.children.iter() {
                    stack.push(*child);
                    transforms.push(transform);
                }
            }
        }

        let mut picking_target_opt = store.context.picker.target(&store.context.backend.display);
        target.clear_color_and_depth(self.clear_color.into(), 1.0);
        store
            .context
            .backend
            .glyph_brush
            .draw_queued(&store.context.backend.display, &mut target);

        for ((shader_id, texture_id, primitive, _has_normals), entry) in self.cache.iter() {
            let transform: [[f32; 4]; 4] = entry.transform.into();
            let uniforms = glium::uniform! {
                u_light: [-1.0, 0.4, 0.9f32],
                tex: &store.context.resource_pack.get_texture(&texture_id).texture,
                transform: transform,
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

    pub fn handle(&mut self, store: &mut Store<W>, event: &glium::glutin::Event) -> Transition<W> {
        let mut action = Transition::Continue;
        match event {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                glium::glutin::WindowEvent::CloseRequested => action = Transition::Quit,
                glium::glutin::WindowEvent::Resized(glium::glutin::dpi::LogicalSize {
                    width,
                    height,
                }) => {
                    store.context.picker.initialize_picking_attachments(
                        &store.context.backend.display,
                        (*width as u32, *height as u32),
                    );
                    // self.stage_machine
                    //     .scene()
                    //     .camera
                    //     .scale((height / width) as f32);
                }
                // glium::glutin::WindowEvent::MouseWheel { delta, .. } => {
                //     if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                //         self.stage_machine.scene().camera.zoom(*y);
                //     }
                // },
                glium::glutin::WindowEvent::CursorMoved { position, .. } => {
                    store.context.mouse.position = Some((position.x as i32, position.y as i32));
                }
                glium::glutin::WindowEvent::MouseInput { state, .. } => {
                    let mouse_action = if let Some(uid) = store.context.picker.entity {
                        match state {
                            glium::glutin::ElementState::Pressed => {
                                if let Some(Entity {
                                    on_mouse_down: Some(on_mouse_down),
                                    ..
                                }) = self.entities.get(&uid)
                                {
                                    Some(on_mouse_down(uid))
                                } else {
                                    None
                                }
                            }
                            glium::glutin::ElementState::Released => {
                                if let Some(Entity {
                                    on_mouse_up: Some(on_mouse_up),
                                    ..
                                }) = self.entities.get(&uid)
                                {
                                    Some(on_mouse_up(uid))
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    };
                    if let Some(mouse_action) = mouse_action {
                        store.dispatch(self, mouse_action);
                    }
                }
                _ => (),
            },
            glium::glutin::Event::DeviceEvent { .. } => (),
            _ => (),
        }
        action
    }
}
