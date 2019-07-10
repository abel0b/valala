use std::collections::HashMap;
use crate::{
    texture::Texture,
    shader::Shader,
    mesh::Mesh,
};

#[derive(Eq, PartialEq, Hash)]
pub struct TextureId(pub &'static str);

#[derive(Eq, PartialEq, Hash)]
pub struct MeshId(pub &'static str);

#[derive(Eq, PartialEq, Hash)]
pub struct ShaderId(pub &'static str);

pub struct ResourcePack {
    pub textures: HashMap<TextureId, Texture>,
    pub shaders: HashMap<ShaderId, Shader>,
    pub meshes: HashMap<MeshId, Mesh>,
}

impl ResourcePack {
    pub fn new() -> ResourcePack {
        let textures = HashMap::new();
        let shaders = HashMap::new();
        let meshes = HashMap::new();

        ResourcePack {
            textures,
            shaders,
            meshes,
        }
    }

    pub fn register_texture(&mut self, id: TextureId, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn register_shader(&mut self, id: ShaderId, shader: Shader) {
        self.shaders.insert(id, shader);
    }

    pub fn register_mesh(&mut self, id: MeshId, mesh: Mesh) {
        self.meshes.insert(id, mesh);
    }

    pub fn get_mesh(&self, id: &MeshId) -> &Mesh {
        &self.meshes.get(id).unwrap()
    }

    pub fn get_texture(&self, id: &TextureId) -> &Texture {
        &self.textures.get(id).unwrap()
    }

    pub fn get_shader(&self, id: &ShaderId) -> &Shader {
        &self.shaders.get(id).unwrap()
    }
}
