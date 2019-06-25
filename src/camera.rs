use std::f32::consts::PI;

pub struct Camera {
    pub view: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub perspective: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        Camera {
            view: Self::get_view(),
            model: Self::get_model(),
            perspective: Self::get_perspective(aspect_ratio),
        }
    }

    fn get_view() -> [[f32; 4]; 4] {
        cgmath::conv::array4x4(
            cgmath::Matrix4::look_at_dir(
                cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 },
                cgmath::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
                cgmath::Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            )
        )
    }

    fn get_perspective(aspect_ratio:f32) -> [[f32; 4]; 4] {
        let right = 10.0;
        let left = -10.0;
        let bottom = -10.0;
        let top = 10.0;
        let far = 100.0;
        let near = -100.0;
        cgmath::conv::array4x4(
            cgmath::Matrix4::from_nonuniform_scale(aspect_ratio, 1.0, 1.0)
            * cgmath::ortho(left, right, bottom, top, near, far)
        )
    }

    fn get_model() -> [[f32; 4]; 4] {
        cgmath::conv::array4x4(
            cgmath::Matrix4::from_angle_x(cgmath::Rad(-(1.0/2.0f32.sqrt()).atan()))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(PI/4.0))
        )
    }

    pub fn scale(&mut self, aspect_ratio:f32) {
        self.perspective = Self::get_perspective(aspect_ratio);
    }

}
