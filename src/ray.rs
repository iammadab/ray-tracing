use crate::vec3::Vec3;

pub(crate) struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub(crate) fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            origin: a,
            direction: b,
        }
    }

    pub(crate) fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub(crate) fn point_at(&self, t: f32) -> Vec3 {
        &self.origin + &self.direction * t
    }
}
