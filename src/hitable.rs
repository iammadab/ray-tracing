use crate::{ray::Ray, vec3::Vec3};

pub(crate) struct HitRecord {
    pub(crate) t: f32,
    point: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub(crate) fn new(t: f32, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
    }
}

pub(crate) trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
