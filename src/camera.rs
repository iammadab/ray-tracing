use crate::ray::Ray;
use crate::vec3::Vec3;

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal_len: Vec3,
    vertical_len: Vec3,
}

impl Camera {
    // u = horizontal_percent
    // v = vertical_percent
    // they are used to compute the exact point co-ordinate
    // for a particular pixel
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        // point on the progrction plane
        let point = &self.lower_left_corner + &self.horizontal_len * u + &self.vertical_len * v;
        Ray::new(&self.origin, point)
    }
}
