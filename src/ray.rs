#![allow(unused)]

use crate::vec3::Vec3;

pub(crate) struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    fn new(a: &'a Vec3, b: &'a Vec3) -> Self {
        Self {
            origin: a,
            direction: b,
        }
    }

    pub(crate) fn origin(&self) -> &Vec3 {
        self.origin
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        self.direction
    }

    pub(crate) fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
