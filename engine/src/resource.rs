use crate::{model::Model, shader::Shader, texture::Texture};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct TextureId(pub &'static str);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct ModelId(pub &'static str);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct ShaderId(pub &'static str);

#[derive(Default)]
pub struct ResourcePack {
    pub textures: HashMap<TextureId, Texture>,
    pub shaders: HashMap<ShaderId, Shader>,
    pub models: HashMap<ModelId, Model>,
}

impl ResourcePack {
    pub fn register_texture(&mut self, id: TextureId, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn register_shader(&mut self, id: ShaderId, shader: Shader) {
        self.shaders.insert(id, shader);
    }

    pub fn register_model(&mut self, id: ModelId, model: Model) {
        self.models.insert(id, model);
    }

    pub fn get_model(&self, id: &ModelId) -> &Model {
        &self.models.get(id).unwrap()
    }

    pub fn get_texture(&self, id: &TextureId) -> &Texture {
        &self.textures.get(id).unwrap()
    }

    pub fn get_shader(&self, id: &ShaderId) -> &Shader {
        &self.shaders.get(id).unwrap()
    }
}
