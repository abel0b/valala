use glium::Program;
use crate::context::GlBackend;

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(backend: &GlBackend, filename: &str) -> Shader {
        let vertex_shader_file = match std::fs::read_to_string(format!("./res/shaders/{}.vert", filename)) {
            Ok(shader) => shader,
            _ => panic!("could not open vertex shader {}", filename),
        };
        let fragment_shader_file = match std::fs::read_to_string(format!("./res/shaders/{}.frag", filename)) {
            Ok(shader) => shader,
            _ => panic!("could not open fragment shader {}", filename),
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
