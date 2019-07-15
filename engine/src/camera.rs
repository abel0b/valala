use std::f32::consts::PI;

pub struct Camera {
    aspect_ratio: f32,
    pub view: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub perspective: [[f32; 4]; 4],
    pub zoom: f32,
}

impl Camera {
    pub fn isometric(aspect_ratio: f32) -> Camera {
        Camera {
            aspect_ratio,
            view: Self::compute_view(),
            model: Self::compute_model(),
            perspective: Self::compute_perspective(aspect_ratio, 1.0),
            zoom: 1.0,
        }
    }

    fn compute_view() -> [[f32; 4]; 4] {
        cgmath::conv::array4x4(cgmath::Matrix4::look_at_dir(
            cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            cgmath::Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ))
    }

    fn compute_perspective(aspect_ratio: f32, zoom: f32) -> [[f32; 4]; 4] {
        let right = 10.0 * zoom;
        let left = -10.0 * zoom;
        let bottom = -10.0 * zoom;
        let top = 10.0 * zoom;
        let far = 200.0;
        let near = -100.0;
        cgmath::conv::array4x4(
            cgmath::Matrix4::from_nonuniform_scale(aspect_ratio, 1.0, 1.0)
                * cgmath::ortho(left, right, bottom, top, near, far),
        )
    }

    fn compute_model() -> [[f32; 4]; 4] {
        cgmath::conv::array4x4(
            cgmath::Matrix4::from_angle_x(cgmath::Rad(-(1.0 / 2.0f32.sqrt()).atan()))
                * cgmath::Matrix4::from_angle_y(cgmath::Rad(PI / 4.0)),
        )
    }

    pub fn scale(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.perspective = Self::compute_perspective(self.aspect_ratio, self.zoom);
    }

    pub fn zoom(&mut self, value: f32) {
        self.zoom = f32::min(2.0, f32::max(0.1, self.zoom - value * 0.1));
        self.perspective = Self::compute_perspective(self.aspect_ratio, self.zoom);
    }
}
