use crate::{
    mesh::{Mesh, Normal, Vertex, PrimitiveType},
    resource::{ModelId, ShaderId, TextureId},
};

pub enum Shape {
    Model(ModelId),
    Mesh(Mesh),
}

pub struct Geometry {
    pub shader_id: ShaderId,
    pub texture_id: TextureId,
    pub shape: Shape,
}

impl Geometry {
    pub fn with_model(model_id: ModelId) -> Geometry {
        Geometry {
            shader_id: ShaderId("model"),
            texture_id: TextureId("default"),
            shape: Shape::Model(model_id),
        }
    }
    pub fn with_model_and_texture(model_id: ModelId, texture_id: TextureId) -> Geometry {
        Geometry {
            shader_id: ShaderId("model"),
            texture_id,
            shape: Shape::Model(model_id),
        }
    }
}

pub struct GeometryBuilder {
    shader_id: ShaderId,
    texture_id: TextureId,
    model_id: Option<ModelId>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    normals: Option<Vec<Normal>>,
    primitive: PrimitiveType,
}

impl Default for GeometryBuilder {
    fn default() -> GeometryBuilder {
        GeometryBuilder {
            shader_id: ShaderId("default"),
            texture_id: TextureId("default"),
            model_id: None,
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: None,
            primitive: PrimitiveType::TrianglesList,
        }
    }
}

impl GeometryBuilder {
    pub fn shader(&mut self, shader: ShaderId) -> &mut GeometryBuilder {
        self.shader_id = shader;
        self
    }

    pub fn vertex(
        &mut self,
        position: (f32, f32, f32),
        color: (f32, f32, f32, f32),
        tex_coords: (f32, f32),
    ) -> &mut GeometryBuilder {

        self.vertices.push(Vertex {
            id: 0,
            position,
            color,
            tex_coords,
        });
        self
    }

    pub fn texture(&mut self, texture_id: TextureId) -> &mut GeometryBuilder {
        self.texture_id = texture_id;
        self
    }

    pub fn triangle(&mut self, a: u32, b: u32, c: u32) -> &mut GeometryBuilder {
        self.primitive = PrimitiveType::TrianglesList;
        self.indices.extend_from_slice(&[a, b, c]);
        self
    }
    pub fn line(&mut self, a: u32, b: u32) -> &mut GeometryBuilder {
        self.primitive = PrimitiveType::LinesList;
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

    pub fn model(&mut self, model_id: ModelId) -> &mut GeometryBuilder {
        self.model_id = Some(model_id);
        self
    }

    pub fn build(self) -> Geometry {
        Geometry {
            shader_id: self.shader_id,
            texture_id: self.texture_id,
            shape: match self.model_id {
                Some(model_id) => Shape::Model(model_id),
                None => Shape::Mesh(Mesh {
                    vertices: self.vertices,
                    indices: self.indices,
                    normals: self.normals,
                    primitive: self.primitive,
                }),
            },
        }
    }
}
