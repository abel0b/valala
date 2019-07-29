use crate::context::GlBackend;
use image;
use log::info;

pub struct Texture {
    pub texture: glium::texture::Texture2d,
}

impl Texture {
    pub fn new(backend: &GlBackend, filename: std::string::String) -> Texture {
        let resource = match image::open(&filename) {
            Ok(resource) => resource.to_rgba(),
            _ => panic!("could not open texture {}", &filename),
        };
        let dimensions = resource.dimensions();
        let resource =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&resource.into_raw(), dimensions);
        info!("Loaded texture '{}'", &filename);
        Texture {
            texture: glium::texture::Texture2d::new(&backend.display, resource).unwrap(),
        }
    }

    pub fn from_raw(backend: &GlBackend, bytes: &[u8]) -> Texture {
        let resource = match image::load_from_memory(bytes) {
            Ok(resource) => resource.to_rgba(),
            _ => panic!("could not load texture"),
        };
        let dimensions = resource.dimensions();
        let resource =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&resource.into_raw(), dimensions);
        Texture {
            texture: glium::texture::Texture2d::new(&backend.display, resource).unwrap(),
        }
    }
}
