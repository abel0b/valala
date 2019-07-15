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

impl GeometryBuilder {
    pub fn new() -> GeometryBuilder {
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

    pub fn shader<'a>(&'a mut self, shader: ShaderId) -> &'a mut GeometryBuilder {
        self.shader_id = Some(shader);
        self
    }

    pub fn vertex<'a>(
        &'a mut self,
        position: (f32, f32, f32),
        tex_coords: (f32, f32),
        data: (f32, f32, f32),
    ) -> &'a mut GeometryBuilder {
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

    pub fn texture<'a>(&'a mut self, texture_id: TextureId) -> &'a mut GeometryBuilder {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn triangle<'a>(&'a mut self, a: u32, b: u32, c: u32) -> &'a mut GeometryBuilder {
        self.primitive = glium::index::PrimitiveType::TrianglesList;
        self.indices.extend_from_slice(&[a, b, c]);
        self
    }
    pub fn line<'a>(&'a mut self, a: u32, b: u32) -> &'a mut GeometryBuilder {
        self.primitive = glium::index::PrimitiveType::LinesList;
        self.indices.extend_from_slice(&[a, b]);
        self
    }

    pub fn normal<'a>(&'a mut self, (x, y, z): (f32, f32, f32)) -> &'a mut GeometryBuilder {
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

    pub fn visible<'a>(&'a mut self) -> &'a mut GeometryBuilder {
        self.visible = true;
        self
    }

    pub fn pickable<'a>(&'a mut self) -> &'a mut GeometryBuilder {
        self.pickable = true;
        self
    }

    pub fn mesh<'a>(&'a mut self, mesh_id: MeshId) -> &'a mut GeometryBuilder {
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
