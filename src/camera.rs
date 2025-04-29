use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal_len: Vec3,
    vertical_len: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0., 0., 0.),
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal_len: Vec3::new(4., 0., 0.),
            vertical_len: Vec3::new(0., 2., 0.),
        }
    }
}

impl Camera {
    // u = horizontal_percent
    // v = vertical_percent
    // they are used to compute the exact point co-ordinate
    // for a particular pixel
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // point on the progrction plane
        let point = &self.lower_left_corner + &self.horizontal_len * u + &self.vertical_len * v;
        Ray::new(self.origin.clone(), point)
    }
}
