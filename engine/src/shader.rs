use glium::Program;
use crate::context::GlBackend;

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(backend: &GlBackend, path: std::string::String) -> Shader {
        let vertex_shader_file = match std::fs::read_to_string(format!("{}.vert", &path)) {
            Ok(shader) => shader,
            _ => panic!("could not open vertex shader {}", &path),
        };
        let fragment_shader_file = match std::fs::read_to_string(format!("{}.frag", &path)) {
            Ok(shader) => shader,
            _ => panic!("could not open fragment shader {}", &path),
        };
        Shader {
            program: glium::Program::from_source(&backend.display, &vertex_shader_file[..], &fragment_shader_file[..], None).unwrap(),
        }
    }

    pub fn from_source(backend: &GlBackend, vertex_shader: &str, fragment_shader: &str) -> Shader {
        Shader {
            program: glium::Program::from_source(&backend.display, vertex_shader, fragment_shader, None).unwrap(),
        }
    }
}
