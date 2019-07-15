use crate::{
    mesh::{Mesh, Normal, Vertex},
    resource::{MeshId, ShaderId, TextureId},
};

pub enum Shape {
    Mesh(MeshId),
    Data(Mesh),
}

pub struct Geometry {
    pub visible: bool,
    pub pickable: bool,
    pub shader_id: Option<ShaderId>,
    pub texture_id: Option<TextureId>,
    pub shape: Shape,
}

pub struct GeometryBuilder {
    visible: bool,
    pickable: bool,
    shader_id: Option<ShaderId>,
    texture_id: Option<TextureId>,
    mesh_id: Option<MeshId>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    normals: Option<Vec<Normal>>,
    primitive: glium::index::PrimitiveType,
}

impl Default for GeometryBuilder {
    fn default() -> GeometryBuilder {
        GeometryBuilder {
            visible: true,
            pickable: false,
            shader_id: None,
            texture_id: None,
            mesh_id: None,
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: None,
            primitive: glium::index::PrimitiveType::TrianglesList,
        }
    }
}

impl GeometryBuilder {
    pub fn shader(&mut self, shader: ShaderId) -> &mut GeometryBuilder {
        self.shader_id = Some(shader);
        self
    }

    pub fn vertex(
        &mut self,
        position: (f32, f32, f32),
        tex_coords: (f32, f32),
        data: (f32, f32, f32),
    ) -> &mut GeometryBuilder {
        // let entity_id = self.entity_id.to_be_bytes();
        // let group_id = self.group_id.to_be_bytes();
        // let id = u32::from_be_bytes([entity_id[0], entity_id[1], group_id[0], group_id[1]]);

        self.vertices.push(Vertex {
            id: 0,
            color: (0.0, 0.0, 0.0, 0.0),
            data: (data.0, data.1, data.2, 0.0),
            position,
            tex_coords,
        });
        self
    }

    // pub fn group(mut self) -> EntityFactory {
    //     self.group_id = self.group_id.checked_add(1).unwrap();
    //     self
    // }

    pub fn texture(&mut self, texture_id: TextureId) -> &mut GeometryBuilder {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn triangle(&mut self, a: u32, b: u32, c: u32) -> &mut GeometryBuilder {
        self.primitive = glium::index::PrimitiveType::TrianglesList;
        self.indices.extend_from_slice(&[a, b, c]);
        self
    }
    pub fn line(&mut self, a: u32, b: u32) -> &mut GeometryBuilder {
        self.primitive = glium::index::PrimitiveType::LinesList;
        self.indices.extend_from_slice(&[a, b]);
        self
    }

    pub fn normal(&mut self, (x, y, z): (f32, f32, f32)) -> &mut GeometryBuilder {
        let normal = Normal { normal: (x, y, z) };
        match self.normals.as_mut() {
            Some(normals) => {
                normals.push(normal);
            }
            None => {
                self.normals = Some(vec![normal]);
            }
        };
        self
    }

    pub fn visible(&mut self) -> &mut GeometryBuilder {
        self.visible = true;
        self
    }

    pub fn pickable(&mut self) -> &mut GeometryBuilder {
        self.pickable = true;
        self
    }

    pub fn mesh(&mut self, mesh_id: MeshId) -> &mut GeometryBuilder {
        self.mesh_id = Some(mesh_id);
        self
    }

    pub fn build(self) -> Geometry {
        Geometry {
            visible: self.visible,
            pickable: self.pickable,
            shader_id: self.shader_id,
            texture_id: self.texture_id,
            shape: match self.mesh_id {
                Some(mesh_id) => Shape::Mesh(mesh_id),
                None => Shape::Data(Mesh {
                    vertices: self.vertices,
                    indices: self.indices,
                    normals: self.normals,
                    primitive: self.primitive,
                }),
            },
        }
    }
}
