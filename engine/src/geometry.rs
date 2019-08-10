use crate::{
    mesh::{Mesh, Normal, PrimitiveType, Vertex},
    resource::{ModelId, ShaderId, TextureId},
};
use nalgebra::{Matrix4, Unit, Vector3};

pub enum Shape {
    Model(ModelId),
    Mesh(Mesh),
}

pub struct Geometry {
    pub shader_id: ShaderId,
    pub texture_id: TextureId,
    pub shape: Shape,
    pub transform: Matrix4<f32>,
}

impl Geometry {
    pub fn with_model(model_id: ModelId) -> Geometry {
        Geometry {
            shader_id: ShaderId("model"),
            texture_id: TextureId("default"),
            shape: Shape::Model(model_id),
            transform: Matrix4::identity(),
        }
    }
    pub fn with_model_and_texture(model_id: ModelId, texture_id: TextureId) -> Geometry {
        Geometry {
            shader_id: ShaderId("model"),
            texture_id,
            shape: Shape::Model(model_id),
            transform: Matrix4::identity(),
        }
    }
}

pub struct GeometryBuilder {
    id: u32,
    shader_id: ShaderId,
    texture_id: TextureId,
    model_id: Option<ModelId>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    normals: Option<Vec<Normal>>,
    primitive: PrimitiveType,
    transform: Matrix4<f32>,
}

impl Default for GeometryBuilder {
    fn default() -> GeometryBuilder {
        GeometryBuilder {
            id: 0,
            shader_id: ShaderId("default"),
            texture_id: TextureId("default"),
            model_id: None,
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: None,
            primitive: PrimitiveType::TrianglesList,
            transform: Matrix4::identity(),
        }
    }
}

impl GeometryBuilder {
    pub fn new() -> GeometryBuilder {
        Default::default()
    }

    pub fn with_id(id: u32) -> GeometryBuilder {
        GeometryBuilder {
            id,
            ..Default::default()
        }
    }

    pub fn with_model_and_texture(model_id: ModelId, texture_id: TextureId) -> GeometryBuilder {
        GeometryBuilder {
            shader_id: ShaderId("model"),
            texture_id,
            model_id: Some(model_id),
            ..Default::default()
        }
    }

    pub fn translate(&mut self, vector: Vector3<f32>) -> &mut GeometryBuilder {
        self.transform = self.transform * Matrix4::new_translation(&vector);
        self
    }

    pub fn scale(&mut self, value: f32) -> &mut GeometryBuilder {
        self.transform = self.transform * Matrix4::new_scaling(value);
        self
    }

    pub fn rotate_x(&mut self, angle: f32) -> &mut GeometryBuilder {
        self.transform = self.transform
            * Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::from([1.0, 0.0, 0.0])), angle);
        self
    }

    pub fn rotate_y(&mut self, angle: f32) -> &mut GeometryBuilder {
        self.transform = self.transform
            * Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::from([0.0, 1.0, 0.0])), angle);
        self
    }

    pub fn rotate_z(&mut self, angle: f32) -> &mut GeometryBuilder {
        self.transform = self.transform
            * Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::from([0.0, 0.0, 1.0])), angle);
        self
    }

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
            id: self.id,
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

    pub fn build(&self) -> Geometry {
        Geometry {
            shader_id: self.shader_id,
            texture_id: self.texture_id,
            shape: match self.model_id {
                Some(model_id) => Shape::Model(model_id),
                None => Shape::Mesh(Mesh {
                    vertices: self.vertices.clone(),
                    indices: self.indices.clone(),
                    normals: self.normals.clone(),
                    primitive: self.primitive,
                }),
            },
            transform: self.transform,
        }
    }
}
