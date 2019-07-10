use image;
use crate::context::GlBackend;

pub struct Texture {
    pub texture: glium::texture::Texture2d,
}

impl Texture {
    pub fn new(backend: &GlBackend, filename: &str) -> Texture {
        let resource = match image::open(format!("./res/textures/{}", filename)) {
            Ok(resource) => resource.to_rgba(),
            _ => panic!("could not open texture {}", filename),
        };
        let dimensions = resource.dimensions();
        let resource = glium::texture::RawImage2d::from_raw_rgba_reversed(&resource.into_raw(), dimensions);
        Texture {
            texture: glium::texture::Texture2d::new(&backend.display, resource).unwrap(),
        }
    }
}
