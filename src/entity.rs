use glium::{VertexBuffer, IndexBuffer};
use crate::mesh::{Vertex, Normal};
use std::collections::HashMap;
use crate::scene::Scene;
use crate::resource::{MeshId, TextureId};

pub type EntityId = u16;

pub struct Entity {
    pub id: EntityId,
    pub vertices: VertexBuffer<Vertex>,
    pub normals:  Option<VertexBuffer<Normal>>,
    pub triangles: Option<IndexBuffer<u32>>,
    pub lines: Option<IndexBuffer<u32>>,
    pub visible: bool,
    pub pickable: bool,
    pub mesh_id: Option<MeshId>,
    pub texture_id: Option<TextureId>,
    // pub line_shader: ShaderId,
    // pub triangle_shader: ShaderId,
}

pub struct EntityFactory {
    entity_id: u16,
    group_id: u16,
    vertices: Vec<Vertex>,
    normals: Option<Vec<Normal>>,
    triangles: Option<Vec<u32>>,
    lines: Option<Vec<u32>>,
    visible: bool,
    pickable: bool,
    pub mesh_id: Option<MeshId>,
    texture_id: Option<TextureId>,
}

impl EntityFactory {
    pub fn new(scene: &mut Scene) -> EntityFactory {
        EntityFactory {
            entity_id: scene.generate_id(),
            group_id: 1,
            vertices: Vec::new(),
            triangles: None,
            lines: None,
            normals: None,
            visible: false,
            pickable: false,
            mesh_id: None,
            texture_id: None,
        }
    }

    pub fn vertex(mut self, position: (f32, f32, f32), tex_coords: (f32, f32), data: (f32, f32, f32)) -> EntityFactory {
        let entity_id = self.entity_id.to_be_bytes();
        let group_id = self.group_id.to_be_bytes();
        let id = u32::from_be_bytes([entity_id[0], entity_id[1], group_id[0], group_id[1]]);

        self.vertices.push(
            Vertex {
                id,
                color: (0.0, 0.0, 0.0, 0.0),
                data: (data.0, data.1, data.2, 0.0),
                position,
                tex_coords,
            }
        );
        self
    }


    pub fn group(mut self) -> EntityFactory {
        self.group_id = self.group_id.checked_add(1).unwrap();
        self
    }

    pub fn texture(mut self, texture_id: TextureId) -> EntityFactory {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn triangle(mut self, a: u32, b: u32, c: u32) -> EntityFactory {
        match self.triangles.as_mut() {
            Some(triangles) => {
                triangles.extend_from_slice(&[a, b, c]);
            },
            None => {
                self.triangles = Some(vec![a, b, c]);
            },
        };
        self
    }

    pub fn normal(mut self, (x, y, z): (f32, f32, f32)) -> EntityFactory {
        let normal = Normal { normal: (x, y, z) };
        match self.normals.as_mut() {
            Some(normals) => {
                normals.push(normal);
            },
            None => {
                self.normals = Some(vec![normal]);
            },
        };
        self
    }

    pub fn line(mut self, a: u32, b: u32) -> EntityFactory {
        match self.lines.as_mut() {
            Some(lines) => {
                lines.extend_from_slice(&[a, b]);
            },
            None => {
                self.lines = Some(vec![a, b]);
            },
        };
        self
    }

    pub fn visible(mut self) -> EntityFactory{
        self.visible = true;
        self
    }

    pub fn pickable(mut self) -> EntityFactory{
        self.pickable = true;
        self
    }

    pub fn mesh(mut self, mesh_id: MeshId) -> EntityFactory {
        self.mesh_id = Some(mesh_id);
        self
    }

    pub fn build(self, display: &glium::Display) -> Entity {
        Entity {
            id: self.entity_id,
            texture_id: self.texture_id,
            mesh_id: self.mesh_id,
            vertices: glium::VertexBuffer::new(display, &self.vertices).unwrap(),
            normals: match self.normals {
                Some(normals) => Some(glium::VertexBuffer::new(display, &normals).unwrap()),
                None => None,
            },
            triangles: match self.triangles {
                Some(triangles) => Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &triangles).unwrap()),
                None => None,
            },
            lines: match self.lines {
                Some(lines) => Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &lines).unwrap()),
                None => None,
            },
            visible: self.visible,
            pickable: self.pickable,
        }
    }
}
