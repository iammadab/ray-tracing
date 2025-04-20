#![allow(unused)]

use crate::vec3::Vec3;

struct Ray<'a> {
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

    fn origin(&self) -> &Vec3 {
        self.origin
    }

    fn direction(&self) -> &Vec3 {
        self.direction
    }

    fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
