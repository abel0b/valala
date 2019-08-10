use crate::{
    geometry::{Geometry, Shape},
    mesh::{Normal, PrimitiveType, Vertex},
    resource::{ShaderId, TextureId},
    scene::{Entity, Uid},
    store::{Store, World},
    view::View,
};
use hashbrown::{hash_map::Iter, HashMap};
use nalgebra::base::Matrix4;
use std::convert::TryInto;

pub struct CacheEntry {
    pub transform: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub normals: Option<Vec<Normal>>,
    pub indices: Vec<u32>,
}

#[derive(Default)]
pub struct Cache {
    entries: HashMap<(ShaderId, TextureId, PrimitiveType, bool), CacheEntry>, // handle transform
    views: HashMap<Uid, View>,
}

impl Cache {
    pub fn new() -> Cache {
        Default::default()
    }

    pub fn iter(&self) -> Iter<(ShaderId, TextureId, PrimitiveType, bool), CacheEntry> {
        self.entries.iter()
    }

    pub fn update<W: World>(
        &mut self,
        store: &mut Store<W>,
        uid: Uid,
        entity: &Entity<W>,
        transform: Matrix4<f32>,
    ) {
        if let Some(render) = entity.render {
            let view = render(store, uid);
            for geometry in view.geometries.iter() {
                self.register(store, transform * geometry.transform, &geometry);
            }
            self.views.insert(uid, view);
        }
    }

    pub fn register<W: World>(
        &mut self,
        store: &Store<W>,
        transform: Matrix4<f32>,
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
