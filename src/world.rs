use crate::camera;
use crate::map;
use crate::scene::Scene;
use crate::map::Map;

pub struct World {
    pub camera: camera::Camera,
    pub map: map::Map,
    pub scene: Scene,
}

impl World {
    pub fn new(display: &glium::Display) -> World {
        let (width, height) = (1080.0, 720.0);

        let mut scene = Scene::new();
        let map = Map::new_hexagonal(&mut scene, display, 6);

        World {
            camera: camera::Camera::new((height/width) as f32),
            map,
            scene,
        }
    }
}
