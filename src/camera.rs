use crate::transform::Transform;

use glam::Mat4;
use crate::window::*;

pub struct Camera {
    pub frustum_near: f32,
    pub frustum_far: f32,
    pub fov: f32, // in radians
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            frustum_near: 0.1,
            frustum_far: 100.0,
            fov: std::f32::consts::PI / 4.0,
            aspect_ratio: SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
            transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
            speed: 1.0,
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov,
            self.aspect_ratio,
            self.frustum_near,
            self.frustum_far,
        )
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }
}