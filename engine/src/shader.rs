use crate::context::GlBackend;
use glium::Program;
use log::info;

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(
        backend: &GlBackend,
        vertex_shader_file: std::string::String,
        fragment_shader_file: std::string::String,
    ) -> Shader {
        info!("Loading vertex shader {}", &vertex_shader_file);
        info!("Loading fragment shader {}", &fragment_shader_file);
        let vertex_shader_src = match std::fs::read_to_string(&vertex_shader_file) {
            Ok(shader) => shader,
            _ => panic!("could not open vertex shader {}", vertex_shader_file),
        };
        let fragment_shader_src = match std::fs::read_to_string(&fragment_shader_file) {
            Ok(shader) => shader,
            _ => panic!("could not open fragment shader {}", fragment_shader_file),
        };
        Shader {
            program: glium::Program::from_source(
                &backend.display,
                &vertex_shader_src[..],
                &fragment_shader_src[..],
                None,
            )
            .unwrap(),
        }
    }

    pub fn from_source(backend: &GlBackend, vertex_shader: &str, fragment_shader: &str) -> Shader {
        Shader {
            program: glium::Program::from_source(
                &backend.display,
                vertex_shader,
                fragment_shader,
                None,
            )
            .unwrap(),
        }
    }
}
