#[derive(Copy, Clone)]
pub struct Vertex {
    pub id: u32,
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
    pub coordinates: (f32, f32, f32),
}

#[derive(Copy, Clone)]
pub struct SimpleVertex {
    pub position: (f32, f32, f32),
}

glium::implement_vertex!(Vertex, id, position, tex_coords, coordinates);
glium::implement_vertex!(SimpleVertex, position);


use std::path::Path;
use tobj;
use crate::mesh;
use glium::{IndexBuffer, VertexBuffer};

pub fn load(display: &glium::Display, path: &str) -> (glium::VertexBuffer<mesh::SimpleVertex>, glium::IndexBuffer<u32>) {
    let obj = tobj::load_obj(&Path::new(path));

    assert!(obj.is_ok());

    let mut vertices: Vec<mesh::SimpleVertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let (models, _materials) = obj.unwrap();

    for (i, m) in models.iter().enumerate() {
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
            })
    	}
    }

    (glium::VertexBuffer::new(display, &vertices).unwrap(), glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap())

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
}
