use nalgebra::{Matrix4, Point3, Unit, Vector, Vector3};
use std::f32::consts::PI;

pub struct Camera {
    aspect_ratio: f32,
    pub view: Matrix4<f32>,
    pub perspective: Matrix4<f32>,
    pub zoom: f32,
}

impl Camera {
    pub fn isometric(aspect_ratio: f32) -> Camera {
        Camera {
            aspect_ratio,
            view: Self::compute_view(),
            perspective: Self::compute_perspective(aspect_ratio, 1.0),
            zoom: 1.0,
        }
    }

    fn compute_view() -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::from([0.0, 0.0, 0.0]),
            &Point3::from([0.0, 0.0, 1.0]),
            &Vector3::from([0.0, 1.0, 0.0]),
        ) * Matrix4::from_axis_angle(
            &Unit::new_normalize(Vector3::from([1.0, 0.0, 0.0])),
            -(1.0 / 2.0f32.sqrt()).atan(),
        ) * Matrix4::from_axis_angle(
            &Unit::new_normalize(Vector3::from([0.0, 1.0, 0.0])),
            PI / 4.0,
        )
    }

    fn compute_perspective(aspect_ratio: f32, zoom: f32) -> Matrix4<f32> {
        let right = 10.0 * zoom;
        let left = -10.0 * zoom;
        let bottom = -10.0 * zoom;
        let top = 10.0 * zoom;
        let far = 200.0;
        let near = -100.0;
        Matrix4::new_nonuniform_scaling(&Vector::from([aspect_ratio, 1.0, 1.0]))
            * Matrix4::new_orthographic(left, right, bottom, top, near, far)
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.perspective * self.view
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
