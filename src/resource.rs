use std::collections::HashMap;

use image;

const RESOURCE_PACK_FOLDER: &str = "./res";

// pub enum ImageId {
//     TERRAIN,
// }

pub struct ImageResource {
    pub texture: glium::texture::Texture2d,
}

pub struct ResourcePack {
    pub images: HashMap<&'static str, ImageResource>,
}

impl ImageResource {
    // TODO : handle errors
    pub fn new(display: &glium::Display, filename: &str) -> ImageResource {
        let resource = match image::open(format!("{}/{}", RESOURCE_PACK_FOLDER, filename)) {
            Ok(resource) => resource.to_rgba(),
            _ => panic!("can not load image"),
        };
        let dimensions = resource.dimensions();
        let resource = glium::texture::RawImage2d::from_raw_rgba_reversed(&resource.into_raw(), dimensions);
        ImageResource {
            texture: glium::texture::Texture2d::new(display, resource).unwrap(),
        }
    }
}

impl ResourcePack {
    pub fn new(display: &glium::Display) -> ResourcePack {
        let images = HashMap::new();
        let mut resources = ResourcePack {
            images,
        };
        resources.load_image(display, "terrain.png");
        resources
    }

    fn load_image(&mut self, display: &glium::Display, filename: &'static str) {
        self.images.insert(filename, ImageResource::new(display, filename));
    }
}
