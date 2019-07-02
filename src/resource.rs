// TODO : handle errors + multithreading

use std::collections::HashMap;
use std::path::Path;
use image;
use tobj;
use crate::mesh;

#[derive(Eq, PartialEq, Hash)]
pub enum TextureId {
    Terrain,
}

#[derive(Eq, PartialEq, Hash)]
pub enum ShaderId {
    Character,
    Grid,
    Path,
    Picking,
    Terrain,
}

#[derive(Eq, PartialEq, Hash)]
pub enum MeshId {
    Character,
}

pub struct Texture {
    pub texture: glium::texture::Texture2d,
}

pub struct Shader {
    pub program: glium::Program,
}

pub struct Mesh {
    pub vertices: glium::VertexBuffer<mesh::SimpleVertex>,
    pub indices: glium::IndexBuffer<u32>,
}

pub struct ResourcePack {
    pub textures: HashMap<TextureId, Texture>,
    pub shaders: HashMap<ShaderId, Shader>,
    pub meshes: HashMap<MeshId, Mesh>,
}

impl Texture {
    pub fn new(display: &glium::Display, filename: &str) -> Texture {
        let resource = match image::open(format!("./res/textures/{}", filename)) {
            Ok(resource) => resource.to_rgba(),
            _ => panic!("could not open texture {}", filename),
        };
        let dimensions = resource.dimensions();
        let resource = glium::texture::RawImage2d::from_raw_rgba_reversed(&resource.into_raw(), dimensions);
        Texture {
            texture: glium::texture::Texture2d::new(display, resource).unwrap(),
        }
    }
}

impl Shader {
    pub fn new(display: &glium::Display, filename: &str) -> Shader {
        let vertex_shader_file = match std::fs::read_to_string(format!("./res/shaders/{}.vert", filename)) {
            Ok(shader) => shader,
            _ => panic!("could not open vertex shader {}", filename),
        };
        let fragment_shader_file = match std::fs::read_to_string(format!("./res/shaders/{}.frag", filename)) {
            Ok(shader) => shader,
            _ => panic!("could not open fragment shader {}", filename),
        };
        Shader {
            program: glium::Program::from_source(display, &vertex_shader_file[..], &fragment_shader_file[..], None).unwrap(),
        }
    }
}

impl Mesh {
    pub fn new(display: &glium::Display, path: &str) -> Mesh {
        let obj = tobj::load_obj(&Path::new(&format!("./res/meshes/{}.obj", path)));

        assert!(obj.is_ok());

        let mut vertices: Vec<mesh::SimpleVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let (models, _materials) = obj.unwrap();

        for (_i, m) in models.iter().enumerate() {
        	let mesh = &m.mesh;
        	for f in 0..mesh.indices.len() / 3 {
        		indices.push(mesh.indices[3 * f]);
        		indices.push(mesh.indices[3 * f + 1]);
        		indices.push(mesh.indices[3 * f + 2]);
        	}

        	assert!(mesh.positions.len() % 3 == 0);
        	for v in 0..mesh.positions.len() / 3 {
                vertices.push(mesh::SimpleVertex {
                    position: (mesh.positions[3 * v], mesh.positions[3 * v + 1], mesh.positions[3 * v + 2])
                });
        	}
        }
        // for (i, m) in materials.iter().enumerate() {
        // 	println!("material[{}].name = \'{}\'", i, m.name);
        // 	println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1],
        // 		m.ambient[2]);
        // 	println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1],
        // 		m.diffuse[2]);
        // 	println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1],
        // 		m.specular[2]);
        // 	println!("    material.Ns = {}", m.shininess);
        // 	println!("    material.d = {}", m.dissolve);
        // 	println!("    material.map_Ka = {}", m.ambient_texture);
        // 	println!("    material.map_Kd = {}", m.diffuse_texture);
        // 	println!("    material.map_Ks = {}", m.specular_texture);
        // 	println!("    material.map_Ns = {}", m.normal_texture);
        // 	println!("    material.map_d = {}", m.dissolve_texture);
        // 	for (k, v) in &m.unknown_param {
        // 		println!("    material.{} = {}", k, v);
        // 	}
        // }

        Mesh {
            vertices: glium::VertexBuffer::new(display, &vertices).unwrap(),
            indices: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
        }
    }
}

impl ResourcePack {
    pub fn new(display: &glium::Display) -> ResourcePack {
        let textures = HashMap::new();
        let shaders = HashMap::new();
        let meshes = HashMap::new();
        let mut resources = ResourcePack {
            textures,
            shaders,
            meshes,
        };

        resources.load_texture(display, TextureId::Terrain, "terrain.png");

        resources.load_shader(display, ShaderId::Grid, "grid");
        resources.load_shader(display, ShaderId::Path, "path");
        resources.load_shader(display, ShaderId::Picking, "picking");
        resources.load_shader(display, ShaderId::Character, "character");
        resources.load_shader(display, ShaderId::Terrain, "terrain");

        resources.load_mesh(display, MeshId::Character, "character");

        resources
    }

    fn load_texture(&mut self, display: &glium::Display, id: TextureId, filename: &'static str) {
        self.textures.insert(id, Texture::new(display, filename));
    }

    fn load_shader(&mut self, display: &glium::Display, id: ShaderId, filename: &'static str) {
        self.shaders.insert(id, Shader::new(display, filename));
    }

    fn load_mesh(&mut self, display: &glium::Display, id: MeshId, filename: &'static str) {
        self.meshes.insert(id, Mesh::new(display, filename));
    }

    pub fn get_mesh(&self, id: &MeshId) -> &Mesh {
        &self.meshes.get(id).unwrap()
    }

    pub fn get_texture(&self, id: &TextureId) -> &glium::Texture2d {
        &self.textures.get(id).unwrap().texture
    }

    pub fn get_shader(&self, id: &ShaderId) -> &glium::Program {
        &self.shaders.get(id).unwrap().program
    }
}
